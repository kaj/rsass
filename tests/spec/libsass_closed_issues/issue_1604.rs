//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1604.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@function test($args...) {\
             \n  $all: ();\n\
             \n  @each $arg in $args {\
             \n    $all: append($all, $arg);\
             \n  }\n\
             \n  @return inspect($all);\
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
