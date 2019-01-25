use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use std::sync::Arc;
use uuid::Uuid;

////////////////////////////////////////////////////////////////////////////////

pub(crate) type ConnectionPool = Arc<Pool<ConnectionManager<PgConnection>>>;

pub(crate) fn create_database_pool(url: &str, size: u32) -> ConnectionPool {
    let manager = ConnectionManager::<PgConnection>::new(url);
    let pool = Pool::builder()
        .max_size(size)
        .build(manager)
        .expect("Error creating a database pool");

    Arc::new(pool)
}
