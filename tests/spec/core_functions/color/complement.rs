//! Tests auto-converted from "sass-spec/spec/core_functions/color/complement.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn alpha() {
    assert_eq!(
        runner().ok("a {b: complement(rgba(turquoise, 0.7))}\n"),
        "a {\
         \n  b: rgba(224, 64, 80, 0.7);\
         \n}\n"
    );
}
mod error {
    #[allow(unused)]
    use super::runner;
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: complement()}\n"),
            "Error: Missing argument $color.\
         \n  ,--> input.scss\
         \n1 | a {b: complement()}\
         \n  |       ^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function complement($color) {\
         \n  |           ================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: complement(red, 1)}\n"),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: complement(red, 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function complement($color) {\
         \n  |           ================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err("a {b: complement(1)}\n"),
            "Error: $color: 1 is not a color.\
         \n  ,\
         \n1 | a {b: complement(1)}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
mod grayscale {
    #[allow(unused)]
    use super::runner;
    #[test]
    fn black() {
        assert_eq!(
            runner().ok("a {b: complement(black)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
    #[test]
    fn gray() {
        assert_eq!(
            runner().ok("a {b: complement(gray)}\n"),
            "a {\
         \n  b: gray;\
         \n}\n"
        );
    }
    #[test]
    fn white() {
        assert_eq!(
            runner().ok("a {b: complement(white)}\n"),
            "a {\
         \n  b: white;\
         \n}\n"
        );
    }
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("a {b: complement($color: red)}\n"),
        "a {\
         \n  b: aqua;\
         \n}\n"
    );
}
#[test]
fn red() {
    assert_eq!(
        runner().ok("a {b: complement(red)}\n"),
        "a {\
         \n  b: aqua;\
         \n}\n"
    );
}
#[test]
fn turquoise() {
    assert_eq!(
        runner().ok("a {b: complement(turquoise)}\n"),
        "a {\
         \n  b: #e04050;\
         \n}\n"
    );
}
