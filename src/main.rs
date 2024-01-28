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
use types::record::{DnsRecord, DnsRecordStruct};

mod parsers;
mod types;

fn main() -> Result<()> {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");

    loop {
        let mut buf = [0; 512];
        match udp_socket.recv_from(&mut buf) {
            Ok((_size, source)) => {
                let (remaining, request_header) =
                    header::parse(&buf).map_err(|err| err.to_owned())?;
                let (_remaining, request_question) =
                    question::parse(remaining).map_err(|err| err.to_owned())?;
                let response_header = DnsHeaderStruct::builder(request_header.id)
                    .qr(1)
                    .qdcount(request_header.qdcount)
                    .ancount(request_header.qdcount)
                    .build();
                let response_header = DnsHeader::from(response_header);
                let response_question = DnsQuestion::from(request_question.clone());
                let response_record = DnsRecordStruct::new(
                    request_question.name,
                    request_question.record_type,
                    request_question.class,
                    60,
                    vec![8, 8, 8, 8],
                );
                let response_record = DnsRecord::from(response_record);
                let response = [
                    response_header.as_slice(),
                    response_question.as_slice(),
                    response_record.as_slice(),
                ]
                .concat();
                udp_socket
                    .send_to(response.as_slice(), source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {e}");
                break;
            }
        }
    }

    Ok(())
}
