//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/error/five_args.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "@use \'sass:color\';\
             \na {b: color.hwb(0, 30%, 40%, 0.5, 0)}\
             \n"
        ).unwrap_err(),
        "Error: Only 4 arguments allowed, but 5 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: color.hwb(0, 30%, 40%, 0.5, 0)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,\
         \n1 | @function hwb($hue, $whiteness, $blackness, $alpha: 1) {\
         \n  |           ============================================ declaration\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
