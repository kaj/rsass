//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/132_test_nested_mixin_shadow.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin bar {a: b}\
            \n\
            \nfoo {\
            \n  @mixin bar {c: d}\
            \n  @include bar;\
            \n}\
            \n\
            \nbaz {@include bar}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  c: d;\
        \n}\
        \nbaz {\
        \n  a: b;\
        \n}\
        \n"
    );
}
