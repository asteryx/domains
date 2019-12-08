use crate::config::Config;
use crate::db::DbExecutor;
use crate::services::ping::Ping;
use actix::prelude::*;

pub struct AppState {
    pub config: Config,
    pub db: Addr<DbExecutor>,
    pub ping: Addr<Ping>,
}

impl AppState {
    pub fn new(db: Addr<DbExecutor>, ping: Addr<Ping>) -> AppState {
        let config: Config = Config::from_file();
        AppState { config, db, ping }
    }
}

impl std::fmt::Debug for AppState {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AppState")
            .field("config", &self.config)
            .field(
                "db",
                &format_args!("Pool<ConnectionManager<SqliteConnection>>"),
            )
            .field("ping", &format_args!("Addr<Ping>"))
            .finish()
    }
}
