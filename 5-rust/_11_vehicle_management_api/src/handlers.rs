use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{
    models::{CreateVehicle, Vehicle},
    state::AppState,
};

/// CREATE vehicle
pub async fn create_vehicle(
    State(state): State<AppState>,
    Json(payload): Json<CreateVehicle>,
) -> Json<Vehicle> {
    let mut db = state.db.write().unwrap();

    let vehicle = Vehicle {
        id: Uuid::new_v4(),
        brand: payload.brand,
        model: payload.model,
        year: payload.year,
    };

    db.insert(vehicle.id, vehicle.clone());
    Json(vehicle)
}

/// READ all vehicles
pub async fn get_vehicles(
    State(state): State<AppState>,
) -> Json<Vec<Vehicle>> {
    let db = state.db.read().unwrap();
    Json(db.values().cloned().collect())
}

/// READ one vehicle
pub async fn get_vehicle(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Vehicle>, StatusCode> {
    let db = state.db.read().unwrap();

    db.get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

/// UPDATE vehicle
pub async fn update_vehicle(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<CreateVehicle>,
) -> Result<Json<Vehicle>, StatusCode> {
    let mut db = state.db.write().unwrap();

    match db.get_mut(&id) {
        Some(vehicle) => {
            vehicle.brand = payload.brand;
            vehicle.model = payload.model;
            vehicle.year = payload.year;
            Ok(Json(vehicle.clone()))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// DELETE vehicle
pub async fn delete_vehicle(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> StatusCode {
    let mut db = state.db.write().unwrap();

    if db.remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
