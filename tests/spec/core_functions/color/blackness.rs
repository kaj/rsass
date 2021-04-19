//! Tests auto-converted from "sass-spec/spec/core_functions/color/blackness.hrx"

mod error {
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
             \na {b: color.blackness()}\
             \n"
            )
            .unwrap_err(),
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
            crate::rsass(
                "@use \'sass:color\';\
             \na {b: color.blackness(red, green)}\
             \n"
            )
            .unwrap_err(),
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
            crate::rsass(
                "@use \'sass:color\';\
             \na {b: color.blackness(1)}\
             \n"
            )
            .unwrap_err(),
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
#[ignore] // wrong result
fn fraction() {
    assert_eq!(
        crate::rsass(
            "@use \'sass:color\';\
            \na {b: color.blackness(color.hwb(0, 0%, 0.5%))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0.3921568627%;\
        \n}\
        \n"
    );
}
#[test]
fn max() {
    assert_eq!(
        crate::rsass(
            "@use \'sass:color\';\
            \na {b: color.blackness(black)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 100%;\
        \n}\
        \n"
    );
}
mod middle {
    #[test]
    #[ignore] // wrong result
    fn half_whiteness() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
            \na {b: color.blackness(color.hwb(0, 50%, 50%))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 49.8039215686%;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn high_whiteness() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
            \na {b: color.blackness(color.hwb(0, 70%, 70%))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 49.8039215686%;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn zero_whiteness() {
        assert_eq!(
            crate::rsass(
                "@use \'sass:color\';\
            \na {b: color.blackness(color.hwb(0, 0%, 50%))}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: 49.8039215686%;\
        \n}\
        \n"
        );
    }
}
#[test]
fn min() {
    assert_eq!(
        crate::rsass(
            "@use \'sass:color\';\
            \na {b: color.blackness(white)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0%;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // wrong result
fn named() {
    assert_eq!(
        crate::rsass(
            "@use \'sass:color\';\
            \na {b: color.blackness($color: color.hwb(0, 0%, 42%))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 41.9607843137%;\
        \n}\
        \n"
    );
}
