//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/import/file/mixin/simple/outside.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin import-google-fonts() {\r\
            \n  @import url(\"http://fonts.googleapis.com/css?family=#{$family}\");\r\
            \n}\r\
            \n$family: unquote(\"Droid+Sans\");\r\
            \n@include import-google-fonts();"
        )
        .unwrap(),
        "@import url(\"http://fonts.googleapis.com/css?family=Droid+Sans\");\
        \n"
    );
}
