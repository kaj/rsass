//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/vars.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("vars")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$x: hello;\
             \n$y: 1/2 3/4 (2+3);\n\
             \ndiv {\
             \n  content: 1 2 $x;\
             \n  content: $y;\
             \n}"),
        "div {\
         \n  content: 1 2 hello;\
         \n  content: 1/2 3/4 5;\
         \n}\n"
    );
}
