use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Item {
    pub id: Uuid,
    pub nombre: String,
    pub descripcion: String,
    pub precio: f32,
    pub cantidad: i32,
}

#[derive(Deserialize)]
pub struct NewItem {
    pub nombre: String,
    pub descripcion: String,
    pub precio: f32,
    pub cantidad: i32,
}