//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1604.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1604")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:list\";\
             \n@use \"sass:meta\";\n\
             \n@function test($args...) {\
             \n  $all: ();\n\
             \n  @each $arg in $args {\
             \n    $all: list.append($all, $arg);\
             \n  }\n\
             \n  @return meta.inspect($all);\
             \n}\n\
             \ntest {\
             \n  args-1: test(1 2 3);\
             \n  args-2: test(1 2, 3 4);\
             \n  args-3: test(1, 2, 3);\
             \n}\n"),
        "test {\
         \n  args-1: (1 2 3);\
         \n  args-2: (1 2) (3 4);\
         \n  args-3: 1 2 3;\
         \n}\n"
    );
}
