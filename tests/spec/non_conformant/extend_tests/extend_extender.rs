//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/extend-extender.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "// For implementations like Dart Sass that process extensions as they occur,\
            \n// extending rules that contain their own extends needs special handling.\
            \n.b {@extend .a}\
            \n.c {@extend .b}\
            \n.a {x: y}\
            \n"
        )
        .unwrap(),
        ".a, .b, .c {\
        \n  x: y;\
        \n}\
        \n"
    );
}
