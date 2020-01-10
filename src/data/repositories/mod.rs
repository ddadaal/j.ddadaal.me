use tokio_diesel::AsyncResult;

pub mod jumps;
pub mod stats;

pub type RepositoryResult<T> = AsyncResult<T>;
