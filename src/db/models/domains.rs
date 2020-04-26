use crate::db::models::users::User;
use crate::db::schema::domain;
use crate::db::schema::domain_status;
use crate::db::DbExecutor;
use crate::CONFIG;
use actix::{Handler, Message};
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use diesel::backend::Backend;
use diesel::debug_query;
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind as DieselErrorKind, Error as DieselError};
use diesel::sql_types::Integer;
use diesel::{deserialize as diesel_deserialize, serialize as diesel_serialize};
use serde_derive::{Deserialize as DeriveDeserialize, Serialize as DeriveSerialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::{Display, Formatter};
use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use strum_macros::EnumIter;
use url::Url;
use validator::{Validate, ValidationError};

#[repr(i32)]
#[derive(
    Debug,
    PartialEq,
    AsExpression,
    Clone,
    Copy,
    Serialize_repr,
    Deserialize_repr,
    FromSqlRow,
    EnumIter,
)]
#[sql_type = "Integer"]
pub enum DomainState {
    Enabled = 1,
    Disabled = 2,
    Removed = 0,
}

impl Display for DomainState {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let res = match self {
            DomainState::Enabled => "Enabled".to_string(),
            DomainState::Disabled => "Disabled".to_string(),
            _ => "Removed".to_string(),
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
        (*self as i32).to_sql(out)
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
    Associations,
    Identifiable,
    Insertable,
    Queryable,
    Debug,
    DeriveSerialize,
    DeriveDeserialize,
    Clone,
    QueryableByName,
)]
#[belongs_to(User, foreign_key = "author")]
#[table_name = "domain"]
pub struct Domain {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub state: DomainState,
    pub author: i32,
    pub color: String,
}

fn validate_urls(url: &str) -> Result<(), ValidationError> {
    let enabled_schemes = ["http", "https"];

    let parsed_result = Url::parse(url);

    match parsed_result {
        Ok(url) => {
            if enabled_schemes.contains(&url.scheme()) {
                Ok(())
            } else {
                Err(ValidationError::new("invalid_url"))
            }
        }
        _ => Err(ValidationError::new("invalid_url")),
    }
}

#[derive(Debug, Insertable, DeriveSerialize, DeriveDeserialize, Validate, Clone)]
#[table_name = "domain"]
pub struct DomainInsertUpdate {
    pub id: Option<i32>,
    pub name: String,
    #[validate(url, custom = "validate_urls")]
    pub url: String,
    pub state: DomainState,
    pub author: Option<i32>,
    pub color: Option<String>,
}

impl Message for DomainInsertUpdate {
    type Result = io::Result<Domain>;
}

impl Handler<DomainInsertUpdate> for DbExecutor {
    type Result = io::Result<Domain>;

    fn handle(&mut self, domain_msg: DomainInsertUpdate, _ctx: &mut Self::Context) -> Self::Result {
        use crate::db::schema::domain::dsl::*;

        dbg!(&domain_msg);

        let inserted = match domain_msg.id {
            Some(0) => {
                let query = diesel::insert_into(domain)
                    .values(&domain_msg)
                    .returning(id);

                let sql = debug_query::<Pg, _>(&query).to_string();
                debug!("`{}`", sql);

                query.get_results::<i32>(&self.pool.get().unwrap())
            }
            Some(domain_id) => {
                let query = diesel::update(domain.filter(id.eq(domain_id)))
                    .set((
                        name.eq(&domain_msg.name),
                        url.eq(&domain_msg.url),
                        state.eq(&domain_msg.state),
                        color.eq(match &domain_msg.color {
                            Some(_color) => _color.clone(),
                            _ => CONFIG.domain_default_color().clone(),
                        }),
                    ))
                    .returning(id);

                query.get_results::<i32>(&self.pool.get().unwrap())
            }
            _ => {
                let query = diesel::insert_into(domain)
                    .values(&domain_msg)
                    .returning(id);

                let sql = debug_query::<Pg, _>(&query).to_string();
                debug!("`{}`", sql);

                query.get_results::<i32>(&self.pool.get().unwrap())
            }
        };

        match inserted {
            Ok(value) => {
                if !value.is_empty() {
                    Ok(Domain {
                        id: *value.get(0).unwrap_or(&0),
                        name: domain_msg.name,
                        url: domain_msg.url,
                        state: domain_msg.state,
                        author: match domain_msg.author {
                            Some(a) => a,
                            _ => 0,
                        },
                        color: match domain_msg.color {
                            Some(_color) => _color,
                            _ => CONFIG.domain_default_color().clone(),
                        },
                    })
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::Other,
                        "Something went wrong. Please reload domain list",
                    ))
                }
            }
            Err(err) => match &err {
                DieselError::DatabaseError(kind, _info) => match kind {
                    DieselErrorKind::UniqueViolation => Err(io::Error::new(
                        io::ErrorKind::Other,
                        "Object with same url is already exists",
                    )),
                    _ => {
                        error!("Domain insert/update error: {}", err);
                        Err(io::Error::new(
                            io::ErrorKind::Other,
                            "Error during update object. Please retry..",
                        ))
                    }
                },
                _ => {
                    error!("Domain insert/update error: {}", err);
                    Err(io::Error::new(
                        io::ErrorKind::Other,
                        "Error during update object. Please retry..",
                    ))
                }
            },
        }
    }
}

