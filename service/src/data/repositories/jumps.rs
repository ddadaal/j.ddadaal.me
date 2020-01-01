use crate::data::db::schema::jumps;
use crate::data::repositories::RepositoryResult;
use crate::Pool;
use async_trait::async_trait;
use diesel::*;
use tokio_diesel::{OptionalExtension, *};

#[async_trait]
pub trait JumpsRepository {
    async fn get_jump_link(&self, from: String) -> RepositoryResult<Option<String>>;
}

#[derive(Clone)]
pub struct JumpsRepositoryImpl {
    pool: Pool,
}

#[async_trait]
impl JumpsRepository for JumpsRepositoryImpl {
    async fn get_jump_link(&self, from: String) -> RepositoryResult<Option<String>> {
        // SELECT full_link FROM links WHERE short_link = {short_link};
        jumps::table
            .filter(jumps::from.eq(from))
            .limit(1)
            .select(jumps::to)
            .get_result_async::<String>(&self.pool)
            .await
            .optional()
    }
}

impl JumpsRepositoryImpl {
    pub fn new(pool: Pool) -> Self {
        JumpsRepositoryImpl { pool }
    }
}
