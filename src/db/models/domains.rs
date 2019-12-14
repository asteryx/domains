use crate::db::models::users::User;
use crate::db::schema::domain;
use crate::db::schema::domain_status;
use crate::db::DbExecutor;
use crate::hashers::PBKDF2PasswordHasher;
use actix::{Handler, Message};
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::backend::Backend;
use diesel::deserialize as diesel_deserialize;
use diesel::prelude::*;
use diesel::serialize as diesel_serialize;
use diesel::sql_types::Integer;
use diesel::sqlite::Sqlite;
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

impl diesel_deserialize::FromSql<Integer, Sqlite> for DomainState {
    fn from_sql(bytes: Option<&<Sqlite as Backend>::RawValue>) -> diesel_deserialize::Result<Self> {
        Ok(
            match <i32 as diesel_deserialize::FromSql<Integer, Sqlite>>::from_sql(bytes) {
                Ok(1) => DomainState::Enabled,
                Ok(2) => DomainState::Disabled,
                _ => DomainState::Removed,
            },
        )
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

        log::info!("Get domain from {:?}", &domain_msg);

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
    pub id: usize,
    pub date: String,
    pub loading_time: NaiveDateTime,
    pub status_code: usize,
    pub headers: String,
    #[column_name = "domain_id"]
    pub domain: usize,
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
    type Result = io::Result<()>;
}

impl Handler<InsertDomainStatusRequest> for DbExecutor {
    type Result = io::Result<()>;

    fn handle(
        &mut self,
        insert_msg: InsertDomainStatusRequest,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        use crate::db::schema::domain_status::dsl::*;

        match diesel::insert_into(domain_status)
            .values(&insert_msg)
            .execute(&self.pool.get().unwrap())
        {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::new(ErrorKind::Other, err.to_string())),
        }
    }
}
