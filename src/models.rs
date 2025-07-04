use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Item {
    pub id: Uuid,
    pub nombre: String,
    pub descripcion: String,
    pub precio: Decimal,
    pub cantidad: i32,
}

#[derive(Deserialize, Debug)]
pub struct NewItem {
    pub nombre: String,
    pub descripcion: String,
    pub precio: Decimal,
    pub cantidad: i32,
}