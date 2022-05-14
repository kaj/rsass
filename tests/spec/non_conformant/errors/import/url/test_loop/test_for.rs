//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/import/url/loop/for.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("for")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@for $i from 1 through 2 {\r\
             \n  @import url(\"http://www.libsass.org\");\r\
             \n}\r\n"),
        "@import url(\"http://www.libsass.org\");\
         \n@import url(\"http://www.libsass.org\");\n"
    );
}
