//! Tests auto-converted from "sass-spec/spec/directives/import/escaped.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("escaped")
        .mock_file("other.scss", "a {b: c}\n")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@impor\\74 \"other\"\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
