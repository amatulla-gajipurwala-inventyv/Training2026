use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

use crate::models::Vehicle;

pub type Db = Arc<RwLock<HashMap<Uuid, Vehicle>>>;

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
}
