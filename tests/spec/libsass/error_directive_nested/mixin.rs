//! Tests auto-converted from "sass-spec/spec/libsass/error-directive-nested/mixin.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin c() {\
             \n  @error test;\
             \n  c: d;\
             \n}\
             \n\
             \na {\
             \n  b: {\
             \n    @include c();\
             \n  }\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: test\
         \n  ,\
         \n8 |     @include c();\
         \n  |     ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 8:5  root stylesheet",
    );
}
