#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::similar_names)]
use std::net::UdpSocket;

use crate::{
    parsers::{header, question},
    types::{
        header::{DnsHeader, DnsHeaderStruct},
        question::DnsQuestion,
    },
};
use anyhow::Result;

mod parsers;
mod types;

fn main() -> Result<()> {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");

    loop {
        let mut buf = [0; 512];
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let (remaining, request_header) =
                    header::parse(&buf).map_err(|err| err.to_owned())?;
                let (_remaining, request_question) =
                    question::parse(remaining).map_err(|err| err.to_owned())?;
                let response_header = DnsHeaderStruct::builder(request_header.id)
                    .qr(1)
                    .qdcount(request_header.qdcount)
                    .build();
                let response_header = DnsHeader::from(response_header);
                let response_question = DnsQuestion::from(request_question);
                let response = [response_header.as_slice(), response_question.as_slice()].concat();
                udp_socket
                    .send_to(response.as_slice(), source)
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
