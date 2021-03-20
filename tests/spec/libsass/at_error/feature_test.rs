//! Tests auto-converted from "sass-spec/spec/libsass/at-error/feature-test.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@if feature-exists(at-error) {\
            \n  div {\
            \n    feature: true;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  feature: true;\
        \n}\
        \n"
    );
}
