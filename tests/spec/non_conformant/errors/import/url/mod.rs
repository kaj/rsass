//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/import/url"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/non_conformant/errors/import/url/control-else.hrx"
#[test]
fn control_else() {
    assert_eq!(
        rsass(
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

// From "sass-spec/spec/non_conformant/errors/import/url/control-if.hrx"
#[test]
fn control_if() {
    assert_eq!(
        rsass(
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

mod test_loop;

mod mixin;

// From "sass-spec/spec/non_conformant/errors/import/url/simple.hrx"
#[test]
fn simple() {
    assert_eq!(
        rsass(
            "@import \"hey1.css\", \"cookie.css\", url(\"hey2.css\"), \"fudge.css\";\
            \n\
            \n$foo:\"goodbye\";\
            \ndiv[name=\"hello\"] {\
            \n  color: blue;\
            \n}\
            \n\
            \n@import \"bludge.css\";"
        )
        .unwrap(),
        "@import \"hey1.css\";\
        \n@import \"cookie.css\";\
        \n@import url(\"hey2.css\");\
        \n@import \"fudge.css\";\
        \n@import \"bludge.css\";\
        \ndiv[name=hello] {\
        \n  color: blue;\
        \n}\
        \n"
    );
}
