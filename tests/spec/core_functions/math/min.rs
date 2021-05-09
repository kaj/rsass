//! Tests auto-converted from "sass-spec/spec/core_functions/math/min.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod error {
    #[allow(unused)]
    use super::runner;
    #[test]
    fn incompatible_units() {
        assert_eq!(
            runner().err(
                "$arg: 1px;\
             \na {b: min($arg, 2s)}\n"
            ),
            "Error: 1px and 2s have incompatible units.\
         \n  ,\
         \n2 | a {b: min($arg, 2s)}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: min()}\n"),
            "Error: At least one argument must be passed.\
         \n  ,\
         \n1 | a {b: min()}\
         \n  |       ^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    mod test_type {
        #[allow(unused)]
        use super::runner;
        #[test]
        fn arg_1() {
            assert_eq!(
                runner().err(
                    "$arg: c;\
             \na {b: min($arg)}\n"
                ),
                "Error: c is not a number.\
         \n  ,\
         \n2 | a {b: min($arg)}\
         \n  |       ^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn arg_2() {
            assert_eq!(
                runner().err(
                    "$arg: c;\
             \na {b: min(1, $arg)}\n"
                ),
                "Error: c is not a number.\
         \n  ,\
         \n2 | a {b: min(1, $arg)}\
         \n  |       ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
        #[test]
        fn arg_3() {
            assert_eq!(
                runner().err(
                    "$arg: c;\
             \na {b: min(1, 2, $arg)}\n"
                ),
                "Error: c is not a number.\
         \n  ,\
         \n2 | a {b: min(1, 2, $arg)}\
         \n  |       ^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
            );
        }
    }
}
#[test]
fn one_arg() {
    assert_eq!(
        runner().ok("$arg: 1;\
             \na {b: min($arg)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn three_args() {
    assert_eq!(
        runner().ok("$arg: 1;\
             \na {b: min(3, $arg, 2)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn two_args() {
    assert_eq!(
        runner().ok("$arg: 1;\
             \na {b: min($arg, 2)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
mod units {
    #[allow(unused)]
    use super::runner;
    #[test]
    fn and_unitless() {
        assert_eq!(
            runner().ok("$arg: 2px;\
             \na {b: min($arg, 1)}\n"),
            "a {\
         \n  b: 1;\
         \n}\n"
        );
    }
    #[test]
    fn compatible() {
        assert_eq!(
            runner().ok("$arg: 1px;\
             \na {b: min($arg, 1in, 1cm)}\n"),
            "a {\
         \n  b: 1px;\
         \n}\n"
        );
    }
    #[test]
    fn same() {
        assert_eq!(
            runner().ok("$arg: 6px;\
             \na {b: min($arg, 2px, 10px)}\n"),
            "a {\
         \n  b: 2px;\
         \n}\n"
        );
    }
}
