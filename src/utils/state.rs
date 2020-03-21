use crate::db::DbExecutor;
use crate::services::ping::Ping;
use actix::prelude::*;

pub struct AppState {
    pub db: Addr<DbExecutor>,
    pub ping: Addr<Ping>,
}

impl AppState {
    pub fn new(db: Addr<DbExecutor>, ping: Addr<Ping>) -> AppState {
        AppState { db, ping }
    }
}

impl std::fmt::Debug for AppState {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AppState")
            .field("db", &format_args!("Pool<ConnectionManager<PgConnection>>"))
            .field("ping", &format_args!("Addr<Ping>"))
            .finish()
    }
}
