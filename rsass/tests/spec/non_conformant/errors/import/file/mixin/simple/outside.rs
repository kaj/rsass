//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/import/file/mixin/simple/outside.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("outside")
        .mock_file("_include.scss", "")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n@mixin import-google-fonts() {\r\
             \n  @import url(\"http://fonts.googleapis.com/css?family=#{$family}\");\r\
             \n}\r\
             \n$family: string.unquote(\"Droid+Sans\");\r\
             \n@include import-google-fonts();"
        ),
        "@import url(\"http://fonts.googleapis.com/css?family=Droid+Sans\");\n"
    );
}
