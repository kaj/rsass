//! Tests auto-converted from "sass-spec/spec/values/numbers/modulo/zeros.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("zeros")
}

#[test]
fn negative_negative() {
    assert_eq!(
        runner().ok("a {\
             \n  b: -0 % -1;\
             \n}\n"),
        "a {\
         \n  b: 0;\
         \n}\n"
    );
}
#[test]
fn negative_positive() {
    assert_eq!(
        runner().ok("a {\
             \n  b: -0 % 1;\
             \n}\n"),
        "a {\
         \n  b: 0;\
         \n}\n"
    );
}
#[test]
fn positive_negative() {
    assert_eq!(
        runner().ok("a {\
             \n  b: +0 % -1;\
             \n}"),
        "a {\
         \n  b: 0;\
         \n}\n"
    );
}
#[test]
fn positive_positive() {
    assert_eq!(
        runner().ok("a {\
             \n  b: +0 % +1;\
             \n}\n"),
        "a {\
         \n  b: 0;\
         \n}\n"
    );
}
#[test]
fn zero_divider() {
    assert_eq!(
        runner().ok("a {\
             \n  b: inspect(1 % 0);\
             \n}\n"),
        "a {\
         \n  b: NaN;\
         \n}\n"
    );
}
