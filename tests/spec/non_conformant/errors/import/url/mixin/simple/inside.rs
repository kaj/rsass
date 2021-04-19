//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/import/url/mixin/simple/inside.hrx"

#[test]
#[ignore] // wrong error
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin import-google-fonts() {\r\
             \n  @import url(\"http://fonts.googleapis.com/css?family=#{$family}\");\r\
             \n}\r\
             \nfoo {\r\
             \n  $family: unquote(\"Droid+Sans\");\r\
             \n  @include import-google-fonts();\r\
             \n}"
        ).unwrap_err(),
        "Error: Undefined variable.\
         \n  ,\
         \n2 |   @import url(\"http://fonts.googleapis.com/css?family=#{$family}\");\
         \n  |                                                         ^^^^^^^\
         \n  \'\
         \n  input.scss 2:57  import-google-fonts()\
         \n  input.scss 6:3   root stylesheet\
         \n",
    );
}
