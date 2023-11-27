use crate::{db::PrismaClient, error::AppError};

pub fn hash_password(password: &str) -> String {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap()
}

pub struct AppPrismaClient {
    client: PrismaClient,
}

impl AppPrismaClient {
    pub fn new(client: PrismaClient) -> Self {
        Self { client }
    }

    pub async fn query<T>(&self, query: prisma_client_rust::Raw) -> Result<Vec<T>, AppError>
    where
        T: prisma_client_rust::Data,
    {
        let data: Vec<T> = self.client._query_raw(query).exec().await?;
        Ok(data)
    }

    pub async fn query_one<T>(&self, query: prisma_client_rust::Raw) -> Result<Option<T>, AppError>
    where
        T: prisma_client_rust::Data,
    {
        let data: Vec<T> = self.client._query_raw(query).exec().await?;
        Ok(data.into_iter().next())
    }
}
