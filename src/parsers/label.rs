use nom::{bytes::complete::take, IResult};

pub fn parse_label(input: &[u8]) -> IResult<&[u8], String> {
    let (remaining, length) = take(1usize)(input)?;
    let (remaining, label) = take(length[0])(remaining)?;
    Ok((remaining, String::from_utf8_lossy(label).to_string()))
}
