//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1752.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin some-fn($args...) {\
            \n  @each $item in $args {\
            \n    @debug($item);\
            \n  }\
            \n}\
            \n\
            \nfoo {\
            \n  @include some-fn(());\
            \n}"
        )
        .unwrap(),
        ""
    );
}
