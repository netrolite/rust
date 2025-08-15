#[derive(Debug)]
pub struct MyError {
    pub reason: MyErrorReason,
}

#[derive(Debug)]
pub enum MyErrorReason {
    UrStupid,
    UrBlack,
}
