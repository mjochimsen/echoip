extern crate echoip;

use std::env::args_os;
use echoip::config::ServerConfig;
use echoip::server::server;

fn main() {
    let config = ServerConfig::parse_args(args_os());
    server(config.address, config.port);
}
