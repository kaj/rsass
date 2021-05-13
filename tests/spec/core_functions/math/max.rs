//! Tests auto-converted from "sass-spec/spec/core_functions/math/max.hrx"

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
             \na {b: max($arg, 2s)}\n\n"
            ),
            "Error: 1px and 2s have incompatible units.\
         \n  ,\
         \n2 | a {b: max($arg, 2s)}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: max()}\n"),
            "Error: At least one argument must be passed.\
         \n  ,\
         \n1 | a {b: max()}\
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
             \na {b: max($arg)}\n"
                ),
                "Error: c is not a number.\
         \n  ,\
         \n2 | a {b: max($arg)}\
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
             \na {b: max(1, $arg)}\n"
                ),
                "Error: c is not a number.\
         \n  ,\
         \n2 | a {b: max(1, $arg)}\
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
             \na {b: max(1, 2, $arg)}\n"
                ),
                "Error: c is not a number.\
         \n  ,\
         \n2 | a {b: max(1, 2, $arg)}\
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
             \na {b: max($arg)}\n"),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn three_args() {
    assert_eq!(
        runner().ok("$arg: 1;\
             \na {b: max(3, $arg, 2)}\n"),
        "a {\
         \n  b: 3;\
         \n}\n"
    );
}
#[test]
fn two_args() {
    assert_eq!(
        runner().ok("$arg: 1;\
             \na {b: max($arg, 2)}\n"),
        "a {\
         \n  b: 2;\
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
             \na {b: max($arg, 1)}\n"),
            "a {\
         \n  b: 2px;\
         \n}\n"
        );
    }
    #[test]
    fn compatible() {
        assert_eq!(
            runner().ok("$arg: 1px;\
             \na {b: max($arg, 1in, 1cm)}\n"),
            "a {\
         \n  b: 1in;\
         \n}\n"
        );
    }
    #[test]
    fn same() {
        assert_eq!(
            runner().ok("$arg: 6px;\
             \na {b: max($arg, 2px, 10px)}\n"),
            "a {\
         \n  b: 10px;\
         \n}\n"
        );
    }
}
