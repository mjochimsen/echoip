extern crate echoip;

use std::env::args_os;
use echoip::config::ClientConfig;

fn main() {
    let config = ClientConfig::parse_args(args_os());

    println!("Hello, echoip! address = {}, port = {}",
             config.address, config.port);
}
