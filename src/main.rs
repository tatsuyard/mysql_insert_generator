extern crate dotenv;
extern crate mysql;

use dotenv::dotenv;
use mysql::{prelude::Queryable, Pool, Row};
use std::env;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let opts = mysql::Opts::from_url(&database_url).unwrap();
    let pool = Pool::new(opts).unwrap();

    let mut conn = pool.get_conn().unwrap();

    let result = conn
        .query_map("SELECT id, name, email FROM users", |row: Row| {
            let (id, name, email): (u32, String, String) = mysql::from_row(row);
            (id, name, email)
        })
        .unwrap();

    for (id, name, email) in result {
        let insert_statement = format!(
            "INSERT INTO users (id, name, email) VALUES ({}, '{}', '{}')",
            id, name, email
        );

        println!("{}", insert_statement);
    }
}
