//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2429.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "input[type=url] {\r\
            \n  content: bar\r\
            \n}"
        )
        .unwrap(),
        "input[type=url] {\
        \n  content: bar;\
        \n}\
        \n"
    );
}
