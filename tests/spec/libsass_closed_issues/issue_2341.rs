//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2341.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@function aFunction() {\r\
            \n    @return 1em;\r\
            \n}\r\
            \n\r\
            \n@media (max-width: 1em) {\r\
            \n    %placeholder {\r\
            \n        color: red;\r\
            \n    }\r\
            \n}\r\
            \n\r\
            \n@media (max-width: aFunction()) {\r\
            \n    .test {\r\
            \n        @extend %placeholder;\r\
            \n    }\r\
            \n}"
        )
        .unwrap(),
        "@media (max-width: 1em) {\
        \n  .test {\
        \n    color: red;\
        \n  }\
        \n}\
        \n"
    );
}
