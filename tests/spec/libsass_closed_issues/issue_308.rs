//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_308.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$var: orange;\
            \n\
            \n.test {\
            \n  color: $var;\
            \n}\
            \n\
            \n.#{$var} {\
            \n  color: #C0362C;\
            \n}\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  color: orange;\
        \n}\
        \n.orange {\
        \n  color: #C0362C;\
        \n}\
        \n"
    );
}
