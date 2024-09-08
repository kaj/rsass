//! Tests auto-converted from "sass-spec/spec/directives/function.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("function")
}

#[test]
#[ignore] // wrong result
fn custom_ident_call() {
    assert_eq!(
        runner().ok("@function __a() {@return 1}\
             \nb {c: --a()}\n"),
        "b {\
         \n  c: 1;\
         \n}\n"
    );
}
#[test]
#[ignore] // wrong result
fn custom_ident_name() {
    assert_eq!(
        runner().ok("@function --a() {@return 1}\
             \nb {c: --a()}\n"),
        "b {\
         \n  c: 1;\
         \n}\n"
    );
}
#[test]
fn double_underscore_name() {
    assert_eq!(
        runner().ok("@function __a() {@return 1}\
             \nb {c: __a()}\n"),
        "b {\
         \n  c: 1;\
         \n}\n"
    );
}
#[test]
fn escaped() {
    assert_eq!(
        runner().ok(
            "// Function names can be defined and referred to using escapes, which are\
             \n// normalized.\
             \n@function f\\6Fo-bar() {@return 1}\n\
             \na {b: foo-b\\61r()}\n"
        ),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
mod vendor_like_underscore {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // wrong result
    fn middle() {
        assert_eq!(
            runner().ok("@function -moz_calc() {@return 1}\
             \nb {c: -moz_calc()}\n"),
            "b {\
         \n  c: 1;\
         \n}\n"
        );
    }
    #[test]
    fn start() {
        assert_eq!(
            runner().ok("@function _moz-calc() {@return 1}\
             \nb {c: _moz-calc()}\n"),
            "b {\
         \n  c: 1;\
         \n}\n"
        );
    }
}