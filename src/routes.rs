use std::{
    env,
    str::FromStr,
    sync::{Arc, Mutex},
};

use axum::{
    routing::{self, Router},
    Extension,
};
use sqlx::mssql::{MssqlConnectOptions, MssqlPoolOptions};

use crate::{
    controllers,
    db::{user_type, PrismaClient},
    models::auth::ADMIN_TYPE_ID_RAW,
    utils::AppPrismaClient,
};

fn create_admin_router() -> Router {
    Router::new()
        .route("/hello", routing::any(|| async { "hello world" }))
        .route("/register", routing::post(controllers::auth::user_register))
}

fn create_reader_router() -> Router {
    Router::new()
}

fn create_api_router() -> Router {
    Router::new()
        .route("/hello", axum::routing::any(|| async { "hello world" }))
        .nest("/admin", create_admin_router())
        .nest("/reader", create_reader_router())
}

async fn init_server() {
    let prisma_client = Arc::new(PrismaClient::_builder().build().await.unwrap());
    let admin_type = prisma_client
        .user_type()
        .find_first(vec![user_type::name::equals("admin".to_string())])
        .exec()
        .await
        .unwrap();

    let admin_id = match admin_type {
        Some(admin_type) => admin_type.id,
        None => {
            prisma_client
                .user_type()
                .create("admin".to_string(), vec![])
                .exec()
                .await
                .unwrap()
                .id
        }
    };
    ADMIN_TYPE_ID_RAW
        .set(Mutex::new(admin_id))
        .expect("Failed to initialize default user type.");
}

pub async fn run_server(bind: &str) {
    init_server().await;

    let pool = Arc::new(
        MssqlPoolOptions::new()
            .connect_with(
                MssqlConnectOptions::from_str(
                    &env::var("DATABASE_URL").expect("env DATABASE_URL not defined"),
                )
                .expect("Failed to parse DATABASE_URL"),
            )
            .await
            .expect("Failed to connect to DB"),
    );

    let prisma_client = Arc::new(PrismaClient::_builder().build().await.unwrap());
    let app_prisma_client = Arc::new(AppPrismaClient::new(
        PrismaClient::_builder().build().await.unwrap(),
    ));

    let app = Router::new()
        .fallback(crate::controllers::spa_static)
        .nest("/api/", create_api_router())
        .layer(Extension(prisma_client))
        .layer(Extension(app_prisma_client))
        .layer(Extension(pool));

    axum::Server::bind(&bind.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
