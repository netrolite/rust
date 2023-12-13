use std::{thread::sleep, time::Duration};

pub fn main() {
  let duration = Duration::from_secs(1);
  let mut n = 5;

  for _ in 0..100 {
    n += 1;
    println!("{}", n);
    sleep(duration);
  }
}
