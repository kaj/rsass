//! Tests auto-converted from "sass-spec/spec/css/plain/style_rule/nesting/combinator.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("combinator")
        .mock_file("plain.css", "a {+ b {c: d}}\n")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("@use \"plain\";\n"),
        "a {\
         \n  + b {\
         \n    c: d;\
         \n  }\
         \n}\n"
    );
}
