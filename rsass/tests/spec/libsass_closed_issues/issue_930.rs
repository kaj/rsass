//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_930.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_930")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n  &.bar {\
             \n    color: #F00;\
             \n  }\
             \n}\n\
             \n$class: \'baz\';\
             \n.foo {\
             \n  &.#{$class} {\
             \n    color: #F00;\
             \n  }\
             \n}\n\
             \n$n: 1;\
             \n.foo {\
             \n  &:nth-child(#{$n}) {\
             \n    color: #F00;\
             \n  }\
             \n}\n"),
        ".foo.bar {\
         \n  color: #F00;\
         \n}\
         \n.foo.baz {\
         \n  color: #F00;\
         \n}\
         \n.foo:nth-child(1) {\
         \n  color: #F00;\
         \n}\n"
    );
}
