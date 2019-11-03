use crate::db::DbExecutor;
use crate::hashers::PBKDF2PasswordHasher;
use actix::{Handler, Message};
use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};
use std::io;
use std::io::Error;
use std::ptr::hash;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    #[serde(skip_serializing)]
    password: String,
    pub name: String,
}

impl User {
    pub fn set_password(&mut self, raw_password: &str) -> bool {
        let hasher: PBKDF2PasswordHasher = PBKDF2PasswordHasher::new();
        match hasher.encode(raw_password) {
            Ok(hashed) => {
                self.password = hashed;
                true
            }
            Err(_) => false,
        }
    }
    pub fn check_password(&self, raw_password: &str) -> bool {
        let hasher: PBKDF2PasswordHasher = PBKDF2PasswordHasher::new();
        match hasher.verify(raw_password, &self.password) {
            Ok(true) => true,
            Err(e) => {
                log::error!("Error: `{}` checking password for user_id {}", e, self.id);
                false
            }
            _ => false,
        }
    }
}

pub struct FindUser {
    pub email: String,
}

impl Message for FindUser {
    type Result = io::Result<User>;
}

impl Handler<FindUser> for DbExecutor {
    type Result = io::Result<User>;

    fn handle(&mut self, actor_user: FindUser, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::users::dsl::*;
        log::info!("Get user from email {}", &actor_user.email);

        match users
            .filter(email.eq(&actor_user.email))
            .limit(2)
            .load::<User>(&self.pool.get().unwrap())
        {
            Ok(mut items) => Ok(items.pop().unwrap()),
            Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Database error")),
        }
    }
}