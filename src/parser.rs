use nom::{
    bits::{bits, complete::take},
    error::Error,
    sequence::tuple,
    IResult,
};

use crate::types::header::DnsHeaderStruct;

pub fn parse_dns_header(input: &[u8]) -> IResult<&[u8], DnsHeaderStruct> {
    let (remaining, (id, qr, opcode, flags, z, rcode, qdcount, ancount, nscount, arcount)) =
        parse(input)?;
    Ok((
        remaining,
        DnsHeaderStruct::builder(id)
            .qr(qr)
            .opcode(opcode)
            .flags(flags)
            .z(z)
            .rcode(rcode)
            .qdcount(qdcount)
            .ancount(ancount)
            .nscount(nscount)
            .arcount(arcount)
            .build(),
    ))
}

fn parse(input: &[u8]) -> IResult<&[u8], (u16, u8, u8, u8, u8, u8, u16, u16, u16, u16)> {
    bits::<_, _, Error<(&[u8], usize)>, _, _>(tuple((
        take(16usize),
        take(1usize),
        take(4usize),
        take(4usize),
        take(3usize),
        take(4usize),
        take(16usize),
        take(16usize),
        take(16usize),
        take(16usize),
    )))(input)
}
