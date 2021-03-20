//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_485.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@media not all and (monochrome) { a {foo: bar} }\
            \n@media not screen and (color), print and (color) { a {foo: bar} }\
            \n@media (not (screen and (color))), print and (color) { a {foo: bar} }\
            \n"
        )
        .unwrap(),
        "@media not all and (monochrome) {\
        \n  a {\
        \n    foo: bar;\
        \n  }\
        \n}\
        \n@media not screen and (color), print and (color) {\
        \n  a {\
        \n    foo: bar;\
        \n  }\
        \n}\
        \n@media (false), print and (color) {\
        \n  a {\
        \n    foo: bar;\
        \n  }\
        \n}\
        \n"
    );
}
