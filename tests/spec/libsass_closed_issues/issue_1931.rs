//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1931.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$var: \'http://test.com\';\
            \nbody {\
            \n  background-image: url( #{$var});\
            \n}"
        )
        .unwrap(),
        "body {\
        \n  background-image: url(http://test.com);\
        \n}\
        \n"
    );
}
