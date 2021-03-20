//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_623.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  filter: alpha(opacity=.3); }\
            \n\
            \ndiv {\
            \n  filter: alpha(opacity=0.7); }\
            \n"
        )
        .unwrap(),
        "a {\
        \n  filter: alpha(opacity=0.3);\
        \n}\
        \ndiv {\
        \n  filter: alpha(opacity=0.7);\
        \n}\
        \n"
    );
}
