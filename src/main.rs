use std::net::UdpSocket;

use crate::{
    parser::parse_dns_header,
    types::header::{DnsHeader, DnsHeaderStruct},
};
use anyhow::Result;

mod parser;
mod types;

fn main() -> Result<()> {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");

    loop {
        let mut buf = [0; 512];
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let (_, dns_header) = parse_dns_header(&buf).map_err(|err| err.to_owned())?;
                let response = DnsHeaderStruct::builder(dns_header.id).qr(1).build();
                udp_socket
                    .send_to(&DnsHeader::from(response), source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }

    Ok(())
}
