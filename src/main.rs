pub mod service;
pub mod model;

#[macro_use] extern crate rocket;

use rocket::State;
use rocket::serde::json::Json;
use sea_orm::DatabaseConnection;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/projects")]
async fn projects(db: &State<DatabaseConnection>) -> Json<Vec<model::projects::ModelEx>> {
    Json(service::projects::get_all(db).await)
}

#[get("/projects/<id>")]
async fn project(db: &State<DatabaseConnection>, id: i32) -> Option<Json<model::projects::ModelEx>> {
    match service::projects::get_by_id(db, id).await {
        Some(project) => Some(Json(project)),
        None => None
    }
}

#[launch]
async fn rocket() -> _ {
    let db = match service::connect_db().await {
        Ok(db) => db,
        Err(e) => { panic!("Failed to connect to database: {}", e); }
    };
    rocket::build().mount("/", routes![index, projects, project])
        .manage(db)
}
