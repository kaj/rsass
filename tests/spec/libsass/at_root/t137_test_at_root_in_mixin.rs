//! Tests auto-converted from "sass-spec/spec/libsass/at-root/137_test_at_root_in_mixin.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin bar {\
            \n  @at-root .bar {a: b}\
            \n}\
            \n\
            \n.foo {\
            \n  @include bar;\
            \n}\
            \n"
        )
        .unwrap(),
        ".bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
