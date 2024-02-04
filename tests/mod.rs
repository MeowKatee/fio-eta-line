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
