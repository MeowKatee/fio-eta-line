use byte_unit::Byte;
use fio_eta_line::{parse_eta_line, FioRateLimit};

#[test]
fn rate_limit_speed() {
    let result = parse_eta_line(
            "Jobs: 1 (f=1), 0B/s-2048KiB/s: [M(1)][11.7%][r=1025KiB/s,w=1025KiB/s][r=256,w=256 IOPS][eta 00m:53s]"
        );

    assert_eq!(
        result.unwrap().1.rate_limit,
        FioRateLimit::SpeedLimit {
            min: Byte::MIN,
            max: Byte::KIBIBYTE.multiply(2048).unwrap()
        }
    )
}

#[test]
fn rate_limit_iops() {
    let result = parse_eta_line(
            "Jobs: 1 (f=1), 59-600 IOPS: [M(1)][11.7%][r=1025KiB/s,w=1025KiB/s][r=256,w=256 IOPS][eta 00m:53s]"
        );

    assert_eq!(
        result.unwrap().1.rate_limit,
        FioRateLimit::IOPSLimit { min: 59, max: 600 }
    );
}
