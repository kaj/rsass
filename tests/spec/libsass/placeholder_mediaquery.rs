//! Tests auto-converted from "sass-spec/spec/libsass/placeholder-mediaquery.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "%foo {\
            \n\t@media screen and (min-width: 300px) {\
            \n\t\tmax-width: 80%;\
            \n\t}\
            \n}\
            \n\
            \nbar {\
            \n\t@extend %foo;\
            \n}\
            \n"
        )
        .unwrap(),
        "@media screen and (min-width: 300px) {\
        \n  bar {\
        \n    max-width: 80%;\
        \n  }\
        \n}\
        \n"
    );
}
