use std::{thread, time::Duration};

pub mod mtasa;

fn main() {
    mtasa::server::start();
    thread::sleep(Duration::from_secs(5000))
}