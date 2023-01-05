//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1355.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1355")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@function test() {\
             \n  @return;\
             \n}\n\
             \ndiv {\
             \n  x: type-of(test());\
             \n}"
        ),
        "Error: Expected expression.\
         \n  ,\
         \n2 |   @return;\
         \n  |          ^\
         \n  \'\
         \n  input.scss 2:10  root stylesheet",
    );
}
