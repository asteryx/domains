use actix::{Actor, Addr, SyncArbiter, SyncContext};
//use diesel::prelude::{PgConnection, SqliteConnection};
use diesel::prelude::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Error, Pool};
use dotenv;
use num_cpus;

//pub struct ConnDsl(pub Pool<ConnectionManager<PgConnection>>);
pub struct ConnDsl(pub Pool<ConnectionManager<SqliteConnection>>);

impl Actor for ConnDsl {
    type Context = SyncContext<Self>;
}

pub fn init() -> () {
    //    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    //    let manager = ConnectionManager::<PgConnection>::new(db_url);
    //    let manager = ConnectionManager::<SqliteConnection>::new("data.sqlite");
    //    let conn = Pool::builder()
    //        .max_size(5)
    //        .build(manager)
    //        .expect("Failed to create pool.");
}
