use std::cmp::Ordering;
use std::io;

use log::{info, warn};
use log4rs;
use rand::Rng;

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    let x = 3;
    info!("let me add five to x={}: {}",x, add_five(x))
}

fn add_five(x: i32) -> i32 {
    x + 5 // instead return, use statement without `;`
}