//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1355.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "@function test() {\
             \n  @return;\
             \n}\
             \n\
             \ndiv {\
             \n  x: type-of(test());\
             \n}"
        )
        .unwrap_err(),
        "Error: Expected expression.\
         \n  ,\
         \n2 |   @return;\
         \n  |          ^\
         \n  \'\
         \n  input.scss 2:10  root stylesheet\
         \n",
    );
}
