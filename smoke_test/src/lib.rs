use std::error::Error;
use std::io::{BufRead, BufReader, Write};
use std::net::SocketAddr;
use std::net::{TcpListener, TcpStream};

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

    loop {
        let mut buf: Vec<u8> = Vec::new();

        // Read bytes from 'stream' and store in 'buf'
        match stream.read_until(0xA, &mut buf) {
            Ok(number_bytes_read) => {
                if number_bytes_read == 0 {
                    // End-of-file (EOF) has been reached.
                    // We break the loop, allowing 'stream' to be dropped, which will close the connection.
                    println!("EOL reached, shutting down connection.");
                    break;
                } else {
                    println!("{} number of bytes read.", number_bytes_read);
                }
            }
            Err(e) => {
                eprintln!("An error occured while reading bytes: {e}");
                break;
            }
        }

        // Write contents of 'buf' to 'stream'
        match stream.get_ref().write(&buf) {
            Ok(number_bytes_written) => {
                println!("{} number of bytes written.", number_bytes_written)
            }
            Err(e) => {
                eprintln!("An error occured while writing bytes: {e}");
                break;
            }
        }
    }
}
