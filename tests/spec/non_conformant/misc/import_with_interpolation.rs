//! Tests auto-converted from "sass-spec/spec/non_conformant/misc/import_with_interpolation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "$family: unquote(\"Droid+Sans\");\
             \n@import url(\"http://fonts.googleapis.com/css?family=#{$family}\");\n"
        ),
        "@import url(\"http://fonts.googleapis.com/css?family=Droid+Sans\");\n"
    );
}
