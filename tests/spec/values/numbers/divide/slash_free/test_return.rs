//! Tests auto-converted from "sass-spec/spec/values/numbers/divide/slash_free/return.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn built_in() {
    assert_eq!(
        runner().ok("a {b: nth(3 1/2 4, 2)}\n"),
        "a {\
         \n  b: 0.5;\
         \n}\n"
    );
}
#[test]
fn user_defined() {
    assert_eq!(
        runner().ok("@function a() {@return 1/2}\n\
             \nb {c: a()}\n"),
        "b {\
         \n  c: 0.5;\
         \n}\n"
    );
}
