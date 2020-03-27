//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/import/url/loop"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/non_conformant/errors/import/url/loop/each.hrx"
#[test]
fn each() {
    assert_eq!(
        rsass(
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

// From "sass-spec/spec/non_conformant/errors/import/url/loop/for.hrx"
#[test]
fn test_for() {
    assert_eq!(
        rsass(
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

// From "sass-spec/spec/non_conformant/errors/import/url/loop/while.hrx"
#[test]
fn test_while() {
    assert_eq!(
        rsass(
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
