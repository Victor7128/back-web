mod models;

use actix_cors::Cors;
use actix_web::{get, post, web, HttpResponse, Responder};
use models::{NewItem, Item};
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::PgPool;
use uuid::Uuid;

#[get("/get_items")]
async fn get_items(db: web::Data<PgPool>) -> impl Responder {
    let items = sqlx::query_as::<_, Item>("SELECT * FROM item")
        .fetch_all(db.get_ref())
        .await;

    match items {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/get_item/{id}")]
async fn get_item_by_id(db: web::Data<PgPool>, id: web::Path<Uuid>) -> impl Responder {
    let item = sqlx::query_as::<_, Item>("SELECT * FROM item WHERE id = $1")
        .bind(*id)
        .fetch_optional(db.get_ref())
        .await;

    match item {
        Ok(Some(item)) => HttpResponse::Ok().json(item),
        Ok(None) => HttpResponse::NotFound().body("Item no encontrado"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/create_item")]
async fn create_item(
    db: web::Data<PgPool>,
    new_item: web::Json<NewItem>,
) -> impl Responder {
    let item = sqlx::query_as::<_, Item>(
        "INSERT INTO item (id, nombre, descripcion, precio, cantidad) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(&new_item.nombre)
    .bind(&new_item.descripcion)
    .bind(new_item.precio)
    .bind(new_item.cantidad)
    .fetch_one(db.get_ref())
    .await;

    match item {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/update_item/{id}")]
async fn update_item_by_id(
    db: web::Data<PgPool>,
    id: web::Path<Uuid>,
    updated: web::Json<NewItem>,
) -> impl Responder {
    let item = sqlx::query_as::<_, Item>(
        "UPDATE item SET nombre=$1, descripcion=$2, precio=$3, cantidad=$4 WHERE id=$5 RETURNING *"
    )
    .bind(&updated.nombre)
    .bind(&updated.descripcion)
    .bind(updated.precio)
    .bind(updated.cantidad)
    .bind(*id)
    .fetch_optional(db.get_ref())
    .await;

    match item {
        Ok(Some(item)) => HttpResponse::Ok().json(item),
        Ok(None) => HttpResponse::NotFound().body("Item no encontrado"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/delete_item/{id}")]
async fn delete_item_by_id(
    db: web::Data<PgPool>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let result = sqlx::query("DELETE FROM item WHERE id = $1")
        .bind(*id)
        .execute(db.get_ref())
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => HttpResponse::NoContent().finish(),
        Ok(_) => HttpResponse::NotFound().body("Item no encontrado"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] connection_string: String,
) -> ShuttleActixWeb<impl FnOnce(&mut actix_web::web::ServiceConfig) + Send + Clone + 'static> {
    let connection_string = if connection_string.contains('?') {
        format!("{}&sslmode=require", connection_string)
    } else {
        format!("{}?sslmode=require", connection_string)
    };

    let pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to create PgPool");
    sqlx::migrate!().run(&pool).await.expect("Failed migrations");

    let db_data = web::Data::new(pool);

    let config = move |cfg: &mut web::ServiceConfig| {
        cfg.app_data(db_data.clone())
            .service(
                actix_web::web::scope("")
                    .wrap(
                        Cors::default()
                            .allow_any_origin()
                            .allow_any_method()
                            .allow_any_header()
                    )
                    .service(get_items)
                    .service(get_item_by_id)
                    .service(create_item)
                    .service(update_item_by_id)
                    .service(delete_item_by_id)
            );
    };

    Ok(config.into())
}