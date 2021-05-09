//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_592.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%a::-webkit-scrollbar {\
             \n    color: green;\
             \n}\n\
             \n.a {\
             \n    .b {\
             \n        @extend %a;\
             \n    }\n\
             \n    .c .b {\
             \n        @extend %a;\
             \n    }\
             \n}\n"),
        ".a .c .b::-webkit-scrollbar, .a .b::-webkit-scrollbar {\
         \n  color: green;\
         \n}\n"
    );
}
