extern crate clap;

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
        use clap::{App, Arg};

        let matches = App::new("echoip")
            .about("Gets the public IP address of the caller")
            .version(clap::crate_version!())
            .arg(Arg::with_name("port")
                 .short("p")
                 .long("port")
                 .value_name("PORT")
                 .help("The port number to use on the echo server")
                 .takes_value(true)
                 .validator(Config::valid_port))
            .arg(Arg::with_name("ADDRESS")
                 .help("The IP address of the echo server")
                 .required(true)
                 .validator(Config::valid_address))
            .get_matches_from(args);

        let port = match matches.value_of("port") {
            Some(val) => val.parse::<u16>().unwrap(),
            None => Config::DEFAULT_PORT,
        };
        let address = matches.value_of("ADDRESS").unwrap()
            .parse::<Ipv4Addr>().unwrap();

        Config { address, port }
    }

    fn valid_port(val: String) -> Result<(), String> {
        match val.parse::<u16>() {
            Ok(_) => Ok(()),
            Err(_) => Err("Not a valid port number".to_string()),
        }
    }

    fn valid_address(val: String) -> Result<(), String> {
        match val.parse::<Ipv4Addr>() {
            Ok(_) => Ok(()),
            Err(_) => Err("Not a valid IP address".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn addr() -> Ipv4Addr {
        "1.2.3.4".parse::<Ipv4Addr>().unwrap()
    }

    #[test]
    fn parse_address() {
        let args = vec!["echoip", "1.2.3.4"];
        let config = Config::parse_args(args);
        assert_eq!(config.port, Config::DEFAULT_PORT);
        assert_eq!(config.address, addr());
    }

    #[test]
    fn parse_port() {
        let args = vec!["echoip", "--port", "12345", "1.2.3.4"];
        let config = Config::parse_args(args);
        assert_eq!(config.port, 12345);
        assert_eq!(config.address, addr());
    }

    #[test]
    fn parse_help() {
    }

    #[test]
    fn parse_version() {
    }
}
