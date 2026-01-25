//! Tests auto-converted from "sass-spec/spec/css/plain/style_rule/nesting/one_level.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("one_level")
        .mock_file("plain.css", "a {b {c: d}}\n")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("@use \"plain\";\n"),
        "a {\
         \n  b {\
         \n    c: d;\
         \n  }\
         \n}\n"
    );
}
