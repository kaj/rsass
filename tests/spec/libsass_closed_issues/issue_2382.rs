//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2382.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".test {\r\
            \n  font: normal normal 400 16px/calc(16px * 1.4) Oxygen;\r\
            \n}"
        )
        .unwrap(),
        ".test {\
        \n  font: normal normal 400 16px/calc(16px * 1.4) Oxygen;\
        \n}\
        \n"
    );
}
