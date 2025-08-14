use std::collections::HashMap;

fn main() {
    let mut db = HashMap::new();

    let blue_team_init_score = 0;
    let red_team_init_score = 10;
    let blue = String::from("blue");
    let red = String::from("red");

    db.insert(&blue, blue_team_init_score);
    db.insert(&red, red_team_init_score);

    let db_response = match db.get(&red) {
        Some(val) => format!("Found value: {val}"),
        None => String::from("Found nothin'"),
    };
    println!("db response: {db_response:?}");

    println!("Iterating over HashMap before update:");
    for (key, val) in &db {
        println!("{key}: {val}");
    }

    let prev_red = *db.get(&red).unwrap_or(&0);
    let prev_blue = *db.get(&blue).unwrap_or(&0);
    db.insert(&red, prev_red + 10);
    db.insert(&red, prev_blue + 10);

    println!("Iterating over HashMap after update:");
    for (key, val) in &db {
        println!("{key}: {val}");
    }

    let key1 = String::from("red");
    let key2 = String::from("sdf");

    let inserted_value1 = db.entry(&key1).or_insert(70);
    println!("first insertion result: {inserted_value1}");

    let inserted_value2 = db.entry(&key2).or_insert(70);
    println!("second insertion result: {inserted_value2}");

    println!("db: {db:?}");
}
