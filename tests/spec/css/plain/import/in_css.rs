//! Tests auto-converted from "sass-spec/spec/css/plain/import/in_css.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("in_css")
        .mock_file("plain.css", "@import \"whatever\";\n")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@import \"plain\";\n"),
        "@import \"whatever\";\n"
    );
}
