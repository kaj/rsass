//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2006.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin main() {\
            \n  bar {\
            \n    baz: 1;\
            \n  }\
            \n}\
            \n\
            \nfoo {\
            \n  @at-root {\
            \n    @include main();\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "bar {\
        \n  baz: 1;\
        \n}\
        \n"
    );
}
