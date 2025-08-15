use std::error::Error;
mod auth;
mod db;
mod error;
mod models;

fn main() -> Result<(), Box<dyn Error>> {
    let db = db::Db::new("connection_string");
    match db.get_user("@lets_get_fucking") {
        Ok(query_result) => println!("Query result: {query_result:?}"),
        Err(err) => {
            println!("Query failed. Error:");
            eprintln!("{err:?}");
        }
    }

    Ok(())
}
