use std::sync::Mutex;

use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    RequestPartsExt, TypedHeader,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::{Lazy, OnceCell};
use serde::{Deserialize, Serialize};

use crate::{
    db::{reader, user},
    error::AuthError,
};

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

pub static ADMIN_TYPE_ID_RAW: OnceCell<Mutex<String>> = OnceCell::new();

pub static ADMIN_TYPE_ID: Lazy<String> = Lazy::new(|| {
    ADMIN_TYPE_ID_RAW
        .get()
        .expect("Default user type 'admin' not initialized.")
        .lock()
        .unwrap()
        .to_string()
});

pub struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserLoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserRegisterPayload {
    pub username: String,
    pub password: String,
}

impl AuthBody {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    pub access_token: String,
    pub token_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims {
    pub id: String,
    pub name: String,
    pub permission: String,
    pub exp: usize,
}

impl From<user::Data> for UserClaims {
    fn from(value: user::Data) -> Self {
        Self {
            id: value.id,
            name: value.name,
            permission: value.permission,
            exp: 3600 * 24 * 30,
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for UserClaims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let token_data =
            decode::<UserClaims>(bearer.token(), &KEYS.decoding, &Validation::default())
                .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

impl UserClaims {
    pub async fn sign(&self) -> Result<String, AuthError> {
        encode(&Header::default(), self, &KEYS.encoding).map_err(|_| AuthError::TokenCreation)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct ReaderClaims {
    pub id: String,
    pub name: String,
    pub library_card_id: String,
    pub exp: usize,
}

impl From<reader::Data> for ReaderClaims {
    fn from(value: reader::Data) -> Self {
        Self {
            id: value.id,
            library_card_id: value.library_card_id.to_string(),
            name: value.name,
            exp: 3600 * 24 * 30,
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for ReaderClaims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let token_data =
            decode::<ReaderClaims>(bearer.token(), &KEYS.decoding, &Validation::default())
                .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

impl ReaderClaims {
    pub async fn sign(&self) -> Result<String, AuthError> {
        encode(&Header::default(), self, &KEYS.encoding).map_err(|_| AuthError::TokenCreation)
    }
}
