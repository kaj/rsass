//! Tests auto-converted from "sass-spec/spec/non_conformant/misc/import_in_mixin.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin import-google-fonts() {\
            \n  @import url(\"http://fonts.googleapis.com/css?family=#{$family}\");\
            \n}\
            \n$family: unquote(\"Droid+Sans\");\
            \n@include import-google-fonts();\
            \n"
        )
        .unwrap(),
        "@import url(\"http://fonts.googleapis.com/css?family=Droid+Sans\");\
        \n"
    );
}
