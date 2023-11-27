use anyhow::{anyhow, Result};
use axum::{http::StatusCode, Json};
use prisma_client_rust::{raw, PrismaValue};

use crate::{
    db::user,
    error::AppError,
    models::auth::{AuthBody, UserClaims, UserLoginPayload, UserRegisterPayload, ADMIN_TYPE_ID},
    types::{AppDatabase, Database},
    utils::hash_password,
};

pub async fn user_register(
    odb: Database,
    db: AppDatabase,
    Json(payload): Json<UserRegisterPayload>,
) -> Result<Json<AuthBody>, AppError> {
    let db_user = db
        .query_one::<user::Data>(raw!(
            include_str!("../../sql/user/select_by_name.sql"),
            PrismaValue::String(payload.username.clone())
        ))
        .await?;

    match db_user {
        Some(_) => Err(AppError::new(
            StatusCode::CONFLICT,
            "username_conflict",
            anyhow!(format!(
                "User with name `{}` already exists",
                payload.username
            )),
        )),
        None => {
            let hashed_password = hash_password(&payload.password);
            let user = odb
                .user()
                .create(
                    payload.username,
                    hashed_password,
                    crate::db::user_type::id::equals(ADMIN_TYPE_ID.to_owned()),
                    "all".to_string(),
                    vec![],
                )
                .exec()
                .await?;

            let user_claims = UserClaims {
                id: user.id,
                name: user.name,
                permission: user.permission,
                exp: 3600 * 24 * 30,
            };

            Ok(Json(AuthBody::new(user_claims.sign().await?)))
        }
    }
}

pub async fn user_login(
    db: Database,
    Json(payload): Json<UserLoginPayload>,
) -> Result<Json<AuthBody>, AppError> {
    let db_user = db
        .user()
        .find_unique(user::name::equals(payload.username.clone()))
        .exec()
        .await?
        .ok_or(AppError::new(
            StatusCode::NOT_FOUND,
            "user_not_exists",
            anyhow!(format!("User with name `{}`, not exists", payload.username)),
        ))?;
    let user_claims: UserClaims = db_user.into();
    Ok(Json(AuthBody::new(user_claims.sign().await?)))
}
