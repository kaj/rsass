//! Tests auto-converted from "sass-spec/spec/css/supports/syntax/calculations.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("calculations")
}

mod calc {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn contains_interpolation() {
        assert_eq!(
            runner().ok("@supports (a: calc(#{1 + 2})) {@d}\n"),
            "@supports (a: calc(3)) {\
         \n  @d;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn in_property_name() {
        assert_eq!(
            runner().ok("@supports (calc(0): a) {@d}\n"),
            "@supports (calc(0): a) {\
         \n  @d;\
         \n}\n"
        );
    }
    #[test]
    fn interpolated() {
        assert_eq!(
            runner().ok("@supports (a: #{calc(1 + 2)}) {@d}\n"),
            "@supports (a: 3) {\
         \n  @d;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn nested() {
        assert_eq!(
            runner().ok("@supports (a: calc(1 + calc(2 + 3))) {@d}\n"),
            "@supports (a: calc(1 + calc(2 + 3))) {\
         \n  @d;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn simple() {
        assert_eq!(
            runner().ok("@supports (a: calc(0)) {@d}\n"),
            "@supports (a: calc(0)) {\
         \n  @d;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn with_operation() {
        assert_eq!(
            runner().ok("@supports (a: calc(1 + 2)) {@d}\n"),
            "@supports (a: calc(1 + 2)) {\
         \n  @d;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn with_variable() {
        assert_eq!(
            runner().ok("$x: 2;\
             \n@supports (a: calc(1 + $x)) {@d}\n"),
            "@supports (a: calc(1 + 2)) {\
         \n  @d;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // wrong result
fn clamp() {
    assert_eq!(
        runner().ok("@supports (a: clamp(0, 1, 2)) {@d}\n"),
        "@supports (a: clamp(0, 1, 2)) {\
         \n  @d;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn max() {
    assert_eq!(
        runner().ok("@supports (a: max(0)) {@d}\n"),
        "@supports (a: max(0)) {\
         \n  @d;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn min() {
    assert_eq!(
        runner().ok("@supports (a: min(0)) {@d}\n"),
        "@supports (a: min(0)) {\
         \n  @d;\
         \n}\n"
    );
}
