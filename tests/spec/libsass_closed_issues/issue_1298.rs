//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1298.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@import url(//fonts.googleapis.com/css?family=Roboto:400,500,700,400italic);\
            \nhtml {\
            \n  font-family: roboto, arial, helvetica, sans-serif;\
            \n}\
            \n"
        )
        .unwrap(),
        "@import url(//fonts.googleapis.com/css?family=Roboto:400,500,700,400italic);\
        \nhtml {\
        \n  font-family: roboto, arial, helvetica, sans-serif;\
        \n}\
        \n"
    );
}
