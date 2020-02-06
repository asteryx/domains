pub mod models;
pub mod schema;

use crate::config::Config;
use actix::{Actor, SyncContext};
use diesel::prelude::PgConnection;
use diesel::r2d2;
use diesel::r2d2::{ConnectionManager, Pool};

pub enum DbError {
    GetConnectionError,
}

pub struct DbExecutor {
    pool: Pool<ConnectionManager<PgConnection>>,
    config: Config,
}

unsafe impl Send for DbExecutor {}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl Default for DbExecutor {
    fn default() -> Self {
        let config = Config::from_file();
        let pool = init_pool(&config);
        DbExecutor::new(config, pool)
    }
}

impl DbExecutor {
    fn new(config: Config, pool: Pool<ConnectionManager<PgConnection>>) -> DbExecutor {
        DbExecutor { config, pool }
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

pub fn init_pool(config: &Config) -> Pool<ConnectionManager<PgConnection>> {
    let manager: ConnectionManager<PgConnection> =
        ConnectionManager::<PgConnection>::new(config.database_url());
    Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Failed to create pool.")
}
