//! Tests auto-converted from "sass-spec/spec/non_conformant/misc/import_in_mixin.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("import_in_mixin")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n@mixin import-google-fonts() {\
             \n  @import url(\"http://fonts.googleapis.com/css?family=#{$family}\");\
             \n}\
             \n$family: string.unquote(\"Droid+Sans\");\
             \n@include import-google-fonts();\n"
        ),
        "@import url(\"http://fonts.googleapis.com/css?family=Droid+Sans\");\n"
    );
}
