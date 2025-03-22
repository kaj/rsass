//! Tests auto-converted from "sass-spec/spec/directives/forward/escaped.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("escaped")
        .mock_file("other.scss", "a {b: c}\n")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@fo\\72ward \"other\"\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
