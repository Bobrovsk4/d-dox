use axum::{Json, extract::State, routing::post};
use loco_rs::{controller::Routes, prelude::*};
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

use crate::models::role;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateRoleRequest {
    pub name: String,
    pub attributes: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteRoleRequest {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct RoleResponse {
    pub id: i32,
    pub name: String,
    pub attributes: serde_json::Value,
}

pub async fn create_role(
    State(ctx): State<AppContext>,
    Json(payload): Json<CreateRoleRequest>,
) -> Result<Json<RoleResponse>> {
    if role::find_by_name(&ctx.db, &payload.name).await?.is_some() {
        return Err(Error::Message("Role already exists".into()));
    }

    let attributes = payload.attributes.unwrap_or(serde_json::json!([]));

    let created_role = role::create(&ctx.db, &payload.name, attributes).await?;

    Ok(Json(RoleResponse {
        id: created_role.id,
        name: created_role.name,
        attributes: created_role.attributes,
    }))
}

pub async fn delete_role(
    State(ctx): State<AppContext>,
    Json(payload): Json<DeleteRoleRequest>,
) -> Result<Json<()>> {
    let found_role = role::find_by_name(&ctx.db, &payload.name)
        .await?
        .ok_or_else(|| Error::Message("Role not found".into()))?;

    role::Entity::delete_many()
        .filter(role::Column::Id.eq(found_role.id))
        .exec(&ctx.db)
        .await
        .map_err(|e| Error::Message(e.to_string()))?;

    Ok(Json(()))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/roles")
        .add("", post(create_role))
        .add("", delete(delete_role))
}
