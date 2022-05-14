//! Tests auto-converted from "sass-spec/spec/core_functions/list/set_nth.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("set_nth")
}

mod t1 {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn of_1() {
        assert_eq!(
            runner().ok("$result: set-nth(join((), b), 1, c);\
             \na {\
             \n  result: $result;\
             \n  type: type-of($result);\
             \n}\n"),
            "a {\
         \n  result: c;\
         \n  type: list;\
         \n}\n"
        );
    }
    #[test]
    fn of_2() {
        assert_eq!(
            runner().ok("a {b: set-nth(c d, 1, e)}\n"),
            "a {\
         \n  b: e d;\
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
            runner().ok("a {b: set-nth(c d, 2, e)}\n"),
            "a {\
         \n  b: c e;\
         \n}\n"
        );
    }
    #[test]
    fn of_4() {
        assert_eq!(
            runner().ok("a {b: set-nth(c d e f, 2, g)}\n"),
            "a {\
         \n  b: c g e f;\
         \n}\n"
        );
    }
}
#[test]
fn bracketed() {
    assert_eq!(
        runner().ok("a {b: set-nth([c, d], 2, e)}\n"),
        "a {\
         \n  b: [c, e];\
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
                runner().err("a {b: set-nth(c d, 0, e)}\n"),
                "Error: $n: List index may not be 0.\
         \n  ,\
         \n1 | a {b: set-nth(c d, 0, e)}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn too_few_args() {
            assert_eq!(
                runner().err("a {b: set-nth(c d, 1)}\n"),
                "Error: Missing argument $value.\
         \n  ,--> input.scss\
         \n1 | a {b: set-nth(c d, 1)}\
         \n  |       ^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:list\
         \n1 | @function set-nth($list, $n, $value) {\
         \n  |           ========================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn too_high() {
            assert_eq!(
                runner().err("a {b: set-nth(c d, 3, e)}\n"),
                "Error: $n: Invalid index 3 for a list with 2 elements.\
         \n  ,\
         \n1 | a {b: set-nth(c d, 3, e)}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn too_low() {
            assert_eq!(
                runner().err("a {b: set-nth(c d, -3, e)}\n"),
                "Error: $n: Invalid index -3 for a list with 2 elements.\
         \n  ,\
         \n1 | a {b: set-nth(c d, -3, e)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
        #[test]
        fn too_many_args() {
            assert_eq!(
                runner().err("a {b: set-nth(c d, 1, 2, 3)}\n"),
                "Error: Only 3 arguments allowed, but 4 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: set-nth(c d, 1, 2, 3)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:list\
         \n1 | @function set-nth($list, $n, $value) {\
         \n  |           ========================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
            );
        }
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err("a {b: set-nth(c d, e, f)}\n"),
            "Error: $n: e is not a number.\
         \n  ,\
         \n1 | a {b: set-nth(c d, e, f)}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
#[test]
fn map() {
    assert_eq!(
        runner().ok("a {b: set-nth((c: d, e: f, g: h), 2, i)}\n"),
        "a {\
         \n  b: c d, i, g h;\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("a {b: set-nth($list: c d, $n: 1, $value: e)}\n"),
        "a {\
         \n  b: e d;\
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
                runner().ok("$result: set-nth(join((), b), -1, c);\
             \na {\
             \n  result: $result;\
             \n  type: type-of($result);\
             \n}\n"),
                "a {\
         \n  result: c;\
         \n  type: list;\
         \n}\n"
            );
        }
        #[test]
        fn of_2() {
            assert_eq!(
                runner().ok("a {b: set-nth(c d, -1, e)}\n"),
                "a {\
         \n  b: c e;\
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
                runner().ok("a {b: set-nth(c d, -2, e)}\n"),
                "a {\
         \n  b: e d;\
         \n}\n"
            );
        }
        #[test]
        fn of_4() {
            assert_eq!(
                runner().ok("a {b: set-nth(c d e f, -2, g)}\n"),
                "a {\
         \n  b: c d g f;\
         \n}\n"
            );
        }
    }
}
#[test]
fn non_list() {
    assert_eq!(
        runner().ok("$result: set-nth(b, 1, c);\
             \na {\
             \n  result: $result;\
             \n  type: type-of($result);\
             \n}\n"),
        "a {\
         \n  result: c;\
         \n  type: list;\
         \n}\n"
    );
}
