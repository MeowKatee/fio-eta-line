#![feature(assert_matches)]

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use fio_eta_line::{parse_eta_line, FioEtaLine, JobStatuses};
    use rust_decimal::Decimal;

    #[test]
    fn parse_line() {
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
                    }, // skipped
                    progress_percentage: Decimal::from_str_exact("37.5").unwrap(),
                    read_speed: Some("7354MiB/s".to_string()),
                    write_speed: None,
                    read_iops: Some("1883k".to_string()),
                    write_iops: None,
                    eta: Duration::from_secs(25)
                }
            ))
        )
    }
}
