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

use std::io;

use crate::store::{AddUserResult, Store};
mod re;
mod store;
mod user;

fn main() -> Result<(), io::Error> {
    let mut store = Store::new();

    loop {
        println!("Type a number to select an action:");
        println!("1) Add user");
        println!("2) Find user");
        println!("3) Delete user");
        println!("4) Print user count");
        println!("5) Exit program");

        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        match number.as_str().trim() {
            "1" => match store.add_user() {
                AddUserResult::Success(user) => {
                    println!("User successfully added:");
                    println!("{user:?}");
                }
                AddUserResult::AlreadyExists(user) => {
                    println!("User with username {} already exists.", user.username);
                    println!("Please choose a different username or delete the existing user.");
                }
            },
            "2" => {
                let user = store.find_user();
                match user {
                    Some(user) => {
                        println!("Here's your user:");
                        println!("{user:?}");
                    }
                    None => println!("User not found"),
                }
            }
            "3" => {
                let deleted_user = store.delete_user();
                match deleted_user {
                    Some(user) => {
                        println!("Successfully deleted user with username {}", user.username)
                    }
                    None => println!("User not found"),
                }
            }
            "4" => println!("User count: {}", store.user_count()),
            "5" => break,
            _ => {
                println!("Invalid action number. Try again");
                continue;
            }
        }
    }

    dbg!(store);
    Ok(())
}
