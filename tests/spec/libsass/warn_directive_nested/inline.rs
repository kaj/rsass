//! Tests auto-converted from "sass-spec/spec/libsass/warn-directive-nested/inline.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  b: {\
            \n    @warn test;\
            \n    c: d;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b-c: d;\
        \n}\
        \n"
    );
}
