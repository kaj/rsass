use rsass::{compile_scss, compile_value, output};

const FORMAT: output::Format = output::Format {
    style: output::Style::Compressed,
    precision: 5,
};

#[test]
fn bad_escape() {
    assert!(compile_scss(b"\\d00000", FORMAT).is_err());
}

#[test]
fn decimal_integer_overflow() {
    assert_eq!(
        compile_value(b"2000000000000000000000000000000000000", FORMAT)
            .unwrap(),
        b"2000000000000000000000000000000000000".to_vec(),
    );
}

#[test]
fn decimal_fraction_overflow() {
    assert_eq!(
        compile_value(b"0.2000000000000000000000000000000000000", FORMAT)
            .unwrap(),
        b".2",
    );
}
