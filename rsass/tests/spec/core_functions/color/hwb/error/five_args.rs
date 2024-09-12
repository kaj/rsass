//! Tests auto-converted from "sass-spec/spec/core_functions/color/hwb/error/five_args.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("five_args")
}

#[test]
fn test() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.hwb(0, 30%, 40%, 0.5, 0)}\n"
        ),
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
