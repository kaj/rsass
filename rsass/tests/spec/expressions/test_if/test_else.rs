//! Tests auto-converted from "sass-spec/spec/expressions/if/else.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("else")
}

#[test]
#[ignore] // wrong result
fn t1() {
    assert_eq!(
        runner().ok("a {b: if(else: c)}\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn t2() {
    assert_eq!(
        runner().ok("a {b: if(else: c; else: d)}\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
