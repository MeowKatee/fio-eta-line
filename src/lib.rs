use std::time::Duration;

use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{anychar, char, digit1, space1},
    combinator::{map, map_res},
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
            num_jobs,
            _f,
            num_files,
            _,
            job_statuses,
            _,
            percentage,
            _,
            (read_speed, write_speed),
            (read_iops, write_iops),
            _eta,
            eta_min,
            _m,
            eta_sec,
            _send,
        ),
    ) = tuple((
        tag("Jobs: "),
        parse_u32,
        tag(" (f="),
        parse_u32,
        tag("): "),
        parse_job_statuses,
        tag("["),
        parse_decimal,
        tag("%]"),
        parse_speed,
        parse_iops,
        tag("[eta "),
        parse_u32,
        tag("m:"),
        parse_u32,
        tag("s]"),
    ))(input)?;

    Ok((
        input,
        FioEtaLine {
            jobs_unfinished: num_jobs,
            opened_files: num_files,
            job_statuses,
            progress_percentage: percentage,
            read_speed,
            write_speed,
            read_iops,
            write_iops,
            eta: Duration::from_secs((eta_min * 60 + eta_sec) as _),
        },
    ))
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

fn parse_rw_pair(input: &str) -> IResult<&str, (Option<String>, Option<String>)> {
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
            for (rw, speed) in values {
                match rw {
                    'r' => read = Some(speed.to_string()),
                    'w' => write = Some(speed.to_string()),
                    _ => (), // ignore unknown type
                }
            }
            (read, write)
        },
    )(input)
}

fn parse_iops(input: &str) -> IResult<&str, (Option<String>, Option<String>)> {
    let (input, (_, iops_content, _, _)) =
        tuple((char('['), parse_rw_pair, space1, tag("IOPS]")))(input)?;

    Ok((input, iops_content))
}

fn parse_speed(input: &str) -> IResult<&str, (Option<String>, Option<String>)> {
    let (input, (_, speed_content, _)) = tuple((char('['), parse_rw_pair, tag("]")))(input)?;

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
