extern crate dotenv;
extern crate mysql;

use dotenv::dotenv;
use mysql::{prelude::Queryable, Pool, Row, Value};
use std::env;

fn main() {
    dotenv().ok(); // .env ファイルをロード

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let opts = mysql::Opts::from_url(&database_url).unwrap();
    let pool = Pool::new(opts).unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Usage: cargo run <table_name> <column1> [<column2>...]");
    }
    let table_name = &args[1];
    let columns: Vec<&str> = args[2..].iter().map(AsRef::as_ref).collect();

    generate_insert_statements(pool, table_name, columns);
}

fn generate_insert_statements(pool: Pool, table_name: &str, columns: Vec<&str>) {
    let mut conn = pool.get_conn().unwrap();

    let column_names = columns.join(", ");
    let select_query = format!("SELECT {} FROM {}", column_names, table_name);

    // SELECT文を実行して結果を取得します。
    let result = conn
        .query_map(&select_query, |row: Row| {
            let values: Vec<Value> = row.unwrap();
            values
                .iter()
                .map(|value| value_to_string(value))
                .collect::<Vec<String>>()
        })
        .unwrap();

    for row_values in result {
        // 取得したデータを使ってINSERT文を作成します。
        let value_str = row_values.join(", ");
        let insert_statement = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table_name, column_names, value_str
        );

        println!("{}", insert_statement);
    }
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::NULL => "NULL".to_string(),
        Value::Bytes(bytes) => String::from_utf8_lossy(bytes).into_owned(),
        Value::Int(num) => num.to_string(),
        Value::UInt(num) => num.to_string(),
        Value::Float(num) => num.to_string(),
        Value::Date(year, month, day, hour, minute, second, micros) => format!(
            "'{}-{}-{} {}:{}:{}.{}'",
            year, month, day, hour, minute, second, micros
        ),
        _ => "".to_string(), // その他の型は空文字列に変換
    }
}
