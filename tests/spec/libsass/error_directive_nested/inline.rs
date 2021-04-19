//! Tests auto-converted from "sass-spec/spec/libsass/error-directive-nested/inline.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "a {\
             \n  b: {\
             \n    @error test;\
             \n    c: d;\
             \n  }\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: test\
         \n  ,\
         \n3 |     @error test;\
         \n  |     ^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:5  root stylesheet",
    );
}
