#[cfg(test)]
mod tests {
    use std::time::Duration;

    use fio_eta_line::{parse_eta_line, FioEtaLine, FioIOPS, FioSpeed, JobStatuses};
    use rust_decimal::Decimal;

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
                        read: Some("7354MiB/s".into()),
                        ..Default::default()
                    },
                    iops: FioIOPS {
                        read: Some("1883k".into()),
                        ..Default::default()
                    },
                    eta: Duration::from_secs(25),
                    ..Default::default()
                }
            ))
        )
    }

    #[test]
    fn rate_limit() {
        let result = parse_eta_line(
            "Jobs: 1 (f=1), 0B/s-2048KiB/s: [M(1)][11.7%][r=1025KiB/s,w=1025KiB/s][r=256,w=256 IOPS][eta 00m:53s]"
        );

        assert!(result.is_ok())
    }

    #[test]
    fn bad_progress() {
        let result = parse_eta_line(
            "Jobs: 1 (f=1): [M(1)][-.-%][r=1025KiB/s,w=1025KiB/s][r=256,w=256 IOPS][eta 00m:53s]",
        );

        assert_eq!(result.unwrap().1.progress_percentage, None)
    }

    #[test]
    fn no_speed_iops() {
        let result = parse_eta_line("Jobs: 1 (f=1): [W(1)][5.7%][eta 00m:30s]");
        assert!(result.is_ok())
    }

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
                    read: Some("1025KiB/s".into()),
                    write: Some("1025KiB/s".into()),
                    ..Default::default()
                },
                iops: FioIOPS {
                    read: Some("256".into()),
                    write: Some("256".into()),
                    ..Default::default()
                },
                eta: Duration::from_secs(secs),
                ..Default::default()
            }
        )
    }
}
