use std::sync::Arc;

use axum::{extract::{Path, State}, http::Method, routing::get, Json, Router};
use rand::Rng;
use tower_http::cors::{Any, CorsLayer};

use eowordle_lib::{list_enemies, Enemy, EnemyDiff};

pub fn get_random_enemy() -> Enemy {
    let enemies = list_enemies();
    let mut rng = rand::rng();
    let enemy = enemies.get(rng.random_range(0..enemies.len())).unwrap().clone();

    println!("The daily enemy is:\n{enemy:?}");
    enemy
}

pub struct ApiState {
    pub enemies: Box<[Enemy]>,
    pub daily_enemy: Enemy
}

pub fn make_router() -> Router {
    Router::new()
        .route("/daily/diff/{id}", get(daily_diff))
        .layer(CorsLayer::new().allow_methods([Method::GET]).allow_origin(Any))
        .with_state(Arc::new(ApiState { 
            daily_enemy: get_random_enemy(), enemies: list_enemies() 
        }))
}

async fn daily_diff(
    Path(id): Path<usize>,
    State(state): State<Arc<ApiState>>) -> Json<Option<EnemyDiff>> {

    if let Some(enemy) = state.enemies.iter().find(|e| e.id == id as u16) {
        let diff = enemy.diff(&state.daily_enemy);
        println!("{diff:?}");

        Json(Some(diff))
    } else {
    Json(None)
    }
}
