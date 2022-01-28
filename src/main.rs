use std::cmp::Ordering;
use std::io;

use log::{info, warn};
use log4rs;
use rand::Rng;

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    let secret_num = rand::thread_rng().gen_range(1..101);
    info!("guess game!");
    info!("input your guess:");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read line!");
        let guess: u32 = match guess.trim().parse() { // parse return enum Result so match works
            Ok(n) => n,
            Err(_) => {
                warn!("NaN! type valid number:");
                continue;
            }
        };

        info!("your type is: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => info!("too small. try again:"),
            Ordering::Equal => {
                info!("bulls eye!");
                break;
            }
            Ordering::Greater => info!("too big. try again:")
        }
    }
}
