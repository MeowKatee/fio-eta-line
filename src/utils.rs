use nom::{bytes::complete::take_while, character::complete::digit1, combinator::map_res, IResult};
use rust_decimal::Decimal;

pub fn parse_iops_str(iops: &str) -> Option<u64> {
    iops.parse::<byte_unit::Byte>().ok().map(|b| b.as_u64())
}

pub fn parse_speed_str(speed: &str) -> Option<byte_unit::Byte> {
    speed.trim_end_matches("/s").parse::<byte_unit::Byte>().ok()
}

pub fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse::<u32>)(input)
}

pub fn parse_decimal(input: &str) -> IResult<&str, Decimal> {
    map_res(
        take_while(|ch: char| ch == '.' || ch.is_ascii_digit()),
        str::parse::<Decimal>,
    )(input)
}
