use crate::data::db::models::Stat;
use crate::data::db::schema::stats;
use crate::data::repositories::RepositoryResult;
use crate::Pool;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use diesel::*;
use tokio_diesel::*;

#[async_trait]
pub trait StatsRepository {
    async fn add_access(
        &self,
        ip: String,
        jump_from: String,
        url: String,
        time: NaiveDateTime,
    ) -> RepositoryResult<()>;
}

#[derive(Clone)]
pub struct StatsRepositoryImpl {
    pool: Pool,
}

#[async_trait]
impl StatsRepository for StatsRepositoryImpl {
    async fn add_access(
        &self,
        ip: String,
        jump_from: String,
        url: String,
        time: NaiveDateTime,
    ) -> RepositoryResult<()> {
        // INSERT INTO `stats` VALUES ({ip},{jump_from},{url},{time});
        let model = Stat {
            id: None,
            ip,
            jump_from,
            url,
            time,
        };

        let result = insert_into(stats::table)
            .values(model)
            .execute_async(&self.pool)
            .await;

        result.and(Ok(()))
    }
}

impl StatsRepositoryImpl {
    pub fn new(pool: Pool) -> Self {
        StatsRepositoryImpl { pool }
    }
}
