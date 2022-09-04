//! Tests auto-converted from "sass-spec/spec/directives/use/with/multiple.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("multiple")
        .mock_file("_other.scss", "$a: original a !default;\n$b: original b !default;\n$c: original c !default;\n\nd {\n  a: $a;\n  b: $b;\n  c: $c;\n}\n")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"other\" with (\
             \n  $a: configured a,\
             \n  $b: configured b,\
             \n  $c: configured c\
             \n);\n"),
        "d {\
         \n  a: configured a;\
         \n  b: configured b;\
         \n  c: configured c;\
         \n}\n"
    );
}
