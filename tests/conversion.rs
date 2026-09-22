use std::io::Write;
use std::process::{Command, Stdio};

fn convert(input: &str) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_temperature-converter"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("converter should start");
    child.stdin.take().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn converts_freezing_and_boiling_points() {
    for (input, expected) in [("0\nc\n", "32"), ("100\nc\n", "212"), ("32\nf\n", "0"), ("212\nf\n", "100")] {
        assert!(convert(input).contains(&format!("result: {expected}\n")));
    }
}

#[test]
fn converts_negative_and_decimal_temperatures() {
    for input in ["-40\nc\n", "-40\nf\n"] {
        assert!(convert(input).contains("result: -40\n"));
    }
    assert!(convert("12.5\nc\n").contains("result: 54.5\n"));
}
