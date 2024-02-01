use nom::{
    bytes::complete::{tag, take},
    multi::many_till,
    IResult,
};

use crate::types::record::DnsRecord;

use super::label::parse_label;

pub fn parse(input: &[u8], count: u16) -> IResult<&[u8], Vec<DnsRecord>> {
    let mut remaining = input;
    let mut records = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let (rem, record) = parse_record(remaining)?;
        remaining = rem;
        records.push(record);
    }
    Ok((remaining, records))
}

fn parse_record(input: &[u8]) -> IResult<&[u8], DnsRecord> {
    let (remaining, (labels, _)) = many_till(parse_label, tag([0]))(input)?;
    let (remaining, record_type) = take(2usize)(remaining)?;
    let record_type = u16::from_be_bytes(record_type.try_into().unwrap()); //it's okey to unwrap here since we know that we have 2 bytes
    let (remaining, class) = take(2usize)(remaining)?;
    let class = u16::from_be_bytes(class.try_into().unwrap()); //it's okey to unwrap here since we know that we have 2 bytes
    let (remaining, ttl) = take(4usize)(remaining)?;
    let ttl = u32::from_be_bytes(ttl.try_into().unwrap()); //it's okey to unwrap here since we know that we have 4 bytes
    let (remaining, data_length) = take(2usize)(remaining)?;
    let data_length = u16::from_be_bytes(data_length.try_into().unwrap()); //it's okey to unwrap here since we know that we have 2 bytes
    let (remaining, data) = take(data_length as usize)(remaining)?;
    let record = DnsRecord::new(labels.join("."), record_type, class, ttl, data.to_vec());
    Ok((remaining, record))
}
