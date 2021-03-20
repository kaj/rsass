//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1604.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@function test($args...) {\
            \n  $all: ();\
            \n\
            \n  @each $arg in $args {\
            \n    $all: append($all, $arg);\
            \n  }\
            \n\
            \n  @return inspect($all);\
            \n}\
            \n\
            \ntest {\
            \n  args-1: test(1 2 3);\
            \n  args-2: test(1 2, 3 4);\
            \n  args-3: test(1, 2, 3);\
            \n}\
            \n"
        )
        .unwrap(),
        "test {\
        \n  args-1: (1 2 3);\
        \n  args-2: (1 2) (3 4);\
        \n  args-3: 1 2 3;\
        \n}\
        \n"
    );
}
