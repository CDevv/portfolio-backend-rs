use sea_orm::{DatabaseConnection, EntityLoaderTrait, EntityTrait};
use crate::model;
use crate::model::projects as db_projects;

pub async fn get_all(db: &DatabaseConnection) -> Vec<db_projects::ModelEx> {
    //db_projects::Entity::find().all(db).await.unwrap()
    db_projects::Entity::load().with(model::project_tags::Entity).all(db).await.unwrap()
}

pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Option<db_projects::ModelEx> {
    //db_projects::Entity::find_by_id(id).one(db).await.unwrap()
    db_projects::Entity::load().with(model::project_tags::Entity).filter_by_id(id).one(db).await.unwrap()
}