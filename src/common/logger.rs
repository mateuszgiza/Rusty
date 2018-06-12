use chrono::prelude::*;

pub fn info(text: &str) {
    let now = Local::now();
    println!("[{}] > {}", now.format("%T"), text);
}
