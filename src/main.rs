fn main() {
    #[derive(Debug)]
    enum Cell {
        Text(String),
        Int(i32),
        BigInt(i64),
        Float(f64),
    }

    let mut row = vec![
        Cell::Text(String::from("my text")),
        Cell::BigInt(30945782345087),
    ];

    if true {
        row.push(Cell::Float(10.0));
        row.push(Cell::Int(10));
    }

    for el in &row {
        if let Cell::BigInt(val) = el {
            println!("The element is a big ass integer: {val}!");
        } else {
            println!("The element is something other than a big ass integer!");
        }
    }

    dbg!(row);
}
