use crate::error::{MyError, MyErrorReason};

#[derive(Debug)]
pub struct User {
    username: String,
    pub password: String,
    age: u8,
}

impl User {
    pub fn new(username: &str, password: &str) -> Result<User, MyError> {
        if password.len() < 8 {
            return Err(MyError {
                reason: MyErrorReason::UrStupid,
            });
        }
        if username.contains("nigger") {
            return Err(MyError {
                reason: MyErrorReason::UrBlack,
            });
        }
        Ok(User {
            username: username.to_string(),
            password: password.to_string(),
            age: 1,
        })
    }
}
