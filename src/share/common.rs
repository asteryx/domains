use serde_derive::{Deserialize, Serialize};

use crate::models::db::ConnDsl;
use actix::Addr;

pub struct AppState {
    pub db: Addr<ConnDsl>,
}

pub const PAGE_SIZE: i32 = 33;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
}
