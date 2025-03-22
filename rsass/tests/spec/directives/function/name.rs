//! Tests auto-converted from "sass-spec/spec/directives/function/name.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("name")
}

mod custom_ident {
    use super::runner;

    #[test]
    fn call() {
        assert_eq!(
            runner().ok("@function __a() {@return 1}\
             \nb {c: --a()}\n"),
            "b {\
         \n  c: 1;\
         \n}\n"
        );
    }
    #[test]
    fn define() {
        assert_eq!(
            runner().ok("@function --a() {@return 1}\
             \nb {c: --a()}\n"),
            "b {\
         \n  c: 1;\
         \n}\n"
        );
    }
}
#[test]
fn double_underscore() {
    assert_eq!(
        runner().ok("@function __a() {@return 1}\
             \nb {c: __a()}\n"),
        "b {\
         \n  c: 1;\
         \n}\n"
    );
}
mod error {
    use super::runner;

    mod special {
        use super::runner;

        #[test]
        #[ignore] // missing error
        fn and() {
            assert_eq!(
                runner().err("@function and() {@return 1}\n"),
                "Error: Invalid function name.\
         \n  ,\
         \n1 | @function and() {@return 1}\
         \n  | ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
            );
        }
        #[test]
        fn calc() {
            assert_eq!(
                runner().err("@function calc() {@return 1}\n"),
                "Error: Invalid function name.\
         \n  ,\
         \n1 | @function calc() {@return 1}\
         \n  | ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn clamp() {
            assert_eq!(
                runner().err("@function clamp() {@return 1}\n"),
                "Error: Invalid function name.\
         \n  ,\
         \n1 | @function clamp() {@return 1}\
         \n  | ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
            );
        }
        #[test]
        fn element() {
            assert_eq!(
                runner().err("@function element() {@return 1}\n"),
                "Error: Invalid function name.\
         \n  ,\
         \n1 | @function element() {@return 1}\
         \n  | ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
            );
        }
        #[test]
        fn expression() {
            assert_eq!(
                runner().err("@function expression() {@return 1}\n"),
                "Error: Invalid function name.\
         \n  ,\
         \n1 | @function expression() {@return 1}\
         \n  | ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn not() {
            assert_eq!(
                runner().err("@function not() {@return 1}\n"),
                "Error: Invalid function name.\
         \n  ,\
         \n1 | @function not() {@return 1}\
         \n  | ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
            );
        }
        #[test]
        #[ignore] // missing error
        fn or() {
            assert_eq!(
                runner().err("@function or() {@return 1}\n"),
                "Error: Invalid function name.\
         \n  ,\
         \n1 | @function or() {@return 1}\
         \n  | ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
            );
        }
        #[test]
        fn url() {
            assert_eq!(
                runner().err("@function url() {@return 1}\n"),
                "Error: Invalid function name.\
         \n  ,\
         \n1 | @function url() {@return 1}\
         \n  | ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:1  root stylesheet",
            );
        }
    }
}
mod special {
    use super::runner;

    mod test_type {
        use super::runner;

        #[test]
        fn lowercase() {
            assert_eq!(
                runner().ok("@function type() {@return 1}\
             \na {b: type()}\n"),
                "a {\
         \n  b: 1;\
         \n}\n"
            );
        }
        #[test]
        fn uppercase() {
            assert_eq!(
                runner().ok("@function TYPE() {@return 1}\
             \na {b: TYPE()}\n"),
                "a {\
         \n  b: 1;\
         \n}\n"
            );
        }
    }
}
mod vendor_like_underscore {
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
