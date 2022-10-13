use std::net::{UdpSocket, SocketAddr};

#[derive(PartialEq, Clone, Copy)]
enum PacketType {
    RRQ = 1,
    WRQ = 2,
    Data = 3,
    ACK = 4,
    Error = 5,
    Invalid
}
impl From<u16> for PacketType {
    fn from(i: u16) -> Self {
        return match i {
        1 => PacketType::RRQ,
        2 => PacketType::WRQ,
        3 => PacketType::Data,
        4 => PacketType::ACK,
        5 => PacketType::Error,
        _ => PacketType::Invalid
        }
    }
}

enum ErrorType {
    NotDefined = 0,
    NotFound = 1,
    AccessViolation = 2,
    DiskFull = 3,
    IllegalOperation = 4,
    UnknownTID = 5,
    AlreadyExists = 6,
    NoSuchUser = 7,
    Invalid
}

impl From<u16> for ErrorType {
    fn from(e: u16) -> Self {
        match e {
            0 => ErrorType::NotDefined,
            1 => ErrorType::NotFound,
            2 => ErrorType::AccessViolation,
            3 => ErrorType::DiskFull,
            4 => ErrorType::IllegalOperation,
            5 => ErrorType::UnknownTID,
            6 => ErrorType::AlreadyExists,
            7 => ErrorType::NoSuchUser,
            _ => ErrorType::Invalid
        }
    }
}

struct Packet {
    packet_type: PacketType,
    error_type: Option<ErrorType>,

    src_tid: u16,
    dst_tid: u16,
    incoming: bool,
    raw: Vec<u8>,

    block: Option<u16>,
    filename: Option<Vec<u8>>,
    data: Option<Vec<u8>>
}

impl Packet {
    fn new_incoming(raw: Vec<u8>, sender: &SocketAddr) -> Packet {
        let packet_type = PacketType::from(u16::from_be_bytes(raw[..2].try_into().unwrap()));
        let error: Option<ErrorType> = None;
        let block: Option<u16> = None;

        match packet_type {
            PacketType::Error => {
                let error = 
                Some(
                    ErrorType::from(
                        u16::from_le_bytes(raw[2..4].try_into().unwrap())
                    )
                );
            },
            PacketType::ACK => {
                let mut block: Option<u16> = 
                    Some(u16::from_be_bytes(raw[2..4].try_into().unwrap()));
            }
            _ => ()
        }



        Packet {
            packet_type: packet_type,
            error_type: error,
            block: block,
            filename: None,
            data: None,
            src_tid: sender.port(),
            dst_tid: 69,
            incoming: true,
            raw: raw
        }
    }

    fn new_outgoing(
        packet_type: PacketType,
        error_type: Option<ErrorType>,
        src_tid: u16,
        dst_tid: u16,
        block: Option<u16>,
        data: Option<Vec<u8>>,
        filename: String) -> Packet {

        match packet_type {
            PacketType::ACK => {
                Packet {
                    packet_type: packet_type,
                    error_type: None,
                    src_tid: src_tid,
                    dst_tid: dst_tid,
                    incoming: false,
                    raw: Vec::new(),
                    block: block,
                    data: data,
                    filename: Some(Vec::from(filename.as_bytes()))
                }
            },
            PacketType::Data => {
                Packet {
                    packet_type: packet_type,
                    error_type: None,
                    src_tid: src_tid,
                    dst_tid: dst_tid,
                    incoming: false,
                    raw: Vec::new(),
                    block: block,
                    data: data,
                    filename: None
                }
            },

            PacketType::Error => {
                Packet {
                    packet_type: packet_type,
                    error_type: error_type,
                    src_tid: src_tid,
                    dst_tid: dst_tid,
                    incoming: false,
                    raw: Vec::new(),
                    block: None,
                    filename: None,
                    data: None 
                }
            },

            _ => {
                panic!("Invalid packet type given");
            }
        }
    }
}