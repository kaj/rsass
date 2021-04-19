//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/error/zero_args.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "a {\
             \n  b: hsl();\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: Missing argument $channels.\
         \n  ,--> input.scss\
         \n2 |   b: hsl();\
         \n  |      ^^^^^ invocation\
         \n  \'\
         \n  ,\
         \n1 | @function hsl($channels) {\
         \n  |           ============== declaration\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
}
