use prisma_query::{ast::*, visitor::*};
use rusqlite::{Connection, NO_PARAMS};

const IN_SIZE: u16 = 10000; // Change me to test the limits

fn main() {
    let conn = Connection::open_in_memory().unwrap();

    conn.execute(
        "CREATE TABLE User (id INTEGER PRIMARY KEY, name TEXT NOT NULL)",
        NO_PARAMS,
    )
    .unwrap();

    conn.execute("INSERT INTO User (name) VALUES ('Alice')", NO_PARAMS)
        .unwrap();

    let selection: Vec<i32> = (0..IN_SIZE).map(|_| 1).collect();

    let select = Select::from_table("User").so_that("id".in_selection(selection));
    let (query, params) = Sqlite::build(select);
    let mut stmt = conn.prepare_cached(dbg!(&query)).unwrap();

    let name = stmt
        .query_map(&params, |row| {
            let name: String = row.get(1);
            name
        })
        .unwrap()
        .map(|res| res.unwrap())
        .next()
        .unwrap();

    println!("Hello, {}!", name);
}
