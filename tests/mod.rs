use std::time::Duration;

use fio_eta_line::byte_unit::Byte;
use fio_eta_line::rust_decimal::Decimal;
use fio_eta_line::{parse_eta_line, FioEtaLine, FioIOPS, FioSpeed, JobStatuses};

#[test]
fn seq_read() {
    let result = parse_eta_line(
        "Jobs: 94 (f=94): [R(6),M(29),M(31)][37.5%][r=7354MiB/s][r=1883k IOPS][eta 00m:25s]",
    );
    assert_eq!(
        result,
        Ok((
            "",
            FioEtaLine {
                jobs_unfinished: 94,
                opened_files: 94,
                job_statuses: JobStatuses {
                    seq_read: 6,
                    mixed_seq_reads_writes: 60,
                    ..Default::default()
                },
                progress_percentage: Decimal::from_str_exact("37.5").ok(),
                speed: FioSpeed {
                    read: Byte::MEBIBYTE.multiply(7354),
                    ..Default::default()
                },
                iops: FioIOPS {
                    read: Some(1883_000),
                    ..Default::default()
                },
                eta: Duration::from_secs(25),
                ..Default::default()
            }
        ))
    )
}

#[test]
fn seq_read_write() {
    let result = parse_eta_line(
        "Jobs: 1 (f=1): [M(1)][80.0%][r=785MiB/s,w=781MiB/s][r=201k,w=200k IOPS][eta 00m:01s]",
    );
    assert_eq!(
        result,
        Ok((
            "",
            FioEtaLine {
                jobs_unfinished: 1,
                opened_files: 1,
                job_statuses: JobStatuses {
                    mixed_seq_reads_writes: 1,
                    ..Default::default()
                },
                progress_percentage: Decimal::from_str_exact("80.0").ok(),
                speed: FioSpeed {
                    read: Byte::MEBIBYTE.multiply(785),
                    write: Byte::MEBIBYTE.multiply(781),
                    ..Default::default()
                },
                iops: FioIOPS {
                    read: Some(201 * 1000),
                    write: Some(200 * 1000),
                    ..Default::default()
                },
                eta: Duration::from_secs(1),
                ..Default::default()
            }
        ))
    )
}
