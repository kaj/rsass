//! Tests auto-converted from "sass-spec/spec/directives/function.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("function")
}

mod comment {
    #[allow(unused)]
    use super::runner;

    mod function {
        #[allow(unused)]
        use super::runner;

        mod after_args {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(runner().ok("@function a() /**/ {}\n"), "");
            }
            #[test]
            fn silent() {
                assert_eq!(
                    runner().ok("@function a() //\
             \n  {}\n"),
                    ""
                );
            }
        }
        mod before_name {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(runner().ok("@function /**/ a() {}\n"), "");
            }
            #[test]
            fn silent() {
                assert_eq!(
                    runner().ok("@function //\
             \n  a() {}\n"),
                    ""
                );
            }
        }
    }
    mod test_return {
        #[allow(unused)]
        use super::runner;

        mod after_value {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(
                    runner().ok("@function a() {@return b /**/}\n"),
                    ""
                );
            }
            #[test]
            fn silent() {
                assert_eq!(
                    runner().ok("@function a() {\
             \n  @return b //\
             \n}\n"),
                    ""
                );
            }
        }
        mod before_value {
            #[allow(unused)]
            use super::runner;

            #[test]
            fn loud() {
                assert_eq!(
                    runner().ok("@function a() {@return /**/ b}\n"),
                    ""
                );
            }
            #[test]
            fn silent() {
                assert_eq!(
                    runner().ok("@function a() {\
             \n  @return //\
             \n    b\
             \n}\n"),
                    ""
                );
            }
        }
    }
}
#[test]
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
