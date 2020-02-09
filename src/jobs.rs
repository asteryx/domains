use crate::db::models::domains::{DomainList, DomainState};
use crate::services::ping::PingRequest;
use crate::AppState;
use actix_web::web;
use futures::executor::block_on;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

pub fn ping_fn(state: Arc<web::Data<AppState>>) {
    loop {
        let result = block_on(state.db.send(DomainList {
            limit: 0,
            offset: 0,
            state: Some(DomainState::Enabled),
            search_string: None,
        }))
        .unwrap_or_else(|err| {
            error!("Error mailbox respond {}", err);

            let nw: Result<Vec<_>, _> = Ok(Vec::new());
            nw
        });

        if let Ok(domains) = result {
            for domain in &domains {
                state.ping.do_send(PingRequest {
                    domain: domain.clone(),
                    state: state.clone(),
                });
            }
        };
        sleep(Duration::from_secs(state.config.ping_interval()));
    }
}
