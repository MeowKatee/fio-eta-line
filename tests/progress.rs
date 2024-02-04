use fio_eta_line::parse_eta_line;
use rust_decimal::Decimal;

#[test]
fn progress() {
    let result = parse_eta_line(
        "Jobs: 1 (f=1): [M(1)][8.7%][r=1025KiB/s,w=1025KiB/s][r=256,w=256 IOPS][eta 00m:53s]",
    );

    assert_eq!(
        result.unwrap().1.progress_percentage,
        Decimal::from_str_exact("8.7").ok()
    )
}

#[test]
fn bad_progress() {
    let result = parse_eta_line(
        "Jobs: 1 (f=1): [M(1)][-.-%][r=1025KiB/s,w=1025KiB/s][r=256,w=256 IOPS][eta 00m:53s]",
    );

    assert_eq!(result.unwrap().1.progress_percentage, None)
}
