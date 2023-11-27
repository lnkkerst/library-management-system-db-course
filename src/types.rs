use std::sync::Arc;

use axum::Extension;
use sqlx::MssqlPool;

use crate::{db::PrismaClient, utils::AppPrismaClient};

pub type Database = Extension<Arc<PrismaClient>>;
pub type AppDatabase = Extension<Arc<AppPrismaClient>>;
pub type Pool = Extension<Arc<MssqlPool>>;
