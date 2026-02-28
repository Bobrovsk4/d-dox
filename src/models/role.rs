use loco_rs::prelude::*;
use sea_orm::{entity::prelude::*, ActiveValue::NotSet};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "roles")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    #[sea_orm(column_type = "Json")]
    pub attributes: serde_json::Value,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub async fn create(
    db: &DatabaseConnection,
    name: &str,
    attributes: serde_json::Value,
) -> Result<Model, DbErr> {
    let res = Entity::insert(ActiveModel {
        id: NotSet,
        name: Set(name.to_string()),
        attributes: Set(attributes),
    })
    .exec(db)
    .await?;

    Entity::find_by_id(res.last_insert_id)
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound("Role not found".to_string()))
}

pub async fn find_by_name(db: &DatabaseConnection, name: &str) -> Result<Option<Model>, DbErr> {
    Entity::find().filter(Column::Name.eq(name)).one(db).await
}

pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
    Entity::find().all(db).await
}