#[derive(Debug)]
pub struct DomainList {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub state: Option<DomainState>,
    pub search_string: Option<String>,
}

impl Message for DomainList {
    type Result = io::Result<Vec<Domain>>;
}

impl Handler<DomainList> for DbExecutor {
    type Result = io::Result<Vec<Domain>>;

    fn handle(&mut self, domain_msg: DomainList, _ctx: &mut Self::Context) -> Self::Result {
        let mut query = domain::table.into_boxed();

        if let Some(state_domain) = &domain_msg.state {
            query = query.filter(domain::state.eq(state_domain));
        } else {
            // query = query.filter(domain::state.ne(DomainState::Removed));
        }

        if let Some(domain_search) = domain_msg.search_string {
            query = query.filter(
                domain::name
                    .ilike(format!("%{}%", domain_search))
                    .or(domain::url.ilike(format!("%{}%", domain_search))),
            );
        }

        if let Some(limit) = domain_msg.limit {
            query = query.limit(limit as i64);
        }

        if let Some(offset) = domain_msg.offset {
            query = query.offset(offset as i64);
        }
        query = query.order_by(domain::name.asc());

        let sql = debug_query::<_, _>(&query).to_string();
        debug!("`{}`", sql);

        let query_result = query.load::<Domain>(&self.pool.get().unwrap());

        match query_result {
            Ok(domains_db) => Ok(domains_db),
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

#[derive(
    Associations, Identifiable, Queryable, Debug, DeriveSerialize, DeriveDeserialize, Clone,
)]
#[table_name = "domain_status"]
#[belongs_to(Domain, foreign_key = "domain_id")]
pub struct DomainStatus {
    pub id: i32,
    pub date: NaiveDateTime,
    pub loading_time: i32,
    pub status_code: i32,
    pub headers: String,
    pub filename: Option<String>,
    #[column_name = "domain_id"]
    pub domain: Domain,
}

#[derive(Debug, DeriveSerialize, DeriveDeserialize, Validate)]
pub struct DomainStatusShort {
    pub date: DateTime<Utc>,
    pub loading_time: i32,
    pub status_code: i32,
    pub headers: String,
    pub filename: Option<String>,
}

#[derive(Insertable, DeriveSerialize, DeriveDeserialize, Clone, Debug)]
#[table_name = "domain_status"]
pub struct InsertDomainStatusRequest {
    pub date: NaiveDateTime,
    pub loading_time: i32,
    pub status_code: i32,
    pub headers: String,
    pub filename: Option<String>,
    pub domain_id: i32,
}

impl Message for InsertDomainStatusRequest {
    type Result = io::Result<Vec<Option<String>>>;
}

impl Handler<InsertDomainStatusRequest> for DbExecutor {
    type Result = io::Result<Vec<Option<String>>>;

    fn handle(
        &mut self,
        insert_msg: InsertDomainStatusRequest,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        use crate::db::schema::domain_status::dsl::*;

        let inserted = diesel::insert_into(domain_status)
            .values(&insert_msg)
            .execute(&self.pool.get().unwrap())
            .is_ok();

        if inserted {
            if CONFIG.domain_statuses_rotation() {
                //calculate date of greater self.config.rotate_days
                let dt_rotate =
                    Utc::now() - Duration::days(CONFIG.domain_statuses_rotate_days() as i64);

                let subquery = domain_status
                    .filter(
                        domain_id
                            .eq(&insert_msg.domain_id)
                            .and(date.lt(dt_rotate.naive_utc())),
                    )
                    .order(date.desc())
                    .select((id, filename))
                    .load::<(i32, Option<String>)>(&self.pool.get().unwrap())
                    .unwrap();

                let ids = subquery.iter().map(|el| el.0).collect::<Vec<i32>>();
                diesel::delete(domain_status.filter(id.eq_any(ids)))
                    .execute(&self.pool.get().unwrap())
                    .ok();

                let filenames = subquery
                    .into_iter()
                    .filter(|el| el.1 != None)
                    .map(|el| el.1)
                    .collect::<Vec<Option<String>>>();

                Ok(filenames)
            } else {
                let empty: Vec<Option<String>> = Vec::new();
                Ok(empty)
            }
        } else {
            Err(Error::new(
                ErrorKind::Other,
                "Cannot insert value".to_string(),
            ))
        }
    }
}
