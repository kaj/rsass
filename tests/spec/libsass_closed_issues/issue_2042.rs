//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2042.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".wizard-editor {\r\
            \n    transform: scale(50/1);\r\
            \n    transform: scale((50/1));\r\
            \n    transform: scale( (50/1) );\r\
            \n}"
        )
        .unwrap(),
        ".wizard-editor {\
        \n  transform: scale(50/1);\
        \n  transform: scale(50);\
        \n  transform: scale(50);\
        \n}\
        \n"
    );
}
