use nom::{
    bytes::complete::{tag, take},
    multi::many_till,
    IResult,
};

use crate::types::question::DnsQuestion;

use super::label::parse_label;

pub fn parse(input: &[u8], qdcount: u16) -> IResult<&[u8], Vec<DnsQuestion>> {
    let mut remaining = input;
    let mut questions = Vec::with_capacity(qdcount as usize);
    for _ in 0..qdcount {
        let (rem, question) = parse_question(remaining)?;
        remaining = rem;
        questions.push(question);
    }
    Ok((remaining, questions))
}

fn parse_question(input: &[u8]) -> IResult<&[u8], DnsQuestion> {
    let (remaining, (labels, _)) = many_till(parse_label, tag([0]))(input)?;
    let (remaining, record_type) = take(2usize)(remaining)?;
    let record_type = u16::from_be_bytes(record_type.try_into().unwrap()); //it's okey to unwrap here since we know that we have 2 bytes
    let (remaining, class) = take(2usize)(remaining)?;
    let class = u16::from_be_bytes(class.try_into().unwrap()); //it's okey to unwrap here since we know that we have 2 bytes
    let question = DnsQuestion::new(labels.join("."), record_type, class);
    Ok((remaining, question))
}
