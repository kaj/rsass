//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1441/sibling.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("sibling")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".sibling {\
             \n    & ~ & {\
             \n        foo: bar;\
             \n    }\
             \n}\n"),
        ".sibling ~ .sibling {\
         \n  foo: bar;\
         \n}\n"
    );
}
