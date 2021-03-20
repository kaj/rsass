//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1634.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "$empty-list: ();\
            \n\
            \n@function foo($args...) {\
            \n    @return call(bar, $args...);\
            \n}\
            \n\
            \n@function bar($list) {\
            \n    @return length($list);\
            \n}\
            \n\
            \ntest {\
            \n  test: foo($empty-list);\
            \n}"
        )
        .unwrap(),
        "test {\
        \n  test: 0;\
        \n}\
        \n"
    );
}
