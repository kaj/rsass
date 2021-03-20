//! Tests auto-converted from "sass-spec/spec/libsass/css_nth_selectors.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ":nth-child(2n + 3) {\
            \n  outer-whitespace: false;\
            \n}\
            \n\
            \n// Regression test for sass/dart-sass#465.\
            \n:nth-child( 2n + 3 ) {\
            \n  outer-whitespace: true;\
            \n}\
            \n"
        )
        .unwrap(),
        ":nth-child(2n+3) {\
        \n  outer-whitespace: false;\
        \n}\
        \n:nth-child(2n+3) {\
        \n  outer-whitespace: true;\
        \n}\
        \n"
    );
}
