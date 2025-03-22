//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/import/url/control-if.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("control-if")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@if (true) {\r\
             \n  @import url(\"http://www.libsass.org\");\r\
             \n}\r\n"),
        "@import url(\"http://www.libsass.org\");\n"
    );
}
