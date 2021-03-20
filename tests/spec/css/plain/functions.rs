//! Tests auto-converted from "sass-spec/spec/css/plain/functions.hrx"

#[test]
#[ignore] // wrong result
fn alpha() {
    assert_eq!(
        crate::rsass(
            "@import \"plain\";\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: alpha(0.1);\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // wrong result
fn defined_elsewhere() {
    assert_eq!(
        crate::rsass(
            "@function a() {@return b}\
            \n\
            \n@import \"plain\";\
            \n"
        )
        .unwrap(),
        "c {\
        \n  d: a();\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // wrong result
fn grayscale() {
    assert_eq!(
        crate::rsass(
            "@import \"plain\";\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: grayscale(0.1);\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // wrong result
fn hsl() {
    assert_eq!(
        crate::rsass(
            "@import \"plain\";\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: hsl(0, 100%, 50%);\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // wrong result
fn hsla() {
    assert_eq!(
        crate::rsass(
            "@import \"plain\";\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: hsla(0, 100%, 50%, 0.5);\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // wrong result
fn invert() {
    assert_eq!(
        crate::rsass(
            "@import \"plain\";\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: invert(0.1);\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // wrong result
fn rgb() {
    assert_eq!(
        crate::rsass(
            "@import \"plain\";\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgb(10, 20, 30);\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // wrong result
fn rgba() {
    assert_eq!(
        crate::rsass(
            "@import \"plain\";\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(10, 20, 30, 0.5);\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // wrong result
fn saturate() {
    assert_eq!(
        crate::rsass(
            "@import \"plain\";\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: saturate(0.1);\
        \n}\
        \n"
    );
}
