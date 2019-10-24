use crate::hashers::PBKDF2PasswordHasher;
use serde_derive::{Deserialize, Serialize};
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
