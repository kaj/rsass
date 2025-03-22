//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/import/url/control-else.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("control-else")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@if (false) {\r\
             \n} @else {\r\
             \n  @import url(\"http://www.libsass.org\");\r\
             \n}\r\n"),
        "@import url(\"http://www.libsass.org\");\n"
    );
}
