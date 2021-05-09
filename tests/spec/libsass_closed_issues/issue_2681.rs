//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue-2681.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%button-styles {\
             \n  color: red;\n\
             \n  &:focus {\
             \n    color: blue;\
             \n  }\
             \n}\n\
             \n[type=\"button\"] {\
             \n  @extend %button-styles;\
             \n}\n\n"),
        "[type=button] {\
         \n  color: red;\
         \n}\
         \n[type=button]:focus {\
         \n  color: blue;\
         \n}\n"
    );
}
