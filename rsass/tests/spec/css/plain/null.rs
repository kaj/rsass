//! Tests auto-converted from "sass-spec/spec/css/plain/null.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("null")
        .mock_file("plain.css", "a {\n  x: null;\n}\n")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"plain\";\n"),
        "a {\
         \n  x: null;\
         \n}\n"
    );
}
