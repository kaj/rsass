//! Tests auto-converted from "sass-spec/spec/core_functions/map/values.hrx"

#[test]
fn empty() {
    assert_eq!(
        crate::rsass(
            "$result: map-values(());\
            \na {\
            \n  value: inspect($result);\
            \n  separator: list-separator($result);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  value: ();\
        \n  separator: comma;\
        \n}\
        \n"
    );
}
mod error {

    // Ignoring "too_few_args", error tests are not supported yet.

    // Ignoring "too_many_args", error tests are not supported yet.

    // Ignoring "test_type", error tests are not supported yet.
}
#[test]
fn multiple() {
    assert_eq!(
        crate::rsass(
            "a {b: map-values((c: d, e: f, g: h))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: d, f, h;\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: map-values($map: (1: 2, 3: 4))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 2, 4;\
        \n}\
        \n"
    );
}
#[test]
fn single() {
    assert_eq!(
        crate::rsass(
            "$result: map-values((1: 2));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n  separator: list-separator($result);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  value: 2;\
        \n  type: list;\
        \n  separator: comma;\
        \n}\
        \n"
    );
}
