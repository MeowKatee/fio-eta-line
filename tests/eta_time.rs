use std::time::Duration;

use byte_unit::Byte;
use fio_eta_line::{parse_eta_line, FioEtaLine, FioIOPS, FioSpeed, JobStatuses};
use rust_decimal::Decimal;

#[test]
fn long_time() {
    let result = parse_eta_line("Jobs: 1 (f=1): [M(1)][0.0%][r=1025KiB/s,w=1025KiB/s][r=256,w=256 IOPS][eta 83d:07h:59m:55s]");

    let Ok(result) = result else {
        panic!("cannot parse long time example!");
    };

    let day = 83;
    let hours = day * 24 + 7;
    let minutes = hours * 60 + 59;
    let secs = minutes * 60 + 55;

    assert_eq!(
        result.1,
        FioEtaLine {
            jobs_unfinished: 1,
            opened_files: 1,
            job_statuses: JobStatuses {
                mixed_seq_reads_writes: 1,
                ..Default::default()
            },
            progress_percentage: Decimal::from_str_exact("0.0").ok(),

            speed: FioSpeed {
                read: Byte::KIBIBYTE.multiply(1025),
                write: Byte::KIBIBYTE.multiply(1025),
                ..Default::default()
            },
            iops: FioIOPS {
                read: Some(256),
                write: Some(256),
                ..Default::default()
            },
            eta: Duration::from_secs(secs),
            ..Default::default()
        }
    )
}
