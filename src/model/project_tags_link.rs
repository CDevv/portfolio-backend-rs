use sea_orm::entity::prelude::*;
use serde::Serialize;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "project_tags_link")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub project_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub tag_id: i32,
    #[sea_orm(belongs_to, from = "project_id", to = "id")]
    pub project: BelongsTo<super::projects::Entity>,
    #[sea_orm(belongs_to, from = "tag_id", to = "id")]
    pub tag: BelongsTo<super::project_tags::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}