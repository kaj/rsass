//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/131_test_nested_mixin_def.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  @mixin bar {a: b}\
            \n  @include bar; }\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}
