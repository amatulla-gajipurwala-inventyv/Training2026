use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vehicle {
    pub id: Uuid,
    pub brand: String,
    pub model: String,
    pub year: u16,
}

#[derive(Debug, Deserialize)]
pub struct CreateVehicle {
    pub brand: String,
    pub model: String,
    pub year: u16,
}
