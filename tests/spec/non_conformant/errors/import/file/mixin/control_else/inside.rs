//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/import/file/mixin/control-else/inside.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("inside")
        .mock_file("_include.scss", "")
}

#[test]
#[ignore] // missing error
fn test() {
    assert_eq!(
        runner().err(
            "@mixin do_import() {\r\
             \n  @if (false) {\r\
             \n  } @else {\r\
             \n    @import \'_include\';\r\
             \n  }\r\
             \n}\r\
             \n\r\
             \nfoo {\r\
             \n  @include do_import();\r\
             \n}"
        ),
        "Error: This at-rule is not allowed here.\
         \n  ,\
         \n4 |     @import \'_include\';\
         \n  |     ^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 4:5  root stylesheet",
    );
}
