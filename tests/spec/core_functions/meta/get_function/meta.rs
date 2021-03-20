//! Tests auto-converted from "sass-spec/spec/core_functions/meta/get_function/meta.hrx"

#[test]
fn inspect() {
    assert_eq!(
        crate::rsass(
            "a {b: inspect(get-function(lighten))};\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: get-function(\"lighten\");\
        \n}\
        \n"
    );
}
#[test]
fn type_of() {
    assert_eq!(
        crate::rsass(
            "a {b: type-of(get-function(lighten))};\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: function;\
        \n}\
        \n"
    );
}
