#[macro_use]

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn connect() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL IS NOT SET");
    PgConnection::establish(&database_url).expect("UNABLE TO CONNECT TO DATABASE")
}
