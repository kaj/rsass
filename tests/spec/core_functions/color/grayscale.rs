//! Tests auto-converted from "sass-spec/spec/core_functions/color/grayscale.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("grayscale")
}

#[test]
fn alpha() {
    assert_eq!(
        runner().ok("a {b: grayscale(rgba(#633736, 0.3))}\n"),
        "a {\
         \n  b: rgba(77, 77, 77, 0.3);\
         \n}\n"
    );
}
mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: grayscale()}\n"),
            "Error: Missing argument $color.\
         \n  ,--> input.scss\
         \n1 | a {b: grayscale()}\
         \n  |       ^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function grayscale($color) {\
         \n  |           ================= declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: grayscale(red, 1)}\n"),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: grayscale(red, 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function grayscale($color) {\
         \n  |           ================= declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err("a {b: grayscale(c)}\n"),
            "Error: $color: c is not a color.\
         \n  ,\
         \n1 | a {b: grayscale(c)}\
         \n  |       ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
#[test]
fn max_saturation() {
    assert_eq!(
        runner().ok("a {b: grayscale(red)}\n"),
        "a {\
         \n  b: gray;\
         \n}\n"
    );
}
#[test]
fn mid_saturation() {
    assert_eq!(
        runner().ok("a {b: grayscale(#633736)}\n"),
        "a {\
         \n  b: #4d4d4d;\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("a {b: grayscale($color: white)}\n"),
        "a {\
         \n  b: white;\
         \n}\n"
    );
}
mod no_saturation {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn black() {
        assert_eq!(
            runner().ok("a {b: grayscale(black)}\n"),
            "a {\
         \n  b: black;\
         \n}\n"
        );
    }
    #[test]
    fn gray() {
        assert_eq!(
            runner().ok("a {b: grayscale(#494949)}\n"),
            "a {\
         \n  b: #494949;\
         \n}\n"
        );
    }
    #[test]
    fn white() {
        assert_eq!(
            runner().ok("a {b: grayscale(white)}\n"),
            "a {\
         \n  b: white;\
         \n}\n"
        );
    }
}
#[test]
fn number() {
    assert_eq!(
        runner().ok(
            "// A number should produce a plain function string, for CSS filter functions.\
             \na {b: grayscale(15%)}\n"
        ),
        "a {\
         \n  b: grayscale(15%);\
         \n}\n"
    );
}
