use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_pool(db_url: &str) -> DbPool {
    let manager = r2d2::ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Failed to create pool")
}
