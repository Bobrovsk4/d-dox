use chrono::Utc;
use loco_rs::prelude::*;
use sea_orm::{ActiveValue::NotSet, entity::prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "files")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub size: i64,
    pub author_id: i32,
    #[sea_orm(column_type = "Timestamp")]
    pub created_at: sea_orm::prelude::DateTime,
    #[sea_orm(column_type = "Timestamp")]
    pub updated_at: sea_orm::prelude::DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::AuthorId",
        to = "super::user::Column::Id"
    )]
    Author,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Author.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub async fn create(
    db: &DatabaseConnection,
    name: &str,
    size: i64,
    author_id: i32,
) -> Result<Model, DbErr> {
    let now = Utc::now().naive_utc();
    let res = Entity::insert(ActiveModel {
        id: NotSet,
        name: Set(name.to_string()),
        size: Set(size),
        author_id: Set(author_id),
        created_at: Set(now),
        updated_at: Set(now),
    })
    .exec(db)
    .await?;

    Entity::find_by_id(res.last_insert_id)
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound("File not found".to_string()))
}

pub async fn find_by_name(db: &DatabaseConnection, name: &str) -> Result<Option<Model>, DbErr> {
    Entity::find().filter(Column::Name.eq(name)).one(db).await
}

pub async fn find_all_with_authors(
    db: &DatabaseConnection,
) -> Result<Vec<(Model, Option<super::user::Model>)>, DbErr> {
    Entity::find()
        .find_also_related(super::user::Entity)
        .all(db)
        .await
}

pub async fn find_with_author(
    db: &DatabaseConnection,
    id: i32,
) -> Result<Option<(Model, Option<super::user::Model>)>, DbErr> {
    Entity::find()
        .find_also_related(super::user::Entity)
        .filter(Column::Id.eq(id))
        .one(db)
        .await
}

pub async fn sync_by_name_and_author(
    db: &DatabaseConnection,
    name: &str,
    size: i64,
    author_id: i32,
) -> Result<Model, DbErr> {
    use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};

    let now = Utc::now().naive_utc();

    if let Some(existing) = Entity::find()
        .filter(Column::Name.eq(name))
        .filter(Column::AuthorId.eq(author_id))
        .one(db)
        .await?
    {
        let mut active_model: ActiveModel = existing.into();
        active_model.size = Set(size);
        active_model.updated_at = Set(now);
        return active_model.update(db).await;
    }

    Entity::insert(ActiveModel {
        id: NotSet,
        name: Set(name.to_string()),
        size: Set(size),
        author_id: Set(author_id),
        created_at: Set(now),
        updated_at: Set(now),
    })
    .exec(db)
    .await?;

    Entity::find()
        .filter(Column::Name.eq(name))
        .filter(Column::AuthorId.eq(author_id))
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound("File not found".to_string()))
}
