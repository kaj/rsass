//! Tests auto-converted from "sass-spec/spec/libsass-todo-tests/errors/import/url/mixin/control-else/inside.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
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
