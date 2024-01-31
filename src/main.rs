#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::similar_names)]
use std::net::UdpSocket;

use crate::{
    parsers::{header, question},
    types::header::DnsHeader,
};
use anyhow::Result;
use types::{record::DnsRecord, Serializable};

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
                let (_remaining, request_questions) =
                    question::parse(remaining, request_header.qdcount)
                        .map_err(|err| err.to_owned())?;
                let response_header = DnsHeader::builder(request_header.id)
                    .qr(1)
                    .opcode(request_header.opcode)
                    .flags(request_header.flags)
                    .qdcount(request_header.qdcount)
                    .ancount(request_header.qdcount)
                    .rcode(if request_header.opcode == 0 { 0 } else { 4 })
                    .build();
                let response_header = response_header.serialize();
                println!("Request: {:?}", request_questions);
                let response_questions = request_questions.serialize();
                let mut response_records = Vec::with_capacity(request_questions.len());
                for question in request_questions {
                    let response_record = DnsRecord::new(
                        question.name,
                        question.record_type,
                        question.class,
                        60,
                        vec![8, 8, 8, 8],
                    );
                    response_records.push(response_record);
                }
                println!("Response: {:?}", response_records);
                let response_records = response_records.serialize();
                let response = [
                    response_header.as_slice(),
                    response_questions.as_slice(),
                    response_records.as_slice(),
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
