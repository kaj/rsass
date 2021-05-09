//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/while_directive.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$i: 1;\n\
             \n.foo {\
             \n  @while $i != 5 {\
             \n    a: $i;\
             \n    $i: $i + 1;\
             \n  }\
             \n}\n"),
        ".foo {\
         \n  a: 1;\
         \n  a: 2;\
         \n  a: 3;\
         \n  a: 4;\
         \n}\n"
    );
}
