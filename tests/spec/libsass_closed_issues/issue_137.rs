//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_137.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n  background-color: lime;\
             \n  a {\
             \n    color: white;\
             \n  }\
             \n}\n\
             \n.baz {\
             \n  @extend .foo;\
             \n}"),
        ".foo, .baz {\
         \n  background-color: lime;\
         \n}\
         \n.foo a, .baz a {\
         \n  color: white;\
         \n}\n"
    );
}
