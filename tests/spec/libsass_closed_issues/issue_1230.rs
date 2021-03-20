//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1230.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  transition-property:\
            \n    border-color,\
            \n    box-shadow,\
            \n    color;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  transition-property: border-color, box-shadow, color;\
        \n}\
        \n"
    );
}
