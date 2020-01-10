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

#[cfg(test)]
pub mod mock {
    use super::*;
    use crate::data::db::models::Jump;
    use chrono;

    #[derive(Clone)]
    pub struct JumpsRepositoryMock {
        data: Vec<Jump>,
    }

    #[async_trait]
    impl JumpsRepository for JumpsRepositoryMock {
        async fn get_jump_link(&self, from: String) -> RepositoryResult<Option<String>> {
            Ok(self
                .data
                .iter()
                .filter(|x| x.from == from)
                .nth(0)
                .map(|x| x.to.clone()))
        }
    }

    impl JumpsRepositoryMock {
        pub fn new<'a>(vec: Vec<(&str, &str)>) -> Self {
            JumpsRepositoryMock {
                data: vec
                    .into_iter()
                    .enumerate()
                    .map(|(i, (from, to))| Jump {
                        id: i as i32,
                        from: from.into(),
                        to: to.into(),
                        create_time: chrono::Local::now().naive_utc(),
                    })
                    .collect(),
            }
        }
    }
}
