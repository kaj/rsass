//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1803/nested.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("nested")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "a {\
             \n  display: block\n\
             \n  b {\
             \n    c {\
             \n      foo: bar;\
             \n    }\
             \n  }\
             \n}\n"
        ),
        "Error: expected \":\".\
         \n  ,\
         \n5 |     c {\
         \n  |       ^\
         \n  \'\
         \n  input.scss 5:7  root stylesheet",
    );
}
