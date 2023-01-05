//! Tests auto-converted from "sass-spec/spec/directives/forward/with/facade_contains_multiple_configured_forwards.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("facade_contains_multiple_configured_forwards")
        .mock_file("_left.scss", "$a: original a !default;\na {a: $a}\n")
        .mock_file("_midstream.scss", "@forward \"left\" with ($a: configured a !default);\n@forward \"right\" with ($b: configured b !default);\n\n$c: original c !default;\nc {c: $c}\n")
        .mock_file("_right.scss", "$b: original b !default;\nb {b: $b}\n")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("// Regression test for sass/dart-sass/#1343.\
             \n@use \"midstream\" with (\
             \n  $a: twice-configured a,\
             \n  $b: twice-configured b,\
             \n  $c: configured c,\
             \n);\n"),
        "a {\
         \n  a: twice-configured a;\
         \n}\
         \nb {\
         \n  b: twice-configured b;\
         \n}\
         \nc {\
         \n  c: configured c;\
         \n}\n"
    );
}
