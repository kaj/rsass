//! Tests auto-converted from "sass-spec/spec/libsass/css_unicode.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("css_unicode")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@charset \"UTF-8\";\
             \nfoo {\
             \n  bar: föö bâr; }\n"),
        "@charset \"UTF-8\";\
         \nfoo {\
         \n  bar: föö bâr;\
         \n}\n"
    );
}
