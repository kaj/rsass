//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/import/url/control-if.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@if (true) {\r\
            \n  @import url(\"http://www.libsass.org\");\r\
            \n}\r\
            \n"
        )
        .unwrap(),
        "@import url(\"http://www.libsass.org\");\
        \n"
    );
}
