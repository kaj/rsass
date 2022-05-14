//! Tests auto-converted from "sass-spec/spec/values/numbers/very_large.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("very_large")
}

#[test]
fn negative() {
    assert_eq!(
        runner().ok("@use \'sass:math\';\
             \na {b: -(math.pow(10.0, 30))}\n"),
        "a {\
         \n  b: -1000000000000000000000000000000;\
         \n}\n"
    );
}
#[test]
fn positive() {
    assert_eq!(
        runner().ok("@use \'sass:math\';\
             \na {b: math.pow(10.0, 30)}\n"),
        "a {\
         \n  b: 1000000000000000000000000000000;\
         \n}\n"
    );
}
