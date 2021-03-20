//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_346.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$mediaquery: \'and (min-width: 300px)\';\
            \n\
            \n@media all #{$mediaquery} {\
            \n  div {\
            \n    display: block;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@media all and (min-width: 300px) {\
        \n  div {\
        \n    display: block;\
        \n  }\
        \n}\
        \n"
    );
}
