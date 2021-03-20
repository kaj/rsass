//! Tests auto-converted from "sass-spec/spec/values/maps/map-values.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  foo: map-values((foo: 1, bar: 2));\
            \n  foo: map-values((foo: 1, bar: 2, baz: 2));\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  foo: 1, 2;\
        \n  foo: 1, 2, 2;\
        \n}\
        \n"
    );
}
