//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/import/url/control-else.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@if (false) {\r\
            \n} @else {\r\
            \n  @import url(\"http://www.libsass.org\");\r\
            \n}\r\
            \n"
        )
        .unwrap(),
        "@import url(\"http://www.libsass.org\");\
        \n"
    );
}
