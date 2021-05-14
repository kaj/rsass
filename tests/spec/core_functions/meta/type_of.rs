//! Tests auto-converted from "sass-spec/spec/core_functions/meta/type_of.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn arglist() {
    assert_eq!(
        runner().ok("@function type-of-arglist($args...) {\
             \n  @return type-of($args);\
             \n}\n\
             \na {b: type-of-arglist()}\n"),
        "a {\
         \n  b: arglist;\
         \n}\n"
    );
}
mod boolean {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn test_false() {
        assert_eq!(
            runner().ok("a {b: type-of(false)}\n"),
            "a {\
         \n  b: bool;\
         \n}\n"
        );
    }
    #[test]
    fn test_true() {
        assert_eq!(
            runner().ok("a {b: type-of(true)}\n"),
            "a {\
         \n  b: bool;\
         \n}\n"
        );
    }
}
#[test]
fn color() {
    assert_eq!(
        runner().ok("a {b: type-of(red)}\n"),
        "a {\
         \n  b: color;\
         \n}\n"
    );
}
mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: type-of()}\n"),
            "Error: Missing argument $value.\
         \n  ,--> input.scss\
         \n1 | a {b: type-of()}\
         \n  |       ^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function type-of($value) {\
         \n  |           =============== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: type-of(1, 2)}\n"),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: type-of(1, 2)}\
         \n  |       ^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function type-of($value) {\
         \n  |           =============== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
#[test]
fn function() {
    assert_eq!(
        runner().ok("a {b: type-of(get-function(\"type-of\"))}\n"),
        "a {\
         \n  b: function;\
         \n}\n"
    );
}
mod list {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn empty() {
        assert_eq!(
            runner().ok("a {b: type-of(())}\n"),
            "a {\
         \n  b: list;\
         \n}\n"
        );
    }
    #[test]
    fn non_empty() {
        assert_eq!(
            runner().ok("a {b: type-of(1 2 3)}\n"),
            "a {\
         \n  b: list;\
         \n}\n"
        );
    }
}
mod map {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn empty() {
        assert_eq!(
            runner().ok("a {b: type-of(map-remove((c: d), c))}\n"),
            "a {\
         \n  b: map;\
         \n}\n"
        );
    }
    #[test]
    fn non_empty() {
        assert_eq!(
            runner().ok("a {b: type-of((c: d))}\n"),
            "a {\
         \n  b: map;\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("a {b: type-of($value: c)}\n"),
        "a {\
         \n  b: string;\
         \n}\n"
    );
}
#[test]
fn null() {
    assert_eq!(
        runner().ok("a {b: type-of(null)}\n"),
        "a {\
         \n  b: null;\
         \n}\n"
    );
}
mod number {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn unit() {
        assert_eq!(
            runner().ok("a {b: type-of(1.5px * 3.4em)}\n"),
            "a {\
         \n  b: number;\
         \n}\n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            runner().ok("a {b: type-of(1)}\n"),
            "a {\
         \n  b: number;\
         \n}\n"
        );
    }
}
mod string {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn quoted() {
        assert_eq!(
            runner().ok("a {b: type-of(\"c\")}\n"),
            "a {\
         \n  b: string;\
         \n}\n"
        );
    }
    #[test]
    fn unquoted() {
        assert_eq!(
            runner().ok("a {b: type-of(c)}\n"),
            "a {\
         \n  b: string;\
         \n}\n"
        );
    }
}
