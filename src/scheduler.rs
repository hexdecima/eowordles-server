use chrono::{Days, Utc};
use eowordle_lib::Enemy;
use std::{sync::Arc, time::Duration};
use tokio::sync::RwLock;

use crate::{api::{get_random_enemy, DailyEnemy}, db::Database};

pub struct Scheduler {
    pub db: Arc<RwLock<Database>>,
    pub daily_enemy: Arc<RwLock<DailyEnemy>>,
}

// TODO: Track time accurately.
impl Scheduler {
    pub fn new(db: Arc<RwLock<Database>>, daily_enemy: Arc<RwLock<DailyEnemy>>) -> Self {
        Self {
            db, daily_enemy
        }
    }
    /// Runs this scheduler in an infinite loop. Should be done in a separate thread.
    pub async fn execute(&mut self) -> ! {
        self.populate_month().await;
        self.init().await;

        loop {
            self.change_daily().await;
            self.populate_month().await;
            tokio::time::sleep(Duration::from_secs(60 * 60)).await;
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
    }
    pub async fn load_for_today(&self) -> Option<Enemy> {
        let today = Utc::now().date_naive();
        let db = self.db.read().await;
        db.get_for_date(today).await
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
        } else { println!("Still the same day, nothing to do."); }
    }
    /// Populates the database history for the next 30 days, including today.
    /// Does nothing for days that are already populated.
    async fn populate_month(&mut self) {
        let mut today = Utc::now().date_naive();
        let end = today + Days::new(30);

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
