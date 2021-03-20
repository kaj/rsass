//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2031/wrapped-not.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ":not(.asd, .qwe) {\r\
            \n  content: test;\r\
            \n}"
        )
        .unwrap(),
        ":not(.asd, .qwe) {\
        \n  content: test;\
        \n}\
        \n"
    );
}
