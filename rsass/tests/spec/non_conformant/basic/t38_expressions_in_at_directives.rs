//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/38_expressions_in_at_directives.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("38_expressions_in_at_directives")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$x: 1;\
             \n$y: 2;\n\
             \n@foo $x $y, hux {\
             \n  bar {\
             \n    whatever: whatever;\
             \n  }\
             \n}\n"),
        "@foo $x $y, hux {\
         \n  bar {\
         \n    whatever: whatever;\
         \n  }\
         \n}\n"
    );
}
