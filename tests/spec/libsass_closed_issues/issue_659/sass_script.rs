//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_659/sass-script.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$foo: null;\
            \n\
            \n@mixin bar() {\
            \n   bar: $foo;\
            \n}\
            \n\
            \n@mixin baz() {\
            \n   baz: $foo !important;\
            \n}\
            \n\
            \nfoo {\
            \n  baz: $foo;\
            \n}\
            \n\
            \nbar {\
            \n  @include bar;\
            \n}\
            \n\
            \nbaz {\
            \n  @include baz;\
            \n}\
            \n"
        )
        .unwrap(),
        "baz {\
        \n  baz: !important;\
        \n}\
        \n"
    );
}
