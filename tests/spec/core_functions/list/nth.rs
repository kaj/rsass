//! Tests auto-converted from "sass-spec/spec/core_functions/list/nth.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("nth")
}

mod t1 {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn of_1() {
        assert_eq!(
            runner().ok("a {b: nth(join((), c), 1)}\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
    #[test]
    fn of_2() {
        assert_eq!(
            runner().ok("a {b: nth(c d, 1)}\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
}
mod t2 {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn of_2() {
        assert_eq!(
            runner().ok("a {b: nth(c d, 2)}\n"),
            "a {\
         \n  b: d;\
         \n}\n"
        );
    }
    #[test]
    fn of_4() {
        assert_eq!(
            runner().ok("a {b: nth(c d e f, 2)}\n"),
            "a {\
         \n  b: d;\
         \n}\n"
        );
    }
}
#[test]
fn bracketed() {
    assert_eq!(
        runner().ok("a {b: nth([c, d], 2)}\n"),
        "a {\
         \n  b: d;\
         \n}\n"
    );
}
mod error {
    #[allow(unused)]
    use super::runner;

    mod index {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn t0() {
            assert_eq!(
                runner().err("a {b: nth(c d, 0)}\n"),
                "Error: $n: List index may not be 0.\
         \n  ,\
         \n1 | a {b: nth(c d, 0)}\
         \n  |       ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn too_high() {
            assert_eq!(
                runner().err("a {b: nth(c d, 3)}\n"),
                "Error: $n: Invalid index 3 for a list with 2 elements.\
         \n  ,\
         \n1 | a {b: nth(c d, 3)}\
         \n  |       ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn too_low() {
            assert_eq!(
                runner().err("a {b: nth(c d, -3)}\n"),
                "Error: $n: Invalid index -3 for a list with 2 elements.\
         \n  ,\
         \n1 | a {b: nth(c d, -3)}\
         \n  |       ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: nth(c d)}\n"),
            "Error: Missing argument $n.\
         \n  ,--> input.scss\
         \n1 | a {b: nth(c d)}\
         \n  |       ^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:list\
         \n1 | @function nth($list, $n) {\
         \n  |           ============== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: nth(c d, 1, 2)}\n"),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: nth(c d, 1, 2)}\
         \n  |       ^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:list\
         \n1 | @function nth($list, $n) {\
         \n  |           ============== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err("a {b: nth(c d, e)}\n"),
            "Error: $n: e is not a number.\
         \n  ,\
         \n1 | a {b: nth(c d, e)}\
         \n  |       ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
#[test]
fn map() {
    assert_eq!(
        runner().ok("a {b: nth((c: d, e: f, g: h), 2)}\n"),
        "a {\
         \n  b: e f;\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("a {b: nth($list: c d, $n: 1)}\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
mod negative {
    #[allow(unused)]
    use super::runner;

    mod t1 {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn of_1() {
            assert_eq!(
                runner().ok("a {b: nth(join((), c), -1)}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        fn of_2() {
            assert_eq!(
                runner().ok("a {b: nth(c d, -1)}\n"),
                "a {\
         \n  b: d;\
         \n}\n"
            );
        }
    }
    mod t2 {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn of_2() {
            assert_eq!(
                runner().ok("a {b: nth(c d, -2)}\n"),
                "a {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        fn of_4() {
            assert_eq!(
                runner().ok("a {b: nth(c d e f, -2)}\n"),
                "a {\
         \n  b: e;\
         \n}\n"
            );
        }
    }
}
#[test]
fn non_list() {
    assert_eq!(
        runner().ok("a {b: nth(c, 1)}\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn units() {
    assert_eq!(
        runner().ok("a {b: nth(c d, 1px)}\n"),
        "a {\
         \n  b: c;\
         \n}\n"
    );
}
