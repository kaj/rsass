//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_424.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "footer {\r\
            \n    color: red;\r\
            \n}\r\
            \n\r\
            \n// Ampersand in SassScript:\r\
            \n/*.button {\r\
            \n    &-primary {\r\
            \n        background: orange;\r\
            \n    }\r\
            \n\r\
            \n    &-secondary {\r\
            \n        background: blue;\r\
            \n    }\r\
            \n}*/\r\
            \n\r\
            \n// Output:\r\
            \n.button-primary {\r\
            \n    background: orange;\r\
            \n}\r\
            \n\r\
            \n.button-secondary {\r\
            \n    background: blue;\r\
            \n}"
        )
        .unwrap(),
        "footer {\
        \n  color: red;\
        \n}\
        \n/*.button {\
        \n    &-primary {\
        \n        background: orange;\
        \n    }\
        \n    &-secondary {\
        \n        background: blue;\
        \n    }\
        \n}*/\
        \n.button-primary {\
        \n  background: orange;\
        \n}\
        \n.button-secondary {\
        \n  background: blue;\
        \n}\
        \n"
    );
}
