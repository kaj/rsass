//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1063.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%foo {\
             \n  & > x { display: block; }\
             \n}\n\
             \na {\
             \n  > b { @extend %foo; }\
             \n  > b > c { @extend %foo; }\
             \n}\n"),
        "a > b > c > x, a > b > x {\
         \n  display: block;\
         \n}\n"
    );
}
