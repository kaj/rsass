//! Tests auto-converted from "sass-spec/spec/libsass/debug-directive-nested/function.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function c() {\
            \n  @warn test;\
            \n  @return d;\
            \n}\
            \n\
            \na {\
            \n  b: {\
            \n    c: c();\
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
