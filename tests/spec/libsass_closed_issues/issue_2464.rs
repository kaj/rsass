//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2464.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ":host(:not(.foo)) {\r\
            \n    left: 0;\r\
            \n}\r\
            \n\r\
            \nfoobar {\r\
            \n    :host(:not(.foo)) {\r\
            \n        left: 0;\r\
            \n    }\r\
            \n}"
        )
        .unwrap(),
        ":host(:not(.foo)) {\
        \n  left: 0;\
        \n}\
        \nfoobar :host(:not(.foo)) {\
        \n  left: 0;\
        \n}\
        \n"
    );
}
