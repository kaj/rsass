//! Tests auto-converted from "sass-spec/spec/core_functions/color/blackness.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("blackness")
}

mod error {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \'sass:color\';\
             \na {b: color.blackness()}\n"
            ),
            "Error: Missing argument $color.\
         \n  ,--> input.scss\
         \n2 | a {b: color.blackness()}\
         \n  |       ^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function blackness($color) {\
         \n  |           ================= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \'sass:color\';\
             \na {b: color.blackness(red, green)}\n"
            ),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: color.blackness(red, green)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function blackness($color) {\
         \n  |           ================= declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \'sass:color\';\
             \na {b: color.blackness(1)}\n"
            ),
            "Error: $color: 1 is not a color.\
         \n  ,\
         \n2 | a {b: color.blackness(1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
        );
    }
}
#[test]
fn fraction() {
    assert_eq!(
        runner().ok("@use \'sass:color\';\
             \na {b: color.blackness(color.hwb(0, 0%, 0.5%))}\n"),
        "a {\
         \n  b: 0.3921568627%;\
         \n}\n"
    );
}
#[test]
fn max() {
    assert_eq!(
        runner().ok("@use \'sass:color\';\
             \na {b: color.blackness(black)}\n"),
        "a {\
         \n  b: 100%;\
         \n}\n"
    );
}
mod middle {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn half_whiteness() {
        assert_eq!(
            runner().ok("@use \'sass:color\';\
             \na {b: color.blackness(color.hwb(0, 50%, 50%))}\n"),
            "a {\
         \n  b: 49.8039215686%;\
         \n}\n"
        );
    }
    #[test]
    fn high_whiteness() {
        assert_eq!(
            runner().ok("@use \'sass:color\';\
             \na {b: color.blackness(color.hwb(0, 70%, 70%))}\n"),
            "a {\
         \n  b: 49.8039215686%;\
         \n}\n"
        );
    }
    #[test]
    fn zero_whiteness() {
        assert_eq!(
            runner().ok("@use \'sass:color\';\
             \na {b: color.blackness(color.hwb(0, 0%, 50%))}\n"),
            "a {\
         \n  b: 49.8039215686%;\
         \n}\n"
        );
    }
}
#[test]
fn min() {
    assert_eq!(
        runner().ok("@use \'sass:color\';\
             \na {b: color.blackness(white)}\n"),
        "a {\
         \n  b: 0%;\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok("@use \'sass:color\';\
             \na {b: color.blackness($color: color.hwb(0, 0%, 42%))}\n"),
        "a {\
         \n  b: 41.9607843137%;\
         \n}\n"
    );
}
