use std::time::Duration;

use nom::{
    bytes::complete::{tag, take_until, take_while},
    character::complete::{anychar, char, digit1, space1},
    combinator::{map, map_res, opt},
    multi::{separated_list0, separated_list1},
    sequence::{pair, preceded, tuple},
    IResult,
};
use rust_decimal::Decimal;

pub fn parse_eta_line(input: &str) -> IResult<&str, FioEtaLine> {
    let (
        input,
        (
            _jobs,
            jobs_unfinished,
            _f,
            opened_files,
            _right_bracket,
            rate_limit,
            _semicon,
            job_statuses,
            _,
            progress_percentage,
            _,
            (read_speed, write_speed, trim_speed),
            (read_iops, write_iops, trim_iops),
            _eta,
            eta_time,
            _send,
        ),
    ) = tuple((
        tag("Jobs: "),
        parse_u32,
        tag(" (f="),
        parse_u32,
        tag(")"),
        parse_rate_limit,
        tag(": "),
        parse_job_statuses,
        tag("["),
        parse_decimal,
        tag("%]"),
        parse_speed,
        parse_iops,
        tag("[eta "),
        parse_eta_time,
        tag("]"),
    ))(input)?;

    Ok((
        input,
        FioEtaLine {
            jobs_unfinished,
            opened_files,
            rate_limit: rate_limit.map(|s| s.to_owned()),
            job_statuses,
            progress_percentage,
            read_speed,
            write_speed,
            trim_speed,
            read_iops,
            write_iops,
            trim_iops,
            eta: eta_time,
        },
    ))
}

fn parse_eta_time(input: &str) -> IResult<&str, Duration> {
    let (input, eta_time) = separated_list1(tag(":"), tuple((parse_u32, anychar)))(input)?;
    let (eta_secs, unit) = eta_time
        .into_iter()
        .reduce(|(time, unit), (next_time, next_unit)| {
            (
                time * if unit == 'd' { 24 } else { 60 } + next_time,
                next_unit,
            )
        })
        .unwrap();
    debug_assert!(unit == 's');
    Ok((input, Duration::from_secs(eta_secs as _)))
}

fn parse_rate_limit(input: &str) -> IResult<&str, Option<&str>> {
    let (input, rate_limit) = opt(preceded(tag(", "), take_until(":")))(input)?;

    Ok((input, rate_limit))
}

fn parse_job_status(input: &str) -> IResult<&str, (JobStatus, u32)> {
    let (input, (status_char, _, count, _)) =
        tuple((anychar, tag("("), parse_u32, tag(")")))(input)?;

    let status = status_char.into();

    Ok((input, (status, count)))
}

fn parse_job_statuses(input: &str) -> IResult<&str, JobStatuses> {
    let (input, (_, statuses, _)) = tuple((
        tag("["),
        separated_list0(char(','), parse_job_status),
        tag("]"),
    ))(input)?;
    Ok((input, fold_job_statuses(statuses)))
}

// returns (read, write, trim)
fn parse_comma_pair(
    input: &str,
) -> IResult<&str, (Option<String>, Option<String>, Option<String>)> {
    map(
        separated_list1(
            tag(","),
            pair(
                anychar,
                preceded(
                    char('='),
                    take_while(|c: char| c.is_alphanumeric() || c == '/'),
                ),
            ),
        ),
        |values: Vec<(char, &str)>| {
            let mut read = None;
            let mut write = None;
            let mut trim = None;
            for (rw, speed) in values {
                match rw {
                    'r' => read = Some(speed.to_string()),
                    'w' => write = Some(speed.to_string()),
                    't' => trim = Some(speed.to_string()),
                    _ => (), // ignore unknown type
                }
            }
            (read, write, trim)
        },
    )(input)
}

fn parse_iops(input: &str) -> IResult<&str, (Option<String>, Option<String>, Option<String>)> {
    let (input, (_, iops_content, _, _)) =
        tuple((char('['), parse_comma_pair, space1, tag("IOPS]")))(input)?;

    Ok((input, iops_content))
}

fn parse_speed(input: &str) -> IResult<&str, (Option<String>, Option<String>, Option<String>)> {
    let (input, (_, speed_content, _)) = tuple((char('['), parse_comma_pair, tag("]")))(input)?;

    Ok((input, speed_content))
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse::<u32>)(input)
}

fn parse_decimal(input: &str) -> IResult<&str, Decimal> {
    map_res(
        take_while(|ch: char| ch == '.' || ch.is_ascii_digit()),
        str::parse::<Decimal>,
    )(input)
}

mod types;
pub use types::fold_job_statuses;
pub use types::{FioEtaLine, JobStatus, JobStatuses};
