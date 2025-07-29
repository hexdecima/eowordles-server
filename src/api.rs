use std::sync::Arc;

use axum::{extract::{Path, State}, routing::get, Json, Router};

use crate::enemy::{get_all_enemies, get_random_enemy, Enemy, EnemyDiff};

pub struct ApiState {
    pub enemies: Vec<Enemy>, // keep them in memory to avoid constant reads.
    pub daily_enemy: Enemy
}

pub fn make_router() -> Router {
    Router::new()
        .route("/daily/diff/{id}", get(daily_diff))
        .with_state(Arc::new(ApiState { 
            daily_enemy: get_random_enemy(), enemies: get_all_enemies() 
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
