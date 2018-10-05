extern crate failure;

use failure::{format_err, Error};

use std::env;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

fn parse_user_host(args: Vec<String>) -> Result<(String, String), Error> {
    let len = args.len();
    let localhost = String::from("localhost:79");

    if len == 1 {
        return Ok((String::from(""), localhost));
    } else if len == 2 {
        let temp = &args[1];
        if temp.contains("@") {
            let parts: Vec<&str> = temp.split('@').collect();
            return Ok((String::from(parts[0]), String::from(parts[1]) + ":79"));
        } else {
            return Ok((temp.clone(), localhost));
        }
    } else {
        return Err(format_err!("[finger-client]: wrong arguments"));
    }
}

fn main() -> Result<(), Error> {
    // get either (), user, or user@host
    let args: Vec<String> = env::args().collect();
    let (user, host) = parse_user_host(args)?;

    println!("User: {}\tHost: {}", user, host);
    let mut stream = TcpStream::connect(&host)?;
    let ipaddr = stream.peer_addr()?;
    // Successfully connected
    println!("Connected to {} @ {}", host, ipaddr);
    println!("Looking up {}", user);
    write!(&mut stream, "{}", user)?;
    write!(&mut stream, "\r\n");
    let mut msg = String::new();
    let _ = stream.read_to_string(&mut msg);
    // Now close connection
    stream.shutdown(Shutdown::Both)?;
    // print out response
    println!("{}", msg);
    return Ok(());
}
