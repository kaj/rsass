//! Tests auto-converted from "sass-spec/spec/core_functions/color/rgba/error/zero_args.hrx"

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        crate::rsass(
            "a {\
             \n  b: rgba();\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: Missing argument $channels.\
         \n  ,--> input.scss\
         \n2 |   b: rgba();\
         \n  |      ^^^^^^ invocation\
         \n  \'\
         \n  ,\
         \n1 | @function rgba($channels) {\
         \n  |           =============== declaration\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
}
