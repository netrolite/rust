use crate::models;
use crate::{
    db,
    error::{MyError, MyErrorReason},
};

pub fn sign_in(username: &str, password: &str) -> Result<models::User, MyError> {
    let user = db::Db::new("connection string").get_user(username)?;
    if check_password(&user, password) {
        Ok(user)
    } else {
        Err(MyError {
            reason: MyErrorReason::UrStupid,
        })
    }
}

fn check_password(user: &models::User, password: &str) -> bool {
    user.password == password
}
