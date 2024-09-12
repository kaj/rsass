//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1634.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1634")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n$empty-list: ();\n\
             \n@function foo($args...) {\
             \n    @return meta.call(bar, $args...);\
             \n}\n\
             \n@function bar($list) {\
             \n    @return list.length($list);\
             \n}\n\
             \ntest {\
             \n  test: foo($empty-list);\
             \n}"),
        "test {\
         \n  test: 0;\
         \n}\n"
    );
}
