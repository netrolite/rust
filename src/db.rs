use crate::{
    error::{self, MyError, MyErrorReason},
    models,
};
use rand::Rng;

enum ConnStatus {
    Connected,
    Disconnected,
}

pub struct Db {
    conn_status: ConnStatus,
}

impl Db {
    pub fn new(conn_str: &str) -> Db {
        if conn_str.is_empty() {
            panic!("Could not connect to the base with the data");
        }
        Db {
            conn_status: ConnStatus::Connected,
        }
    }

    pub fn get_user(&self, username: &str) -> Result<models::User, error::MyError> {
        let mut rng = rand::rng();

        if rng.random_bool(0.5) {
            models::User::new(username, "12345678")
        } else {
            Err(MyError {
                reason: MyErrorReason::UrStupid,
            })
        }
    }
}
