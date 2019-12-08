use crate::db::models::users::User;
use crate::db::schema::domains;
use crate::db::DbExecutor;
use crate::hashers::PBKDF2PasswordHasher;
use actix::{Handler, Message};
use diesel::backend::Backend;
use diesel::deserialize as diesel_deserialize;
use diesel::prelude::*;
use diesel::serialize as diesel_serialize;
use diesel::sql_types::Integer;
use diesel::sqlite::Sqlite;
use serde_derive::{Deserialize, Serialize};
use std::io;
use std::io::prelude::*;
use std::io::Error;
use std::ptr::hash;

#[repr(i32)]
#[derive(Debug, PartialEq, AsExpression, Clone, Serialize, Deserialize, FromSqlRow)]
#[sql_type = "Integer"]
pub enum DomainStatus {
    Enabled,
    Disabled,
    Removed,
}

impl<DB> diesel_serialize::ToSql<Integer, DB> for DomainStatus
where
    DB: Backend,
    i32: diesel_serialize::ToSql<Integer, DB>,
{
    fn to_sql<W: Write>(
        &self,
        out: &mut diesel_serialize::Output<W, DB>,
    ) -> diesel_serialize::Result {
        match self {
            &DomainStatus::Enabled => 1,
            &DomainStatus::Disabled => 2,
            _ => 0,
        }
        .to_sql(out)
    }
}

impl diesel_deserialize::FromSql<Integer, Sqlite> for DomainStatus {
    fn from_sql(bytes: Option<&<Sqlite as Backend>::RawValue>) -> diesel_deserialize::Result<Self> {
        Ok(
            match <i32 as diesel_deserialize::FromSql<Integer, Sqlite>>::from_sql(bytes) {
                Ok(1) => DomainStatus::Enabled,
                Ok(2) => DomainStatus::Disabled,
                _ => DomainStatus::Removed,
            },
        )
    }
}

#[derive(Associations, Identifiable, Queryable, Debug, Serialize, Deserialize, Clone)]
#[belongs_to(User, foreign_key = "author")]
pub struct Domain {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub active: DomainStatus,
    pub author: i32,
}

#[derive(Debug)]
pub struct FindDomain {
    pub name: Option<String>,
    pub status: DomainStatus,
}

impl Message for FindDomain {
    type Result = io::Result<Vec<Domain>>;
}

impl Handler<FindDomain> for DbExecutor {
    type Result = io::Result<Vec<Domain>>;

    fn handle(&mut self, domain_msg: FindDomain, ctx: &mut Self::Context) -> Self::Result {
        use crate::db::schema::domains::dsl::*;

        log::info!("Get domain from {:?}", &domain_msg);

        let query_result = match &domain_msg.name {
            Some(domain_name) => domains
                .filter(status.eq(&domain_msg.status).and(name.eq(domain_name)))
                .load::<Domain>(&self.pool.get().unwrap()),
            None => domains
                .filter(status.eq(&domain_msg.status))
                .load::<Domain>(&self.pool.get().unwrap()),
        };

        match query_result {
            Ok(mut domains_db) => Ok(domains_db),
            Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Database error")),
        }
    }
}
