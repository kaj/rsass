//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1216.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  width: 4.0px;\
            \n  height: 3.00px;\
            \n  opacity: 1.0;\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  width: 4px;\
        \n  height: 3px;\
        \n  opacity: 1;\
        \n}\
        \n"
    );
}
