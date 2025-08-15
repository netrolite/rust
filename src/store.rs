use crate::re;
use crate::user::User;
use std::collections::HashMap;
use std::io;

#[derive(Debug)]
pub struct Store {
    user_count: u32,
    users: HashMap<String, User>,
}

pub enum AddUserResult {
    Success(User),
    AlreadyExists(User),
}

impl Store {
    pub fn new() -> Store {
        Store {
            user_count: 0,
            users: HashMap::new(),
        }
    }

    pub fn add_user(&mut self) -> AddUserResult {
        println!("You chose to add a user.");
        let username = Store::get_username();
        let password = Store::get_password();
        let key = username.clone();
        let new_user = User {
            username: username.clone(),
            password: password.clone(),
        };
        let value = User {
            username: username.clone(),
            password,
        };

        match self.users.insert(key, value) {
            Some(existing_user) => AddUserResult::AlreadyExists(existing_user),
            None => AddUserResult::Success(new_user),
        }
    }

    pub fn find_user(&self) -> Option<&User> {
        println!("You chose to find a user.");
        let username = Store::get_username();
        self.users.get(&username)
    }

    pub fn delete_user(&mut self) -> Option<User> {
        println!("You chose to delete a user.");
        let username = Store::get_username();
        self.users.remove(&username)
    }

    pub fn user_count(&self) -> usize {
        self.users.len()
    }

    fn get_username() -> String {
        loop {
            println!("Enter username:");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_string();
            if !re::username().is_match(&input) {
                println!(
                    "Usernames can only contain uppercase and lowercase english characters and underscores"
                );
            } else {
                break input;
            }
        }
    }

    fn get_password() -> String {
        loop {
            println!("Enter password:");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_string();
            if !(10..=256).contains(&input.len()) {
                println!("Password must be at least 10 and at most 256 characters long");
            } else {
                break input;
            }
        }
    }
}
