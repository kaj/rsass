//! Tests auto-converted from "sass-spec/spec/libsass/variable-scoping/feature-test.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@if feature-exists(global-variable-shadowing) {\
            \n  div {\
            \n    feature: true;\
            \n  }\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  feature: true;\
        \n}\
        \n"
    );
}
