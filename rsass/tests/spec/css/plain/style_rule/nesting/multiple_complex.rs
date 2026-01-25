//! Tests auto-converted from "sass-spec/spec/css/plain/style_rule/nesting/multiple_complex.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("multiple_complex")
        .mock_file("plain.css", "a, b {c, d {e: f}}\n")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("@use \"plain\";\n"),
        "a, b {\
         \n  c, d {\
         \n    e: f;\
         \n  }\
         \n}\n"
    );
}
