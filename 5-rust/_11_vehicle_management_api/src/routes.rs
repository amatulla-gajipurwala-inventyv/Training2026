use axum::{
    routing::{get, post, put, delete},
    Router,
};

use crate::{handlers, state::AppState};

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route(
            "/vehicles",
            post(handlers::create_vehicle)
                .get(handlers::get_vehicles),
        )
        .route(
            "/vehicles/:id",
            get(handlers::get_vehicle)
                .put(handlers::update_vehicle)
                .delete(handlers::delete_vehicle),
        )
        .with_state(state)
}
