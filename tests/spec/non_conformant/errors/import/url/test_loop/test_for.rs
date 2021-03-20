//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/import/url/loop/for.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@for $i from 1 through 2 {\r\
            \n  @import url(\"http://www.libsass.org\");\r\
            \n}\r\
            \n"
        )
        .unwrap(),
        "@import url(\"http://www.libsass.org\");\
        \n@import url(\"http://www.libsass.org\");\
        \n"
    );
}
