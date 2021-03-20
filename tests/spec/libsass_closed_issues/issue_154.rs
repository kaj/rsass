//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_154.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "test {\r\
            \n  filter:alpha(opacity=75);\r\
            \n}"
        )
        .unwrap(),
        "test {\
        \n  filter: alpha(opacity=75);\
        \n}\
        \n"
    );
}
