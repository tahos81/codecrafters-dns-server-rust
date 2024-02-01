#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::similar_names)]
use std::net::UdpSocket;

use crate::{parsers::packet, types::header::DnsHeader};
use anyhow::Result;
use types::{packet::DnsPacket, record::DnsRecord, Serializable};

mod parsers;
mod types;

fn main() -> Result<()> {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");

    loop {
        let mut buf = [0; 512];
        match udp_socket.recv_from(&mut buf) {
            Ok((_size, source)) => {
                let (_, request_packet) = packet::parse(&buf).map_err(|err| err.to_owned())?;

                let response_packet = prepare_response(request_packet)?;
                udp_socket
                    .send_to(&response_packet.serialize(), source)
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

fn prepare_response(packet: DnsPacket) -> Result<DnsPacket> {
    let response_header = prepare_response_header(&packet.header);
    let response_records = prepare_response_answers(&packet)?;
    let response_questions = packet.questions;
    Ok(DnsPacket {
        header: response_header,
        questions: response_questions,
        answers: response_records,
        authorities: Vec::new(),
        additionals: Vec::new(),
    })
}

fn prepare_response_header(request_header: &DnsHeader) -> DnsHeader {
    DnsHeader::builder(request_header.id)
        .qr(1)
        .opcode(request_header.opcode)
        .flags(request_header.flags)
        .qdcount(request_header.qdcount)
        .ancount(request_header.qdcount)
        .rcode(if request_header.opcode == 0 { 0 } else { 4 })
        .build()
}

fn prepare_response_answers(request_packet: &DnsPacket) -> Result<Vec<DnsRecord>> {
    let resolver_addr = std::env::args().nth(2).expect("Missing resolver address");
    let resolver_socket = UdpSocket::bind("127.0.0.1:8000").expect("Failed to bind to resolver");
    let request_header = &request_packet.header;
    let request_questions = &request_packet.questions;
    let mut response_records = Vec::with_capacity(request_questions.len());
    let query_header = DnsHeader::builder(request_header.id)
        .qdcount(1)
        .qr(0)
        .opcode(request_header.opcode)
        .flags(request_header.flags)
        .build();
    for question in request_questions {
        let query = [
            query_header.serialize().as_slice(),
            question.serialize().as_slice(),
        ]
        .concat();
        resolver_socket.send_to(query.as_slice(), resolver_addr.as_str())?;
        let mut buf = [0; 512];
        resolver_socket.recv_from(&mut buf)?;
        let (_, response_packet) = packet::parse(&buf).map_err(|err| err.to_owned())?;
        response_records.extend(response_packet.answers);
    }
    Ok(response_records)
}
