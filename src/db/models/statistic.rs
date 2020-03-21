use crate::config::Config;
use crate::db::models::domains::{Domain, DomainState, DomainStatusShort};
use crate::db::schema::domain;
use crate::db::schema::domain_status;
use crate::db::DbExecutor;
use actix::{Handler, Message};
use chrono::{DateTime, Duration, Local, NaiveDateTime, Utc};
use diesel::debug_query;
use diesel::prelude::*;
use serde_derive::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};
use serde_json;
use std::io;
use validator::{Validate, ValidationError};

fn deserialize_domain_list(serialized: &str) -> Result<Vec<i32>, serde_json::Error> {
    serde_json::from_str(&serialized)
}

fn validate_domain_list(string_list: &str) -> Result<(), ValidationError> {
    let deserialized: Result<Vec<i32>, _> = deserialize_domain_list(string_list);
    match deserialized {
        Ok(_) => Ok(()),
        Err(err) => {
            // dbg!(err);
            Err(ValidationError::new("invalid_domain_list"))
        }
    }
}

#[derive(Debug, SerdeSerialize, SerdeDeserialize, Validate)]
pub struct Statistic {
    pub dt_start: Option<DateTime<Local>>,
    pub dt_end: Option<DateTime<Local>>,
    #[validate(custom = "validate_domain_list")]
    pub domain_list: Option<String>,
}

#[derive(Debug, SerdeSerialize, SerdeDeserialize, Validate)]
pub struct DomainGroup {
    pub id: i32,
    pub name: String,
    pub statuses: Vec<DomainStatusShort>,
}

impl Message for Statistic {
    type Result = io::Result<Option<Vec<DomainGroup>>>;
}

impl Handler<Statistic> for DbExecutor {
    type Result = io::Result<Option<Vec<DomainGroup>>>;

    fn handle(&mut self, domain_msg: Statistic, _ctx: &mut Self::Context) -> Self::Result {
        let config = Config::from_file();

        let mut query = domain::table
            .inner_join(domain_status::table.on(domain::id.eq(domain_status::domain_id)))
            .into_boxed();

        if let Some(start_date) = &domain_msg.dt_start {
            query = query.filter(domain_status::date.ge(start_date.naive_utc()));
        } else {
            let today = Local::today().and_hms(0, 0, 0);
            query = query.filter(domain_status::date.ge(today.naive_utc()));
        }

        if let Some(end_date) = &domain_msg.dt_end {
            query = query.filter(domain_status::date.le(end_date.naive_utc()));
        }

        if let Some(domain_list) = &domain_msg.domain_list {
            if let Ok(list) = deserialize_domain_list(domain_list) {
                query = query.filter(domain_status::domain_id.eq_any(list));
            }
        } else {
            query = query.filter(domain::state.ne(DomainState::Removed));
        }

        let sql = debug_query::<_, _>(&query).to_string();
        debug!("`{}`", sql);
        //
        let query_result = query
            .select((
                domain::id,
                domain::name,
                domain_status::date,
                domain_status::loading_time,
                domain_status::status_code,
                domain_status::headers,
                domain_status::filename,
            ))
            .order((domain::id.asc(), domain_status::date.asc()))
            .load::<(i32, String, NaiveDateTime, i32, i32, String, Option<String>)>(
                &self.pool.get().unwrap(),
            );

        match query_result {
            Ok(domains_db) => {
                dbg!(&domains_db);
                let mut result: Vec<DomainGroup> = Vec::new();
                let mut lastid = -1;
                for (id, name, date, loading_time, status_code, headers, file) in
                    domains_db.into_iter()
                {
                    let current_status = DomainStatusShort {
                        date,
                        loading_time,
                        status_code,
                        headers,
                        filename: file,
                    };

                    if id != lastid {
                        let group = DomainGroup {
                            id,
                            name,
                            statuses: vec![current_status],
                        };
                        result.push(group);
                        lastid = id;
                    } else {
                        let length = result.len();
                        result[length - 1].statuses.push(current_status);
                    }
                }
                dbg!(&result);
                Ok(Some(result))
            }
            Err(err) => {
                error!("Error in db search {}", &err);
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Error search object. Please retry..",
                ))
            }
        }
    }
}
