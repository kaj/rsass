//! Tests auto-converted from "sass-spec/spec/css/plain/slash.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("slash")
        .mock_file("plain.css", "a {\n  slash: 1/2/foo/bar;\n}\n")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@import \"plain\";\n"),
        "a {\
         \n  slash: 1/2/foo/bar;\
         \n}\n"
    );
}
