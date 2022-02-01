use log::{info};
use log4rs;

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    let x = 3;
    info!("let me add five to x={}: {}",x, add_five(x));

    let be_five = true;
    let number = if be_five { 5 } else { 4 };
    info!("are we five: {} then we are: {}", be_five, number);
    counting_loop_break();
}

fn add_five(x: i32) -> i32 {
    x + 5 // instead return, use statement without `;`
}

fn counting_loop_break() {
    let mut count = 0;
    'counting_up: loop { //awesome :D
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}
