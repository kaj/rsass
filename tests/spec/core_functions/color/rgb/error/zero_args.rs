//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgb/error/zero_args.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "a {\
             \n  b: rgb();\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: Missing argument $channels.\
         \n  ,--> input.scss\
         \n2 |   b: rgb();\
         \n  |      ^^^^^ invocation\
         \n  \'\
         \n  ,\
         \n1 | @function rgb($channels) {\
         \n  |           ============== declaration\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
}
