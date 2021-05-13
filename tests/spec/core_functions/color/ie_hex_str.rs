//! Tests auto-converted from "sass-spec/spec/core_functions/color/ie_hex_str.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod error {
    #[allow(unused)]
    use super::runner;
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err("a {b: ie-hex-str()}\n"),
            "Error: Missing argument $color.\
         \n  ,--> input.scss\
         \n1 | a {b: ie-hex-str()}\
         \n  |       ^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function ie-hex-str($color) {\
         \n  |           ================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err("a {b: ie-hex-str(red, blue)}\n"),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: ie-hex-str(red, blue)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function ie-hex-str($color) {\
         \n  |           ================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err("a {b: ie-hex-str(c)}\n"),
            "Error: $color: c is not a color.\
         \n  ,\
         \n1 | a {b: ie-hex-str(c)}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
#[test]
fn leading_zero() {
    assert_eq!(
        runner().ok("a {b: ie-hex-str(rgba(#020304, 0.003))}\n"),
        "a {\
         \n  b: #01020304;\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("a {b: ie-hex-str($color: #daddee)}\n"),
        "a {\
         \n  b: #FFDADDEE;\
         \n}\n"
    );
}
#[test]
fn opaque() {
    assert_eq!(
        runner().ok("a {b: ie-hex-str(#daddee)}\n"),
        "a {\
         \n  b: #FFDADDEE;\
         \n}\n"
    );
}
#[test]
fn translucent() {
    assert_eq!(
        runner().ok("a {b: ie-hex-str(rgba(#daddee, 0.3))}\n"),
        "a {\
         \n  b: #4DDADDEE;\
         \n}\n"
    );
}
#[test]
fn transparent() {
    assert_eq!(
        runner().ok("a {b: ie-hex-str(rgba(turquoise, 0))}\n"),
        "a {\
         \n  b: #0040E0D0;\
         \n}\n"
    );
}
#[test]
fn test_type() {
    assert_eq!(
        runner().ok("a {b: type-of(ie-hex-str(#daddee))}\n"),
        "a {\
         \n  b: string;\
         \n}\n"
    );
}
