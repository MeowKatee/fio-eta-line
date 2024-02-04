use fio_eta_line::parse_eta_line;

#[test]
fn no_speed_iops() {
    let result = parse_eta_line("Jobs: 1 (f=1): [W(1)][5.7%][eta 00m:30s]");
    assert!(result.is_ok())
}
