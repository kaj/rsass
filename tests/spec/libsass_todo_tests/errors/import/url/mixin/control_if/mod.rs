//! Tests auto-converted from "sass-spec/spec/libsass-todo-tests/errors/import/url/mixin/control-if"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/libsass-todo-tests/errors/import/url/mixin/control-if/inside.hrx"
#[test]
#[ignore] // wrong result
fn inside() {
    assert_eq!(
        rsass(
            "@mixin do_import() {\r\
            \n  @if (true) {\r\
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
