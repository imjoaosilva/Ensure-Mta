use std::{thread, time::Duration};

pub mod mtasa;
pub mod utils;

fn main() {
    mtasa::server::start();
    thread::sleep(Duration::from_secs(5000))
}