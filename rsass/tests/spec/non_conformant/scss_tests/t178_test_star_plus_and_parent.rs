//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/178_test_star_plus_and_parent.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("178_test_star_plus_and_parent")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {*+html & {a: b}}\n"),
        "* + html foo {\
         \n  a: b;\
         \n}\n"
    );
}
