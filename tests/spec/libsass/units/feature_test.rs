//! Tests auto-converted from "sass-spec/spec/libsass/units/feature-test.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@if feature-exists(units-level-3) {\
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
