//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/import/url/loop/each.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@each $i in (1) {\r\
            \n  @import url(\"http://www.libsass.org\");\r\
            \n}\r\
            \n"
        )
        .unwrap(),
        "@import url(\"http://www.libsass.org\");\
        \n"
    );
}
