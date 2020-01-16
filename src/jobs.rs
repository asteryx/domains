use crate::db::models::domains::{Domain, DomainState, FindDomain};
use crate::services::ping::PingRequest;
use crate::AppState;
use actix::prelude::*;
use actix_web::web;
use futures::executor::block_on;
use futures::Sink;
use reqwest;
use std::io::{Error as IoError, ErrorKind as ioErrorKind};
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, Instant};

pub fn ping_fn(state: Arc<web::Data<AppState>>) {
    loop {
        let result = block_on(state.db.send(FindDomain {
            name: None,
            status: DomainState::Enabled,
        }))
        .map_err(|err| IoError::new(ioErrorKind::Interrupted, err))
        .and_then(|result| result);

        if let Ok(domains) = result {
            for domain in &domains {
                state.ping.do_send(PingRequest {
                    domain: domain.clone(),
                    state: state.clone(),
                });
            }
        };
        sleep(Duration::from_secs(state.config.ping_interval));
    }
}
