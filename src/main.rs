#![allow(unused)]
use std::net::{UdpSocket};
use inquire::{Text};
use std::io::{Write, ErrorKind, Read, BufReader, prelude::*};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut stderr = StandardStream::stderr(ColorChoice::Always);

    println!("Welcome to SimpleTFTPD");
    println!("Settings:");

    

    let address = Text::new("Bind address")
    .with_default("0.0.0.0")
    .prompt()
    .expect("Error during bind address prompt");

    let port = Text::new("Port number")
    .with_default("69")
    .prompt()
    .expect("Error during bind port prompt");

    let cwd = std::env::current_dir().expect("Error when getting current directory");

    let directory = Text::new("Working directory")
    .with_default(cwd.to_str().unwrap())
    .prompt()
    .expect("Error during directory prompt");

    let bind_address = format!("{}:{}", address, port);

    let listener = UdpSocket::bind(&bind_address);

    if let Err(e) = &listener {
        stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red)));
        writeln!(stderr, "Error binding to address {}", &bind_address);
        writeln!(stderr, "Error message: {}", e);

        if e.kind() == ErrorKind::PermissionDenied {
            stderr.set_color(ColorSpec::new().set_fg(Some(Color::Blue)));
            writeln!(&mut stderr, "Have you tried running this program as an admin user?");
        }

        
        return;
    }

    println!("Starting TFTP server on {}", bind_address);

    let socket = listener.unwrap();

    let mut buf = [0; 516];
    let (amt, src) = socket.recv_from(&mut buf).unwrap();
    
   

}