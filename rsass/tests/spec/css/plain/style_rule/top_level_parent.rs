//! Tests auto-converted from "sass-spec/spec/css/plain/style_rule/top_level_parent.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("top_level_parent")
        .mock_file("plain.css", "& {a: b}\n")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"plain\";\n"),
        "& {\
         \n  a: b;\
         \n}\n"
    );
}
