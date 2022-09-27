use std::error::Error;
use std::io::{BufRead, BufReader, Write};
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

        let port: u16 = match args[1].clone().parse() {
            Ok(number) => number,
            Err(_) => Err("port must be a value between 0-65535")?,
        };

        Ok(Config { port })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], config.port)))?;
    //should I also add a timeout for the read?
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("We have a new connection!");
                thread::spawn(move || handle_connection(stream));
            }
            Err(e) => eprintln!("{e}"),
        }
    }
    Ok(())
}

fn handle_connection(stream: TcpStream) {
    let mut stream = BufReader::new(stream);
    //maybe you can use a while let here instead of a loop

    loop {
        let mut buf: Vec<u8> = Vec::new();

        match stream.read_until(0xA, &mut buf) {
            Ok(0) => {
                println!("EOL reached, shutting down connection.");
                stream
                    .get_ref()
                    .shutdown(Shutdown::Both)
                    .expect("shutdown call failed"); //do I need to do something other than except here, will this shutdown the entire program? this should be a recoverable error
                                                     //I think except is ok here, the thread will panic and shutdown.
                break;
            }
            Err(e) => {
                eprintln!("An error occured while reading bytes: {e}");
                break;
            }
            _ => {
                //this is ok(number of bytes read, so say how many are read)
                stream.get_ref().write(&buf).unwrap();
                //then say how many bytes are written here.
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn pass() {
//         assert!(true);
//     }
// }
