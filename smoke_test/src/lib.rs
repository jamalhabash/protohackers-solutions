use std::error::Error;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::SocketAddr;
use std::net::{Shutdown, TcpListener, TcpStream};

use std::thread;

pub struct Config {
    pub port: u16,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let port: u16 = args[1].clone().parse().unwrap(); //get rid of this unwrap...question mark syntax?, match statement?

        Ok(Config { port })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], config.port))).unwrap(); //fix this

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                //handle_connection(stream);
                thread::spawn(move || handle_connection(stream));
            }
            Err(e) => {
                eprintln!("{}", e) //fix this, should pass up
            }
        }
    }
    Ok(())
}

fn handle_connection(stream: TcpStream) {
    let mut stream = BufReader::new(stream);
    //maybe you can use a while let here instead of a loop
    println!("We have a new connection!");

    loop {
        let mut buf = String::new();
        let mut buf = Vec::new();
        //read line will return ok(0) if it is end of file so use a match statement and end.break
        //stream.get_ref().shutdown(how) probably use this to close the connection on EOF
        match stream.read_until(0xA, &mut buf) {
            Ok(0) => {
                println!("EOL reached");

                stream
                    .get_ref()
                    .shutdown(Shutdown::Both)
                    .expect("shutdown call failed");
                break;
            }
            Err(e) => {
                println!("There was an error {}", e);
                break;
            }
            _ => (),
        }
        // if stream.read_line(&mut buf).is_err() {
        //     break; //check to see if connection is every terminated
        // }
        println!("{:?}", buf);
        stream.get_ref().write(&buf).unwrap(); //TODO
    }

    println!("We are out of loop!")
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn pass() {
//         assert!(true);
//     }
// }
