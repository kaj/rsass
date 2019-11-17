//! Tests auto-converted from "sass-spec/spec/libsass/error-directive-nested/function.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function c() {\
             \n  @error test;\
             \n  @return d;\
             \n}\
             \n\
             \na {\
             \n  b: {\
             \n    c: c();\
             \n  }\
             \n}\
             \n"
        )
        .unwrap_err(),
        "Error: test\
         \n  ,\
         \n8 |     c: c();\
         \n  |        ^^^\
         \n  \'\
         \n  input.scss 8:8  root stylesheet\
         \n",
    );
}
