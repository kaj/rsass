//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsl/error/two_args.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "a {\
             \n  b: hsl(#123, 0.5);\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: Missing argument $lightness.\
         \n  ,\
         \n2 |   b: hsl(#123, 0.5);\
         \n  |      ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet",
    );
}
