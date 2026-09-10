pub mod service;
pub mod model;

#[macro_use] extern crate rocket;

use rocket::State;
use rocket::serde::json::Json;
use sea_orm::{DatabaseConnection};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::model::projects::ModelEx;
use crate::service::ErrorResponder;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[utoipa::path]
#[get("/projects")]
async fn projects(db: &State<DatabaseConnection>) -> Result<Json<Vec<ModelEx>>, ErrorResponder> {
    match service::projects::get_all(db).await {
        Ok(projects) => Ok(Json(projects)),
        Err(err) => Err(err.into())
    }
}

#[utoipa::path]
#[get("/projects/<id>")]
async fn project(db: &State<DatabaseConnection>, id: i32) -> Result<Option<Json<ModelEx>>, ErrorResponder> {
    match service::projects::get_by_id(db, id).await {
        Ok(project) => { Ok(project.map(|p| Json(p))) }
        Err(err) => Err(err.into())
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(projects, project)
)]
struct ApiDoc;

#[launch]
async fn rocket() -> _ {
    let db = match service::connect_db().await {
        Ok(db) => db,
        Err(e) => { panic!("Failed to connect to database: {}", e); }
    };
    rocket::build().mount("/", routes![index, projects, project])
        .mount("/", SwaggerUi::new("/api/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .manage(db)
}
