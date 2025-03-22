//! Tests auto-converted from "sass-spec/spec/values/numbers/divide/slash_free/return.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("return")
}

#[test]
fn built_in() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \na {b: list.nth(3 1/2 4, 2)}\n"),
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
