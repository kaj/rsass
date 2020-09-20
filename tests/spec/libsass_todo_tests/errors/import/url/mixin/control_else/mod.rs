//! Tests auto-converted from "sass-spec/spec/libsass-todo-tests/errors/import/url/mixin/control-else"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-todo-tests/errors/import/url/mixin/control-else/inside.hrx"
#[test]
#[ignore] // wrong result
fn inside() {
    assert_eq!(
        rsass(
            "@mixin do_import() {\r\
            \n  @if (false) {\r\
            \n  } @else {\r\
            \n    @import url(\"http://www.libsass.org\");\r\
            \n  }\r\
            \n}\r\
            \n\r\
            \nfoo {\r\
            \n  @include do_import();\r\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  @import url(\"http://www.libsass.org\");\
        \n}\
        \n"
    );
}
