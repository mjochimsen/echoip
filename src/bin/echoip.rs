extern crate echoip;

use std::env::args_os;
use echoip::config::ClientConfig;
use echoip::client::client;

fn main() {
    let config = ClientConfig::parse_args(args_os());
    client(config.address, config.port);
}
