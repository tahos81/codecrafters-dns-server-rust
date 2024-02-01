use nom::IResult;

use crate::types::packet::DnsPacket;

use super::{header, question, record};

pub fn parse(input: &[u8]) -> IResult<&[u8], DnsPacket> {
    let (remaining, header) = header::parse(input)?;
    let (remaining, questions) = question::parse(remaining, header.qdcount)?;
    let (remaining, answers) = record::parse(remaining, header.ancount)?;
    let (remaining, nameservers) = record::parse(remaining, header.nscount)?;
    let (remaining, additional) = record::parse(remaining, header.arcount)?;
    let packet = DnsPacket::new(header, questions, answers, nameservers, additional);
    Ok((remaining, packet))
}
