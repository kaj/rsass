//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/028_test_import_with_interpolation.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("028_test_import_with_interpolation")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n$family: string.unquote(\"Droid+Sans\");\
             \n@import url(\"http://fonts.googleapis.com/css?family=#{$family}\");\n"
        ),
        "@import url(\"http://fonts.googleapis.com/css?family=Droid+Sans\");\n"
    );
}
