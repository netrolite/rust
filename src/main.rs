/*
Store:
fields:
users
users_count

functions:
new

methods:
create_user(username, password)
delete_user(username)
find_user(username)
find_users(username[])

----------------------

User struct:
username
password
*/

const MESSAGES: [&str; 3] = [
    "LMAO YOU FUCKING RETART WROOOOOOOOOOOOONG!!!!",
    "BETTER FUCKING LUCK NEXT FUCKING TIME AHAHAHHAHA",
    "YOU SUCK FUCKING ASS IDIOT",
];

use rand::Rng;
use std::io;

fn main() -> Result<(), io::Error> {
    let mut rng = rand::rng();

    loop {
        let secret_number = rng.random_range(0..=10);
        let number = loop {
            println!("Enter a god damn number!!!!!!!!!!!!!!: ");
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            match input.trim().parse::<i32>() {
                Ok(val) => break val,
                Err(err) => eprintln!("{err}"),
            };
        };

        if number == secret_number {
            break;
        } else {
            let random_message = MESSAGES[rng.random_range(0..MESSAGES.len())];
            println!("{random_message}");
        }
    }
    println!("You guessed correctly! Now FUCK OFF!!!!!!!!!!!!!");

    Ok(())
}
