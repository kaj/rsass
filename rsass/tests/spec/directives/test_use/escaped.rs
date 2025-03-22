//! Tests auto-converted from "sass-spec/spec/directives/use/escaped.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("escaped")
        .mock_file("other.scss", "a {b: c}\n")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@u\\73 e \"other\"\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
