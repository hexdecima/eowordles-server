use std::sync::Arc;

use api::DailyEnemy;
use db::Database;
use tokio::sync::RwLock;
use scheduler::Scheduler;
use shuttle_runtime::CustomError;
use sqlx::PgPool;

mod api;
mod db;
mod scheduler;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(CustomError::new)?;

    let db = Arc::new(RwLock::new(Database::new(pool)));
    let daily = Arc::new(RwLock::new(DailyEnemy::get_dummy()));
    let scheduler = Arc::new(RwLock::new(Scheduler::new(db.clone(), daily.clone())));

    let runner = scheduler.clone();
    tokio::spawn(async move {
        let mut scheduler = runner.write().await;
        scheduler.execute().await;
    });

    Ok(api::make_router(daily.clone()).into())
}
