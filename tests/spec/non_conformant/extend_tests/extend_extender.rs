//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/extend-extender.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("extend-extender")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(
            "// For implementations like Dart Sass that process extensions as they occur,\
             \n// extending rules that contain their own extends needs special handling.\
             \n.b {@extend .a}\
             \n.c {@extend .b}\
             \n.a {x: y}\n"
        ),
        ".a, .b, .c {\
         \n  x: y;\
         \n}\n"
    );
}
