#![allow(unused)]
use std::{net::{UdpSocket}, fmt::Display};
use inquire::{Text};
use std::io::{Write, ErrorKind, Read, BufReader, prelude::*};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(PartialEq, Clone, Copy)]
enum OPCode {
    RRQ = 1,
    WRQ = 2,
    DATA = 3,
    ACK = 4,
    ERR = 5,
    INVALID
}

impl From<u16> for OPCode {
    fn from(i: u16) -> Self {
        return match i {
        1 => OPCode::RRQ,
        2 => OPCode::WRQ,
        3 => OPCode::DATA,
        4 => OPCode::ACK,
        5 => OPCode::ERR,
        _ => OPCode::INVALID
        }
    }
}

impl Display for OPCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", match self {
            OPCode::RRQ => "Read Request",
            OPCode::WRQ => "Write Request",
            OPCode::DATA => "Data",
            OPCode::ACK => "Acknowledge",
            OPCode::ERR => "Error",
            _ => "Invalid OPCode"
        });
    }
}

#[derive(PartialEq, Clone, Copy)]
enum TransferMode {
    NETASCII,
    OCTET,
    MAIL,
    INVALID
}

impl From<&str> for TransferMode {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "netascii" => TransferMode::NETASCII,
            "octet" => TransferMode::OCTET,
            "mail" => TransferMode::MAIL,
            _ => TransferMode::INVALID
        }
    }
}



fn main() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut stderr = StandardStream::stderr(ColorChoice::Always);

    println!("Welcome to SimpleTFTPD");
    println!("Settings:");

    
    #[cfg(debug_assertions)]
    let address = "0.0.0.0";

    #[cfg(not(debug_assertions))]
    let address = Text::new("Bind address")
    .with_default("0.0.0.0")
    .prompt()
    .expect("Error during bind address prompt");

    #[cfg(debug_assertions)]
    let port = "1233";
    
    #[cfg(not(debug_assertions))]
    let port = Text::new("Port number")
    .with_default("69")
    .prompt()
    .expect("Error during bind port prompt");

    let cwd = std::env::current_dir().expect("Error when getting current directory");
    #[cfg(debug_assertions)]
    let directory = cwd.to_str().unwrap();

    #[cfg(not_debug_assertions)]
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
    loop {
        let mut buf = [0; 516];
        let (amt, src) = socket.recv_from(&mut buf).unwrap();

        let opcode: u16 = u16::from_be_bytes(buf[..2].try_into().unwrap());

        let op = OPCode::from(opcode);

        println!("Received {} packet from {}", op, src);
        println!("TID: {}", src.port());


        if op == OPCode::RRQ {        
            let mut filename_buf: Vec<u8> = Vec::new();

            for byte in buf[2..amt].bytes() {
                let byte = byte.unwrap();
                if byte == 0 {break}

                filename_buf.push(byte);
            }
            
            let filename = String::from_utf8(filename_buf.clone()).unwrap();

            let mut mode_buf: Vec<u8> = Vec::new();
            for byte in buf[filename_buf.len()+3..amt].bytes() {
                let byte = byte.unwrap();
                if byte == 0 {break}

                mode_buf.push(byte);
            }
            let mode_str = String::from_utf8(mode_buf).unwrap();

            let mode = TransferMode::from(mode_str.as_str());

            println!("Provided filename: {}", filename);  
            println!("Transfer mode: {}", mode_str);
        }

        if op == OPCode::DATA {
            
        }
    }
}