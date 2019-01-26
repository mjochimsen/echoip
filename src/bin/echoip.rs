extern crate clap;
extern crate echoip;

use std::net::Ipv4Addr;
use std::env::args_os;
use std::ffi::OsString;

fn main() {
    let config = Config::parse_args(args_os());

    println!("Hello, echoip! address = {}, port = {}",
             config.address, config.port);
}

#[derive(Clone, PartialEq, Debug)]
struct Config {
    address: Ipv4Addr,
    port: u16,
}

impl Config {

    const DEFAULT_PORT: u16 = 5300;

    fn parse_args<I, T>(args: I) -> Config
        where I: IntoIterator<Item = T>,
              T: Into<OsString> + Clone {
        use clap::{App, crate_version};
        use echoip::config::{arg_address, arg_port};

        let matches = App::new("echoip")
            .about("Gets the public IP address of the caller")
            .version(crate_version!())
            .arg(arg_port("The port number to use on the echo server"))
            .arg(arg_address("The IP address of the echo server")
                 .required(true))
            .get_matches_from(args);

        let port = match matches.value_of("port") {
            Some(val) => val.parse::<u16>().unwrap(),
            None => Config::DEFAULT_PORT,
        };
        let address = matches.value_of("ADDRESS").unwrap()
            .parse::<Ipv4Addr>().unwrap();

        Config { address, port }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_cli() {
        let addr = "1.2.3.4".parse::<Ipv4Addr>().unwrap();

        let args = vec!["echoip", "--port", "12345", "1.2.3.4"];
        let config = Config::parse_args(args);
        assert_eq!(config.port, 12345);
        assert_eq!(config.address, addr);

        let args = vec!["echoip", "1.2.3.4"];
        let config = Config::parse_args(args);
        assert_eq!(config.port, Config::DEFAULT_PORT);
        assert_eq!(config.address, addr);
    }
}
