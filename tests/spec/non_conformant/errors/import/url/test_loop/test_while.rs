//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/import/url/loop/while.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$count: 0;\r\
            \n@while ($count < 1) {\r\
            \n  @import url(\"http://www.libsass.org\");\r\
            \n  $count: $count + 1;\r\
            \n}\r\
            \n"
        )
        .unwrap(),
        "@import url(\"http://www.libsass.org\");\
        \n"
    );
}
