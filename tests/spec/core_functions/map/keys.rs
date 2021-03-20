//! Tests auto-converted from "sass-spec/spec/core_functions/map/keys.hrx"

#[test]
fn empty() {
    assert_eq!(
        crate::rsass(
            "$result: map-keys(());\
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
            "a {b: map-keys((c: d, e: f, g: h))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c, e, g;\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: map-keys($map: (1: 2, 3: 4))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1, 3;\
        \n}\
        \n"
    );
}
#[test]
fn single() {
    assert_eq!(
        crate::rsass(
            "$result: map-keys((1: 2));\
            \na {\
            \n  value: $result;\
            \n  type: type-of($result);\
            \n  separator: list-separator($result);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  value: 1;\
        \n  type: list;\
        \n  separator: comma;\
        \n}\
        \n"
    );
}
