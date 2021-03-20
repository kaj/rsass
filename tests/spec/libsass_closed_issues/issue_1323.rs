//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1323.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@import url(foo.css) only screen;\
            \n@import url(foo.css) (min-width:400px);\
            \n@import url(foo.css) (min-width:400px) and (max-width:599px);\
            \n"
        )
        .unwrap(),
        "@import url(foo.css) only screen;\
        \n@import url(foo.css) (min-width: 400px);\
        \n@import url(foo.css) (min-width: 400px) and (max-width: 599px);\
        \n"
    );
}
