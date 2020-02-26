//! Tests auto-converted from "sass-spec/spec/libsass/at-error"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass/at-error/feature-test.hrx"
#[test]
fn feature_test() {
    assert_eq!(
        rsass(
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
