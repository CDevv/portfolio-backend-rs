use sea_orm::{DatabaseConnection, DbErr, EntityLoaderTrait};
use crate::model;
use crate::model::projects as db_projects;

pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<db_projects::ModelEx>, DbErr> {
    db_projects::Entity::load()
        .with(model::project_tags::Entity)
        .all(db).await
}

pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<db_projects::ModelEx>, DbErr> {
    //db_projects::Entity::find_by_id(id).one(db).await.unwrap()
    db_projects::Entity::load()
        .with(model::project_tags::Entity)
        .filter_by_id(id)
        .one(db).await
}