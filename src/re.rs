use regex::Regex;

pub fn username() -> Regex {
    Regex::new(r"[a-zA-Z0-9_]").unwrap()
}
