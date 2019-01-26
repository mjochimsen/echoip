use clap::Arg;
use std::net::Ipv4Addr;

pub fn arg_address(help: &str) -> Arg {
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

pub fn arg_port(help: &str) -> Arg {
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
}
