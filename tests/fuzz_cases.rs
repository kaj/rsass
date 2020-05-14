use rsass::{compile_scss, output};

#[test]
fn bad_escape() {
    let format = output::Format {
        style: output::Style::Compressed,
        precision: 5,
    };
    assert!(compile_scss(b"\\d00000", format).is_err());
}
