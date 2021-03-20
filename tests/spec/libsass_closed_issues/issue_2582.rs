//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2582.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".test {\r\
            \n  font-size: (16px / 16px) + 0em;\r\
            \n  font-size: (16px / 16px  + 0em);\r\
            \n  font-size: 16px / 16px  + 0em;\r\
            \n}"
        )
        .unwrap(),
        ".test {\
        \n  font-size: 1em;\
        \n  font-size: 1em;\
        \n  font-size: 1em;\
        \n}\
        \n"
    );
}
