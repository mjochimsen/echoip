use std::net::Ipv4Addr;
use std::ffi::OsString;

use clap::{App, Arg, crate_version};

fn arg_address(help: &str) -> Arg {
    Arg::with_name("ADDRESS")
        .validator(valid_address)
        .help(help)
}

fn valid_address(val: String) -> Result<(), String> {
    match val.parse::<Ipv4Addr>() {
        Ok(_) => Ok(()),
        Err(_) => Err("Not a valid IP address".to_string()),
    }
}

fn arg_port(help: &str) -> Arg {
    Arg::with_name("port")
        .short("p")
        .long("port")
        .takes_value(true)
        .value_name("PORT")
        .validator(valid_port)
        .help(help)
}

fn valid_port(val: String) -> Result<(), String> {
    match val.parse::<u16>() {
        Ok(_) => Ok(()),
        Err(_) => Err("Not a valid port number".to_string()),
    }
}

const DEFAULT_PORT: u16 = 5300;

#[derive(Clone, PartialEq, Debug)]
pub struct ClientConfig {
    pub address: Ipv4Addr,
    pub port: u16,
}

impl ClientConfig {
    pub fn parse_args<I, T>(args: I) -> ClientConfig
        where I: IntoIterator<Item = T>,
              T: Into<OsString> + Clone {

        let matches = App::new("echoip")
            .about("Gets the public IP address of the caller")
            .version(crate_version!())
            .arg(arg_address("The IP address of the echo server")
                 .required(true))
            .arg(arg_port("The port number to use on the echo server"))
            .get_matches_from(args);

        let address = matches.value_of("ADDRESS").unwrap()
            .parse::<Ipv4Addr>().unwrap();
        let port = match matches.value_of("port") {
            Some(val) => val.parse::<u16>().unwrap(),
            None => DEFAULT_PORT,
        };

        ClientConfig { address, port }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ServerConfig {
    pub address: Ipv4Addr,
    pub port: u16,
}

impl ServerConfig {
    pub fn parse_args<I, T>(args: I) -> ServerConfig
        where I: IntoIterator<Item = T>,
              T: Into<OsString> + Clone {

        let matches = App::new("echoipd")
            .about("Returns the public IP address of any connection")
            .version(crate_version!())
            .arg(arg_address("The IP address to listen on"))
            .arg(arg_port("The port number to listen on"))
            .get_matches_from(args);

        let address = match matches.value_of("ADDRESS") {
            Some(val) => val.parse::<Ipv4Addr>().unwrap(),
            None => Ipv4Addr::UNSPECIFIED,
        };
        let port = match matches.value_of("port") {
            Some(val) => val.parse::<u16>().unwrap(),
            None => DEFAULT_PORT,
        };

        ServerConfig { address, port }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{App, ErrorKind};

    #[test]
    fn parse_valid_address() {
        let tester = App::new("test_addr").arg(arg_address("test"));
        let args = vec!["test_addr", "1.2.3.4"];
        let result = tester.get_matches_from_safe(args);
        assert!(result.is_ok());
        let matches = result.unwrap();
        let address = matches.value_of("ADDRESS");
        assert!(address.is_some());
        assert_eq!(address.unwrap(), "1.2.3.4");
    }

    #[test]
    fn parse_missing_address() {
        let tester = App::new("test_addr").arg(arg_address("test"));
        let args = vec!["test_addr"];
        let result = tester.get_matches_from_safe(args);
        assert!(result.is_ok());
        let matches = result.unwrap();
        let address = matches.value_of("ADDRESS");
        assert!(address.is_none());
    }

    #[test]
    fn parse_invalid_address() {
        let tester = App::new("test_addr").arg(arg_address("test"));
        let args = vec!["test_addr", "abcd"];
        let result = tester.get_matches_from_safe(args);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.kind, ErrorKind::ValueValidation);
    }

    #[test]
    fn parse_out_of_range_address() {
        let tester = App::new("test_addr").arg(arg_address("test"));
        let args = vec!["test_addr", "127.0.0.256"];
        let result = tester.get_matches_from_safe(args);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.kind, ErrorKind::ValueValidation);
    }

    #[test]
    fn help_text_for_address() {
        let tester = App::new("test_addr")
            .arg(arg_address("test_message"));
        let args = vec!["test_addr", "--help"];
        let result = tester.get_matches_from_safe(args);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.kind, ErrorKind::HelpDisplayed);
        assert!(error.message.contains("test_message"));
    }

    #[test]
    fn parse_valid_port() {
        let tester = App::new("test_port").arg(arg_port("test"));
        let args = vec!["test_port", "--port", "12345"];
        let result = tester.get_matches_from_safe(args);
        assert!(result.is_ok());
        let matches = result.unwrap();
        let port = matches.value_of("port");
        assert!(port.is_some());
        assert_eq!(port.unwrap(), "12345");
    }

    #[test]
    fn parse_missing_port_value() {
        let tester = App::new("test_port").arg(arg_port("test"));
        let args = vec!["test_port", "--port"];
        let result = tester.get_matches_from_safe(args);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.kind, ErrorKind::EmptyValue);
    }

    #[test]
    fn parse_invalid_port() {
        let tester = App::new("test_port").arg(arg_port("test"));
        let args = vec!["test_port", "--port", "abcd"];
        let result = tester.get_matches_from_safe(args);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.kind, ErrorKind::ValueValidation);
    }

    #[test]
    fn parse_out_of_range_port() {
        let tester = App::new("test_port").arg(arg_port("test"));
        let args = vec!["test_port", "--port", "65536"];
        let result = tester.get_matches_from_safe(args);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.kind, ErrorKind::ValueValidation);
    }

    #[test]
    fn help_text_for_port() {
        let tester = App::new("test_port").arg(arg_port("test_message"));
        let args = vec!["test_port", "--help"];
        let result = tester.get_matches_from_safe(args);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.kind, ErrorKind::HelpDisplayed);
        assert!(error.message.contains("test_message"));
    }

    #[test]
    fn parse_client_cli() {
        let args = vec!["echoip", "127.0.0.1"];
        let config = ClientConfig::parse_args(args);
        assert_eq!(config.address, Ipv4Addr::LOCALHOST);
        assert_eq!(config.port, DEFAULT_PORT);
    }

    #[test]
    fn parse_client_cli_with_port() {
        let args = vec!["echoip", "--port", "12345", "127.0.0.1"];
        let config = ClientConfig::parse_args(args);
        assert_eq!(config.address, Ipv4Addr::LOCALHOST);
        assert_eq!(config.port, 12345);
    }

    #[test]
    fn parse_server_cli() {
        let args = vec!["echoipd"];
        let config = ServerConfig::parse_args(args);
        assert_eq!(config.address, Ipv4Addr::UNSPECIFIED);
        assert_eq!(config.port, DEFAULT_PORT);
    }

    #[test]
    fn parse_server_cli_with_addr() {
        let args = vec!["echoipd", "127.0.0.1"];
        let config = ServerConfig::parse_args(args);
        assert_eq!(config.address, Ipv4Addr::LOCALHOST);
        assert_eq!(config.port, DEFAULT_PORT);
    }

    #[test]
    fn parse_server_cli_with_port() {
        let args = vec!["echoipd", "--port", "12345"];
        let config = ServerConfig::parse_args(args);
        assert_eq!(config.address, Ipv4Addr::UNSPECIFIED);
        assert_eq!(config.port, 12345);
    }
}
