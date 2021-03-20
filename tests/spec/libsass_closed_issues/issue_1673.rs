//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1673.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "%foo {\
            \n  test: outer;\
            \n\
            \n  &-inner {\
            \n    test: inner;\
            \n  }\
            \n}\
            \n\
            \n.foo {\
            \n  @extend %foo;\
            \n\
            \n  &.inner { @extend %foo-inner; }\
            \n}"
        )
        .unwrap(),
        ".foo {\
        \n  test: outer;\
        \n}\
        \n.foo.inner {\
        \n  test: inner;\
        \n}\
        \n"
    );
}
