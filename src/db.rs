use std::sync::Arc;

use chrono::NaiveDate;
use eowordle_lib::{list_enemies, Enemy};
use sqlx::{prelude::FromRow, PgPool};
use tokio::sync::RwLock;

pub struct Database {
    pub pool: Arc<RwLock<PgPool>>,
}

impl Database {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool: Arc::new(RwLock::new(pool)),
        }
    }
    /// Returns the enemy for a given `date`, if any.
    pub async fn get_for_date(&self, date: NaiveDate) -> Option<Enemy> {
        let date = date.to_string();

        let pool = self.pool.read().await;
        match sqlx::query_as::<_, HistoryEntry>("SELECT * FROM history WHERE date = $1").bind(date).fetch_one(&*pool).await {
            Ok(entry) => {
                let id = entry.id.parse::<u16>().unwrap();
                let enemies = list_enemies();
                enemies.iter().find(|enemy| enemy.id == id).cloned()
            },
            Err(e) => {
                eprintln!("can't get enemy for date: {e:?}");
                None
            },
        }
    }
    /// Adds an enemy `id` to given `date`, returns whether or not it was added.
    pub async fn add_for_date(&self, id: u16, date: NaiveDate) -> bool {
        let pool = self.pool.read().await;

        match sqlx::query("INSERT INTO history VALUES ($1, $2)").bind(date.to_string()).bind(id.to_string()).execute(&*pool).await {
            Ok(_) => true,
            Err(_) => false // TODO: Handle errors, for now we assume they're duplicate errors
                            // (23505).
        }
    }
}


#[derive(FromRow)]
#[allow(dead_code)]
struct HistoryEntry {
    date: String,
    id: String,
}
