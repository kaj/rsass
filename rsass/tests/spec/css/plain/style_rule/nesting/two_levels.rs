//! Tests auto-converted from "sass-spec/spec/css/plain/style_rule/nesting/two_levels.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("two_levels")
        .mock_file("plain.css", "a {b {c {d: e}}}\n")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("@use \"plain\";\n"),
        "a {\
         \n  b {\
         \n    c {\
         \n      d: e;\
         \n    }\
         \n  }\
         \n}\n"
    );
}
