//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2149.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo {\
            \n  background: url(\'background.png\') -10px -10px/110% no-repeat\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  background: url(\"background.png\") -10px -10px/110% no-repeat;\
        \n}\
        \n"
    );
}
