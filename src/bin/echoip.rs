extern crate clap;

use std::net::Ipv4Addr;
use clap::{App, Arg, crate_version};

const DEFAULT_PORT: u16 = 5300;

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

fn main() {
    let matches = App::new("echoip")
        .about("Gets the public IP address of the caller")
        .version(crate_version!())
        .arg(Arg::with_name("port")
             .short("p")
             .long("port")
             .value_name("PORT")
             .help("The port number to use when contacting the server")
             .takes_value(true)
             .validator(valid_port))
        .arg(Arg::with_name("ADDRESS")
             .help("The IP address of the echo server")
             .required(true)
             .validator(valid_address))
        .get_matches();

    let port = match matches.value_of("port") {
        Some(val) => val.parse::<u16>().unwrap(),
        None => DEFAULT_PORT,
    };
    let address = matches.value_of("ADDRESS").unwrap()
        .parse::<Ipv4Addr>().unwrap();

    println!("Hello, echoip! port = {}, address = {}", port, address);
}
