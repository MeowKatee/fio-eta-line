use std::time::Duration;

use nom::{
    bytes::complete::{tag, take_until, take_while},
    character::complete::{anychar, char, space1},
    combinator::{map, opt},
    multi::{separated_list0, separated_list1},
    sequence::{pair, preceded, terminated, tuple},
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
            _lb,
            progress_percentage,
            _rb,
            (speed, iops),
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
        parse_percentage,
        tag("]"),
        parse_speed_iops,
        tag("[eta "),
        parse_eta_time,
        tag("]"),
    ))(input)?;

    Ok((
        input,
        FioEtaLine {
            jobs_unfinished,
            opened_files,
            rate_limit,
            job_statuses,
            progress_percentage,
            speed,
            iops,
            eta: eta_time,
        },
    ))
}

fn parse_speed_iops(input: &str) -> IResult<&str, (FioSpeed, FioIOPS)> {
    let (input, speed_iops) = take_until("[eta")(input)?;
    if speed_iops.is_empty() {
        Ok((input, (Default::default(), Default::default())))
    } else {
        let (_, (speed, iops)) = tuple((parse_speed, parse_iops))(speed_iops)?;
        Ok((input, (speed, iops)))
    }
}

fn parse_percentage(input: &str) -> IResult<&str, Option<Decimal>> {
    let (input, progress) = terminated(take_until("%"), tag("%"))(input)?;
    Ok((input, parse_decimal(progress).ok().map(|(_, per)| per)))
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

fn parse_rate_limit(input: &str) -> IResult<&str, FioRateLimit> {
    let (input, rate_limit) = opt(preceded(tag(", "), take_until(":")))(input)?;

    let Some(rate_limit) = rate_limit else {
        return Ok((input, FioRateLimit::NoRateLimit));
    };

    if rate_limit.ends_with(" IOPS") {
        let iops_limits = rate_limit.trim_end_matches(" IOPS");
        if let (_, (Some(min), _, Some(max))) = tuple((
            map(take_until("-"), parse_iops_str),
            char('-'),
            map(take_while(|c: char| c.is_ascii_graphic()), parse_iops_str),
        ))(iops_limits)?
        {
            return Ok((input, FioRateLimit::IOPSLimit { min, max }));
        };
    } else {
        let speed_limits = rate_limit;
        if let (_, (Some(min), _, Some(max))) = tuple((
            map(take_until("-"), parse_speed_str),
            char('-'),
            map(take_while(|c: char| c.is_ascii_graphic()), parse_speed_str),
        ))(speed_limits)?
        {
            return Ok((input, FioRateLimit::SpeedLimit { min, max }));
        };
    }

    Ok((input, FioRateLimit::NoRateLimit))
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
fn parse_comma_pair<R>(
    process: fn(&str) -> Option<R>,
) -> impl FnMut(&str) -> IResult<&str, (Option<R>, Option<R>, Option<R>)> {
    move |input| {
        map(
            separated_list1(
                tag(","),
                pair(
                    anychar,
                    map(
                        preceded(
                            char('='),
                            take_while(|c: char| c.is_alphanumeric() || c == '/'),
                        ),
                        process,
                    ),
                ),
            ),
            |values| {
                let mut read = None;
                let mut write = None;
                let mut trim = None;
                for (rw, result) in values {
                    if let Some(result) = result {
                        match rw {
                            'r' => read = Some(result),
                            'w' => write = Some(result),
                            't' => trim = Some(result),
                            _ => (), // ignore unknown type
                        }
                    }
                }
                (read, write, trim)
            },
        )(input)
    }
}

fn parse_iops(input: &str) -> IResult<&str, FioIOPS> {
    let (input, (_lb, (read, write, trim), _space, _iops_rb)) = tuple((
        char('['),
        parse_comma_pair(parse_iops_str),
        space1,
        tag("IOPS]"),
    ))(input)?;

    Ok((input, FioIOPS { read, write, trim }))
}

fn parse_speed(input: &str) -> IResult<&str, FioSpeed> {
    let (input, (_, (read, write, trim), _)) =
        tuple((char('['), parse_comma_pair(parse_speed_str), tag("]")))(input)?;

    Ok((input, FioSpeed { read, write, trim }))
}

mod utils;
use utils::*;
mod types;
pub use types::fold_job_statuses;
pub use types::{FioEtaLine, FioIOPS, FioRateLimit, FioSpeed, JobStatus, JobStatuses};

pub use byte_unit;
pub use rust_decimal;
