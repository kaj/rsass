//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1021.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\r\
            \n    top: 10px - 2 * 5px /* arrow size */;\r\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  top: 0px;\
        \n}\
        \n"
    );
}
