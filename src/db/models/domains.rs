use crate::db::models::users::User;
use crate::db::schema::domain;
use crate::db::schema::domain_status;
use crate::db::DbExecutor;
use actix::{Handler, Message};
use chrono::{Duration, NaiveDateTime, Utc};
use diesel::backend::Backend;
use diesel::prelude::*;
use diesel::serialize as diesel_serialize;
use diesel::sql_types::Integer;
use diesel::{deserialize as diesel_deserialize, sql_query};
use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};

#[repr(i32)]
#[derive(Debug, PartialEq, AsExpression, Clone, Serialize, Deserialize, FromSqlRow)]
#[sql_type = "Integer"]
pub enum DomainState {
    Enabled,
    Disabled,
    Removed,
}

impl Display for DomainState {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let res = match self {
            &DomainState::Enabled => "1".to_string(),
            &DomainState::Disabled => "2".to_string(),
            _ => "0".to_string(),
        };
        write!(f, "{}", res)
    }
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

#[derive(
    Associations, Identifiable, Queryable, Debug, Serialize, Deserialize, Clone, QueryableByName,
)]
#[belongs_to(User, foreign_key = "author")]
#[table_name = "domain"]
pub struct Domain {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub state: DomainState,
    pub author: i32,
}

#[derive(Debug)]
pub struct DomainList {
    pub limit: usize,
    pub offset: usize,
    pub status: Option<DomainState>,
    pub search_string: Option<String>,
}

impl Message for DomainList {
    type Result = io::Result<Vec<Domain>>;
}

impl Handler<DomainList> for DbExecutor {
    type Result = io::Result<Vec<Domain>>;

    fn handle(&mut self, domain_msg: DomainList, _ctx: &mut Self::Context) -> Self::Result {
        //        use crate::db::schema::domain::dsl::*;

        debug!("Get domain from {:?}", &domain_msg);

        let mut query = "SELECT * FROM domain ".to_string();
        if let Some(domain_state) = &domain_msg.status {
            query.push_str(format!("WHERE state = {} ", domain_state).as_str());
        }
        if domain_msg.limit > 0usize {
            query.push_str(format!("LIMIT {} ", domain_msg.limit).as_str());
        }
        if domain_msg.offset > 0usize {
            query.push_str(format!("LIMIT {} ", domain_msg.offset).as_str());
        }
        query.push_str("ORDER BY id");
        dbg!(&query);

        let query_result = sql_query(query).load::<Domain>(&self.pool.get().unwrap());

        match query_result {
            Ok(domains_db) => Ok(domains_db),
            Err(err) => {
                error!("Error id db search {}", err);
                let vvv: Vec<Domain> = Vec::new();
                Ok(vvv)
            }
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
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        use crate::db::schema::domain_status::dsl::*;

        let inserted = match diesel::insert_into(domain_status)
            .values(&insert_msg)
            .execute(&self.pool.get().unwrap())
        {
            Ok(_) => true,
            Err(_) => false,
        };

        if inserted && self.config.domain_statuses_rotation() {
            //calculate date of greater self.config.rotate_days
            let dt_rotate =
                Utc::now() - Duration::days(self.config.domain_statuses_rotate_days() as i64);

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
            diesel::delete(domain_status.filter(id.eq_any(ids)))
                .execute(&self.pool.get().unwrap())
                .ok();

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
