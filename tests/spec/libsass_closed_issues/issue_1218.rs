//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1218.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "$foo: 20px;\
            \n@media screen and (\"min-width:#{$foo}\") {\
            \n    .bar {\
            \n        width: 12px;\
            \n    }\
            \n}\
            \n@media screen and (\"min-width:0\") {\
            \n    .bar {\
            \n        width: 12px;\
            \n    }\
            \n}\
            \n"
        )
        .unwrap(),
        "@media screen and (min-width:20px) {\
        \n  .bar {\
        \n    width: 12px;\
        \n  }\
        \n}\
        \n@media screen and (min-width:0) {\
        \n  .bar {\
        \n    width: 12px;\
        \n  }\
        \n}\
        \n"
    );
}
