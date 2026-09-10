use sea_orm::entity::prelude::*;
use serde::Serialize;
use crate::model;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "projects")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub description: String,
    pub github: String,
    pub itch: Option<String>,
    pub weblink: Option<String>,
    #[sea_orm(has_many, via = "project_tags_link")]
    pub tags: HasMany<model::project_tags::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}