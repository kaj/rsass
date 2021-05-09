//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/star_plus_and_parent.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
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
