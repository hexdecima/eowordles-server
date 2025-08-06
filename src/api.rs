use std::sync::Arc;

use axum::{extract::{Path, State}, http::Method, routing::get, Json, Router};
use chrono::{NaiveDate, Utc};
use rand::Rng;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

use eowordle_lib::{list_enemies, Enemy, EnemyDiff};

pub struct DailyEnemy {
    pub enemy: Option<Enemy>,
    pub day: NaiveDate,
}

impl DailyEnemy {
    pub fn get_dummy() -> Self {
        Self { enemy: None, day: Utc::now().date_naive() }
    }
}

pub struct ApiState {
    pub enemies: Box<[Enemy]>,
    pub daily_enemy: Arc<RwLock<DailyEnemy>>,
    pub yesterdays: Arc<RwLock<Option<Enemy>>>
}

pub fn make_router(daily_enemy: Arc<RwLock<DailyEnemy>>, yesterdays: Arc<RwLock<Option<Enemy>>>) -> Router {
    Router::new()
        .route("/daily/diff/{id}", get(daily_diff))
        .route("/yesterday", get(get_yesterdays))
        .layer(CorsLayer::new().allow_methods([Method::GET]).allow_origin(Any))
        .with_state(Arc::new(ApiState { 
            enemies: list_enemies(), daily_enemy, yesterdays
        }))
}

async fn get_yesterdays(State(state): State<Arc<ApiState>>) -> Json<Option<Enemy>> {
    Json((*state.yesterdays.read().await).clone())
}

async fn daily_diff(
    Path(id): Path<usize>,
    State(state): State<Arc<ApiState>>) -> Json<Option<EnemyDiff>> {

    if let Some(enemy) = state.enemies.iter().find(|e| e.id == id as u16) {
        let daily = state.daily_enemy.read().await;
        let diff = enemy.diff(&daily.enemy.clone().unwrap());

        Json(Some(diff))
    } else {
    Json(None)
    }
}

pub fn get_random_enemy() -> Enemy {
    let enemies = list_enemies();
    let mut rng = rand::rng();
    let enemy = enemies
        .get(rng.random_range(0..enemies.len()))
        .unwrap()
        .clone();
    enemy
}
