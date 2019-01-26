extern crate echoip;

use std::env::args_os;
use echoip::config::ServerConfig;

fn main() {
    let config = ServerConfig::parse_args(args_os());

    println!("Hello, echoipd! address = {}, port = {}",
             config.address, config.port);
}
