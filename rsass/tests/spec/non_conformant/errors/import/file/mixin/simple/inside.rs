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
            "@use \"sass:string\";\
             \n@mixin import-google-fonts() {\r\
             \n  @import url(\"http://fonts.googleapis.com/css?family=#{$family}\");\r\
             \n}\r\
             \nfoo {\r\
             \n  $family: string.unquote(\"Droid+Sans\");\r\
             \n  @include import-google-fonts();\r\
             \n}"
        ),
        "Error: Undefined variable.\
         \n  ,\
         \n3 |   @import url(\"http://fonts.googleapis.com/css?family=#{$family}\");\
         \n  |                                                         ^^^^^^^\
         \n  \'\
         \n  input.scss 3:57  import-google-fonts()\
         \n  input.scss 7:3   root stylesheet",
    );
}
