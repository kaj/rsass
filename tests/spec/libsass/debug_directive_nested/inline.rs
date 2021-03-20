//! Tests auto-converted from "sass-spec/spec/libsass/debug-directive-nested/inline.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  b: {\
            \n    @debug test;\
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
