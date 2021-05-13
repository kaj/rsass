//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/04_basic_variables.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$color: \"black\";\
             \n$color: red;\
             \n$background: \"blue\";\n\
             \na {\
             \n  color: $color;\
             \n  background: $background;\
             \n}\n\
             \n$y: before;\n\
             \n$x: 1 2 $y;\n\
             \nfoo {\
             \n  a: $x;\
             \n}\n\
             \n$y: after;\n\
             \nfoo {\
             \n  a: $x;\
             \n}"),
        "a {\
         \n  color: red;\
         \n  background: \"blue\";\
         \n}\
         \nfoo {\
         \n  a: 1 2 before;\
         \n}\
         \nfoo {\
         \n  a: 1 2 before;\
         \n}\n"
    );
}
