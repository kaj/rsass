//! Tests auto-converted from "sass-spec/spec/css/plain/import/sass_takes_precedence.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("sass_takes_precedence")
        .mock_file("other.css", "other {syntax: css}\n")
        .mock_file("other.sass", "other\n  syntax: sass\n")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@import \"other\";\n"),
        "other {\
         \n  syntax: sass;\
         \n}\n"
    );
}
