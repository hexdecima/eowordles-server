use chrono::{Days, Utc};
use eowordle_lib::prelude::Enemy;
use std::{sync::Arc, time::Duration};
use tokio::sync::RwLock;

use crate::{api::{get_random_enemy, DailyEnemy}, db::Database};

pub struct Scheduler {
    pub db: Arc<RwLock<Database>>,
    pub daily_enemy: Arc<RwLock<DailyEnemy>>,
    pub yesterdays: Arc<RwLock<Option<Enemy>>>
}

// TODO: Track time accurately.
impl Scheduler {
    pub fn new(db: Arc<RwLock<Database>>, daily_enemy: Arc<RwLock<DailyEnemy>>, yesterdays: Arc<RwLock<Option<Enemy>>>) -> Self {
        Self {
            db, daily_enemy, yesterdays
        }
    }
    /// Runs this scheduler in an infinite loop. Should be done in a separate thread.
    pub async fn execute(&mut self) -> ! {
        self.populate_week().await;
        self.init().await;

        loop {
            tokio::time::sleep(Duration::from_secs(60 * 60)).await;
            self.change_daily().await;
            self.populate_week().await;
        }
    }
    pub async fn init(&self) {
        let Some(enemy) = self.load_for_today().await else {
            eprintln!("No enemy scheduled for today, exiting.");
            std::process::exit(1)
        };

        let today = Utc::now().date_naive();
        let mut daily = self.daily_enemy.write().await;
        *daily = DailyEnemy {
            enemy: Some(enemy.clone()),
            day: today.clone()
        };
        println!("The enemy for today ({today}) is {}!", enemy.name);

        if let Some(enemy) = self.load_for_yesterday().await {
            let mut yesterdays = self.yesterdays.write().await;
            *yesterdays = Some(enemy);
        }
    }
    pub async fn load_for_today(&self) -> Option<Enemy> {
        let today = Utc::now().date_naive();
        let db = self.db.read().await;
        db.get_for_date(today).await
    }
    pub async fn load_for_yesterday(&self) -> Option<Enemy> {
        let yesterday = Utc::now().date_naive() - Days::new(1);
        let db = self.db.read().await;
        db.get_for_date(yesterday).await
    }
    /// Changes the daily enemy, if today is a new day.
    /// Does nothing if not.
    async fn change_daily(&mut self) {
        let today = Utc::now().date_naive();
        let mut current_daily = self.daily_enemy.write().await;

        if current_daily.day != today {
            let db = self.db.read().await;
            match db.get_for_date(today).await {
                Some(enemy) => {
                    *current_daily = DailyEnemy { 
                        enemy: Some(enemy.clone()),
                        day: today.clone()
                    };
                    println!("The enemy for today ({today}) is {}!", enemy.name);
                },
                None => {
                    eprintln!("Nothing defined for day {today:?}")
                },
            }
            if let Some(enemy) = self.load_for_yesterday().await {
                let mut yesterdays = self.yesterdays.write().await;
                *yesterdays = Some(enemy);
            }
    } else { println!("Still the same day, nothing to do."); }
    }
    /// Populates the database history for the next week, including today.
    /// Does nothing for days that are already populated.
    async fn populate_week(&mut self) {
        let mut today = Utc::now().date_naive();
        let end = today + Days::new(7);

        let db = self.db.read().await;
        while today != end {
            let enemy = get_random_enemy();
            let added = db.add_for_date(enemy.id, today.clone()).await;
            if added { 
                println!("Added '{}' for {today}.", enemy.name);
            } else {
                println!("Could not add '{}' for {today}. Assumed duplicate.", enemy.name);
                println!("Halting population.");
                break
            };

            today = today + Days::new(1);
        }
        drop(db);

        println!("Populated month until {today}.");
    }
}
