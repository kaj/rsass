//! Tests auto-converted from "sass-spec/spec/css/plain/import/scss_takes_precedence.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("scss_takes_precedence")
        .mock_file("other.css", "other {syntax: css}\n")
        .mock_file("other.scss", "other {syntax: scss}\n")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@import \"other\";\n"),
        "other {\
         \n  syntax: scss;\
         \n}\n"
    );
}
