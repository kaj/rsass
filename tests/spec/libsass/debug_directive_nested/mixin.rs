//! Tests auto-converted from "sass-spec/spec/libsass/debug-directive-nested/mixin.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin c() {\
            \n  @warn test;\
            \n  c: d;\
            \n}\
            \n\
            \na {\
            \n  b: {\
            \n    @include c();\
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
