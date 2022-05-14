//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/import/file/mixin/simple/inside.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("inside")
        .mock_file("_include.scss", "")
}

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        runner().err(
            "@mixin import-google-fonts() {\r\
             \n  @import url(\"http://fonts.googleapis.com/css?family=#{$family}\");\r\
             \n}\r\
             \nfoo {\r\
             \n  $family: unquote(\"Droid+Sans\");\r\
             \n  @include import-google-fonts();\r\
             \n}"
        ),
        "Error: Undefined variable.\
         \n  ,\
         \n2 |   @import url(\"http://fonts.googleapis.com/css?family=#{$family}\");\
         \n  |                                                         ^^^^^^^\
         \n  \'\
         \n  input.scss 2:57  import-google-fonts()\
         \n  input.scss 6:3   root stylesheet",
    );
}
