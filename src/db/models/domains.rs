use crate::db::models::users::User;
use crate::db::schema::domain;
use crate::db::schema::domain_status;
use crate::db::DbExecutor;
use crate::hashers::PBKDF2PasswordHasher;
use actix::{Handler, Message};
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use diesel::backend::Backend;
use diesel::deserialize as diesel_deserialize;
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::serialize as diesel_serialize;
use diesel::sql_types::Integer;
use serde_derive::{Deserialize, Serialize};
use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::ptr::hash;

#[repr(i32)]
#[derive(Debug, PartialEq, AsExpression, Clone, Serialize, Deserialize, FromSqlRow)]
#[sql_type = "Integer"]
pub enum DomainState {
    Enabled,
    Disabled,
    Removed,
}

impl<DB> diesel_serialize::ToSql<Integer, DB> for DomainState
where
    DB: Backend,
    i32: diesel_serialize::ToSql<Integer, DB>,
{
    fn to_sql<W: Write>(
        &self,
        out: &mut diesel_serialize::Output<W, DB>,
    ) -> diesel_serialize::Result {
        match self {
            &DomainState::Enabled => 1,
            &DomainState::Disabled => 2,
            _ => 0,
        }
        .to_sql(out)
    }
}

impl<DB> diesel_deserialize::FromSql<Integer, DB> for DomainState
where
    DB: Backend,
    i32: diesel_deserialize::FromSql<Integer, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> diesel_deserialize::Result<Self> {
        match i32::from_sql(bytes) {
            Ok(1) => Ok(DomainState::Enabled),
            Ok(2) => Ok(DomainState::Disabled),
            _ => Ok(DomainState::Removed),
        }
    }
}

#[derive(Associations, Identifiable, Queryable, Debug, Serialize, Deserialize, Clone)]
#[belongs_to(User, foreign_key = "author")]
#[table_name = "domain"]
pub struct Domain {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub active: DomainState,
    pub author: i32,
}

#[derive(Debug)]
pub struct FindDomain {
    pub name: Option<String>,
    pub status: DomainState,
}

impl Message for FindDomain {
    type Result = io::Result<Vec<Domain>>;
}

impl Handler<FindDomain> for DbExecutor {
    type Result = io::Result<Vec<Domain>>;

    fn handle(&mut self, domain_msg: FindDomain, ctx: &mut Self::Context) -> Self::Result {
        use crate::db::schema::domain::dsl::*;

        debug!("Get domain from {:?}", &domain_msg);

        let query_result = match &domain_msg.name {
            Some(domain_name) => domain
                .filter(state.eq(&domain_msg.status).and(name.eq(domain_name)))
                .load::<Domain>(&self.pool.get().unwrap()),
            None => domain
                .filter(state.eq(&domain_msg.status))
                .load::<Domain>(&self.pool.get().unwrap()),
        };

        match query_result {
            Ok(mut domains_db) => Ok(domains_db),
            Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Database error")),
        }
    }
}

#[derive(Associations, Identifiable, Queryable, Debug, Serialize, Deserialize, Clone)]
#[table_name = "domain_status"]
#[belongs_to(Domain, foreign_key = "domain_id")]
pub struct DomainStatus {
    pub id: i32,
    pub date: NaiveDateTime,
    pub loading_time: i32,
    pub status_code: i32,
    pub headers: String,
    pub filename: String,
    #[column_name = "domain_id"]
    pub domain: i32,
}

#[derive(Insertable, Serialize, Deserialize, Clone, Debug)]
#[table_name = "domain_status"]
pub struct InsertDomainStatusRequest {
    pub date: NaiveDateTime,
    pub loading_time: i32,
    pub status_code: i32,
    pub headers: String,
    pub filename: String,
    pub domain_id: i32,
}

impl Message for InsertDomainStatusRequest {
    type Result = io::Result<Vec<String>>;
}

impl Handler<InsertDomainStatusRequest> for DbExecutor {
    type Result = io::Result<Vec<String>>;

    fn handle(
        &mut self,
        insert_msg: InsertDomainStatusRequest,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        use crate::db::schema::domain_status::dsl::*;

        let inserted = match diesel::insert_into(domain_status)
            .values(&insert_msg)
            .execute(&self.pool.get().unwrap())
        {
            Ok(_) => true,
            Err(err) => false,
        };

        if inserted && self.config.rotate_domain_statuses {
            //calculate date of greater self.config.rotate_days
            let dt_rotate = Utc::now() - Duration::days(self.config.rotate_days as i64);

            let subquery = domain_status
                .filter(
                    domain_id
                        .eq(&insert_msg.domain_id)
                        .and(date.lt(dt_rotate.naive_utc())),
                )
                .order(date.desc())
                .select((id, filename))
                .load::<(i32, String)>(&self.pool.get().unwrap())
                .unwrap();

            let ids = subquery.iter().map(|el| el.0).collect::<Vec<i32>>();
            diesel::delete(domain_status.filter(id.eq_any(ids))).execute(&self.pool.get().unwrap());

            let filenames = subquery
                .iter()
                .map(|el| el.1.to_string())
                .collect::<Vec<String>>();

            Ok(filenames)
        } else {
            Err(Error::new(
                ErrorKind::Other,
                "Cannot insert value".to_string(),
            ))
        }
    }
}
