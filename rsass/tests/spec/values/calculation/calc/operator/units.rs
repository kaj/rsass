//! Tests auto-converted from "sass-spec/spec/values/calculation/calc/operator/units.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("units")
}

#[test]
fn denominators() {
    assert_eq!(
        runner().ok("a {b: calc(1/2px + 1/4px) * 1px}\n"),
        "a {\
         \n  b: 0.75;\
         \n}\n"
    );
}
#[test]
fn division() {
    assert_eq!(
        runner().ok("a {b: calc(1px / 2px)}\n"),
        "a {\
         \n  b: 0.5;\
         \n}\n"
    );
}
#[test]
fn multiplication() {
    assert_eq!(
        runner().ok("@use \"sass:math\";\
             \na {b: math.div(calc(2px * 3px), 4px)}\n"),
        "a {\
         \n  b: 1.5px;\
         \n}\n"
    );
}
mod percent {
    use super::runner;

    #[test]
    fn and_known() {
        assert_eq!(
            runner().ok("a {b: calc(1% + 1px)}\n"),
            "a {\
         \n  b: calc(1% + 1px);\
         \n}\n"
        );
    }
    #[test]
    fn and_unknown() {
        assert_eq!(
            runner().ok("a {b: calc(1% + 1unknown)}\n"),
            "a {\
         \n  b: calc(1% + 1unknown);\
         \n}\n"
        );
    }
}
mod unknown {
    use super::runner;

    #[test]
    fn and_known() {
        assert_eq!(
            runner().ok("a {b: calc(1unknown + 1px)}\n"),
            "a {\
         \n  b: calc(1unknown + 1px);\
         \n}\n"
        );
    }
    #[test]
    fn and_unknown() {
        assert_eq!(
            runner().ok("a {b: calc(1unknown + 1other)}\n"),
            "a {\
         \n  b: calc(1unknown + 1other);\
         \n}\n"
        );
    }
}
