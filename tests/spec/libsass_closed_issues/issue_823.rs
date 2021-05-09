//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_823.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%test {\
             \n  > {\
             \n    .red {\
             \n      color: #F00;\
             \n    }\
             \n  }\
             \n}\n\
             \np {\
             \n  @extend %test;\n\
             \n  > {\
             \n    a {\
             \n      @extend %test;\
             \n    }\
             \n  }\
             \n}\n"),
        "p > a > .red, p > .red {\
         \n  color: #F00;\
         \n}\n"
    );
}
