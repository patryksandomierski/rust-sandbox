use log::{info, debug};
use log4rs;

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    debug!("hiho");
    info!("hmmmmm");
    for i in 1..1000 {
        info!(target: "log_file", "it should land in file only: {}", i);
    }
}
