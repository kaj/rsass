//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_58.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "test {\r\
            \n  background: url(/static_loc/img/beta.png);\r\
            \n}"
        )
        .unwrap(),
        "test {\
        \n  background: url(/static_loc/img/beta.png);\
        \n}\
        \n"
    );
}
