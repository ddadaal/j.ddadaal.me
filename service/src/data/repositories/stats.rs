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

#[cfg(test)]
pub mod mock {
    use super::*;
    use crate::data::db::models::Stat;
    use chrono;
    use std::cell::RefCell;
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::sync::MutexGuard;

    #[derive(Clone)]
    pub struct StatsRepositoryMock {
        // Interior Mutability Pattern
        // It might be shared, so should be Arc<Mutex,
        // the multi-threaded counterpart of Rc<RefCell
        data: Arc<Mutex<Vec<Stat>>>,
    }

    #[async_trait]
    impl StatsRepository for StatsRepositoryMock {
        async fn add_access(
            &self,
            ip: String,
            jump_from: String,
            url: String,
            time: NaiveDateTime,
        ) -> RepositoryResult<()> {
            self.data.lock().unwrap().push(Stat {
                id: None,
                ip,
                jump_from,
                url,
                time,
            });
            Ok(())
        }
    }

    impl StatsRepositoryMock {
        pub fn new() -> Self {
            StatsRepositoryMock {
                data: Arc::new(Mutex::new(Vec::new())),
            }
        }

        pub fn add<'a>(&mut self, ip: &'a str, jump_from: &'a str, url: &'a str) -> &Self {
            self.data.lock().unwrap().push(Stat {
                id: None,
                ip: ip.into(),
                jump_from: jump_from.into(),
                url: url.into(),
                time: chrono::Local::now().naive_local(),
            });
            self
        }
        pub fn data(&self) -> MutexGuard<Vec<Stat>> {
            self.data.lock().unwrap()
        }
    }
}
