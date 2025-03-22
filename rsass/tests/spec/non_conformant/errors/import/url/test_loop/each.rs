//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/import/url/loop/each.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("each")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@each $i in (1) {\r\
             \n  @import url(\"http://www.libsass.org\");\r\
             \n}\r\n"),
        "@import url(\"http://www.libsass.org\");\n"
    );
}
