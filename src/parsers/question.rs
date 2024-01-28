use nom::{
    bytes::complete::{tag, take},
    multi::many_till,
    IResult,
};

use crate::types::question::DnsQuestion;

pub fn parse_dns_question(input: &[u8]) -> IResult<&[u8], DnsQuestion> {
    let (remaining, (labels, _)) = many_till(parse_label, tag([0]))(input)?;
    let (remaining, record_type) = take(2usize)(remaining)?;
    let record_type = u16::from_be_bytes(record_type.try_into().unwrap()); //it's okey to unwrap here since we know that we have 2 bytes
    let (remaining, class) = take(2usize)(remaining)?;
    let class = u16::from_be_bytes(class.try_into().unwrap()); //it's okey to unwrap here since we know that we have 2 bytes
    let question = DnsQuestion::new(labels.join("."), record_type, class);
    Ok((remaining, question))
}

fn parse_label(input: &[u8]) -> IResult<&[u8], String> {
    let (remaining, length) = take(1usize)(input)?;
    let (remaining, label) = take(length[0])(remaining)?;
    Ok((remaining, String::from_utf8_lossy(label).to_string()))
}