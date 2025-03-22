//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1622.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1622")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\
             \n@function foo($list) {\
             \n    @return meta.call(bar, $list);\
             \n}\n\
             \n@function bar($list, $args...) {\
             \n    @return list.length($list);\
             \n}\n\
             \ntest {\
             \n  test: foo(1 2 3);\
             \n}\n"),
        "test {\
         \n  test: 3;\
         \n}\n"
    );
}
