//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1437.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\r\
            \n\r\
            \n  @media screen and (min-width: 37.5em) {\r\
            \n    /* asd */\r\
            \n  }\r\
            \n\r\
            \n  @media screen and (min-width: 48em) {\r\
            \n    display: none;\r\
            \n  }\r\
            \n}"
        )
        .unwrap(),
        "@media screen and (min-width: 37.5em) {\
        \n  div {\
        \n    /* asd */\
        \n  }\
        \n}\
        \n@media screen and (min-width: 48em) {\
        \n  div {\
        \n    display: none;\
        \n  }\
        \n}\
        \n"
    );
}
