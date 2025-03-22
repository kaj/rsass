//! Tests auto-converted from "sass-spec/spec/libsass-todo-tests/errors/import/url/mixin/control-else/inside.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("inside")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@mixin do_import() {\r\
             \n  @if (false) {\r\
             \n  } @else {\r\
             \n    @import url(\"http://www.libsass.org\");\r\
             \n  }\r\
             \n}\r\
             \n\r\
             \nfoo {\r\
             \n  @include do_import();\r\
             \n}"),
        "foo {\
         \n  @import url(\"http://www.libsass.org\");\
         \n}\n"
    );
}
