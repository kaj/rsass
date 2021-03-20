//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_64.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$var: 10px;\r\
            \np {\r\
            \n\twidth: -$var;\r\
            \n}"
        )
        .unwrap(),
        "p {\
        \n  width: -10px;\
        \n}\
        \n"
    );
}
