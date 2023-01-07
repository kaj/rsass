//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/multiple-operators.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("multiple-operators")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$x: 2;\
             \n$y: 1;\n\
             \n@function getResult() { @return true; }\n\
             \n.test {\
             \n    a: $x > $y == getResult();\
             \n}\n"),
        ".test {\
         \n  a: true;\
         \n}\n"
    );
}
