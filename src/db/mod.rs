pub mod models;
pub mod schema;

use crate::CONFIG;
use actix::{Actor, SyncContext};
use diesel::prelude::PgConnection;
use diesel::r2d2;
use diesel::r2d2::{ConnectionManager, Pool};

pub enum DbError {
    GetConnectionError,
}

pub struct DbExecutor {
    pool: Pool<ConnectionManager<PgConnection>>,
}

unsafe impl Send for DbExecutor {}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl Default for DbExecutor {
    fn default() -> Self {
        let pool = init_pool();
        DbExecutor::new(pool)
    }
}

impl DbExecutor {
    fn new(pool: Pool<ConnectionManager<PgConnection>>) -> DbExecutor {
        DbExecutor { pool }
    }
    pub fn get_connection(
        &self,
    ) -> Result<r2d2::PooledConnection<ConnectionManager<PgConnection>>, DbError> {
        match self.pool.get() {
            Ok(conn) => Ok(conn),
            Err(_) => Err(DbError::GetConnectionError),
        }
    }
}

pub fn init_pool() -> Pool<ConnectionManager<PgConnection>> {
    let manager: ConnectionManager<PgConnection> =
        ConnectionManager::<PgConnection>::new(CONFIG.database_url());
    Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Failed to create pool.")
}
