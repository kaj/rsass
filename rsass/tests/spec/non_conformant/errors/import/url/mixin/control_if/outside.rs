//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/import/url/mixin/control-if/outside.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("outside")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin do_import() {\r\
             \n  @import url(\"http://www.libsass.org\");\r\
             \n}\r\
             \n\r\
             \n@if (true) {\r\
             \n  @include do_import();\r\
             \n}"),
        "@import url(\"http://www.libsass.org\");\n"
    );
}
