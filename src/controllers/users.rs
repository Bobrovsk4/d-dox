use axum::{Json, extract::State, routing::post};
use loco_rs::{controller::Routes, prelude::*};
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

use crate::models::{role, user};

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterRequest {
    pub username: String,
    pub login: String,
    pub password: String,
    pub role_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteUserRequest {
    pub login: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddRoleRequest {
    pub login: String,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub login: String,
    pub role: Option<RoleResponse>,
}

#[derive(Debug, Serialize)]
pub struct RoleResponse {
    pub id: i32,
    pub name: String,
}

pub async fn register(
    State(ctx): State<AppContext>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<UserResponse>> {
    if user::find_by_login(&ctx.db, &payload.login)
        .await?
        .is_some()
    {
        return Err(Error::Message("User already exists".into()));
    }

    let role_name = payload.role_name.unwrap_or_else(|| "user".to_string());
    let role = match role::find_by_name(&ctx.db, &role_name).await? {
        Some(r) => r,
        None => {
            return Err(Error::Message(format!("Role '{}' not found", role_name)));
        }
    };

    let password_hash = hash_password(&payload.password)?;

    let created_user = user::create(
        &ctx.db,
        &payload.username,
        &payload.login,
        &password_hash,
        role.id,
    )
    .await?;

    Ok(Json(UserResponse {
        id: created_user.id,
        username: created_user.username,
        login: created_user.login,
        role: Some(RoleResponse {
            id: role.id,
            name: role.name,
        }),
    }))
}

pub async fn delete_user(
    State(ctx): State<AppContext>,
    Json(payload): Json<DeleteUserRequest>,
) -> Result<Json<()>> {
    let found_user = user::find_by_login(&ctx.db, &payload.login)
        .await?
        .ok_or_else(|| Error::Message("User not found".into()))?;

    if !verify_password(&payload.password, &found_user.password)? {
        return Err(Error::Message("Invalid password".into()));
    }

    user::Entity::delete_many()
        .filter(user::Column::Id.eq(found_user.id))
        .exec(&ctx.db)
        .await
        .map_err(|e| Error::Message(e.to_string()))?;

    Ok(Json(()))
}

pub async fn add_role(
    State(ctx): State<AppContext>,
    Json(payload): Json<AddRoleRequest>,
) -> Result<Json<UserResponse>> {
    let found_user = user::find_by_login(&ctx.db, &payload.login)
        .await?
        .ok_or_else(|| Error::Message("User not found".into()))?;

    let role = role::find_by_name(&ctx.db, &payload.role)
        .await?
        .ok_or_else(|| Error::Message(format!("Role '{}' not found", payload.role)))?;

    let user_id = found_user.id;
    let username = found_user.username.clone();
    let login = found_user.login.clone();

    let mut user_active: user::ActiveModel = found_user.into();
    user_active.role_id = Set(role.id);
    user_active
        .save(&ctx.db)
        .await
        .map_err(|e| Error::Message(e.to_string()))?;

    Ok(Json(UserResponse {
        id: user_id,
        username,
        login,
        role: Some(RoleResponse {
            id: role.id,
            name: role.name,
        }),
    }))
}

fn hash_password(password: &str) -> Result<String> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(|e| Error::Message(e.to_string()))
}

fn verify_password(password: &str, hash: &str) -> Result<bool> {
    bcrypt::verify(password, hash).map_err(|e| Error::Message(e.to_string()))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/users")
        .add("/register", post(register))
        .add("/delete", post(delete_user))
        .add("/add_role", post(add_role))
}
