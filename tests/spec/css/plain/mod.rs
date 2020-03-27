//! Tests auto-converted from "sass-spec/spec/css/plain"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/css/plain/boolean_operations.hrx"
#[test]
#[ignore] // wrong result
fn boolean_operations() {
    assert_eq!(
        rsass(
            "@import \"plain\";\
            \n"
        )
        .unwrap(),
        "a {\
        \n  and: true and false;\
        \n  or: true or false;\
        \n  not: not true;\
        \n}\
        \n"
    );
}

mod error;

// From "sass-spec/spec/css/plain/extend.hrx"
#[test]
#[ignore] // unexepected error
fn extend() {
    assert_eq!(
        rsass(
            "@import \"plain\";\
            \n\
            \na {@extend b}\
            \n"
        )
        .unwrap(),
        "b, a {\
        \n  c: d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/css/plain/functions.hrx"
mod functions {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn alpha() {
        assert_eq!(
            rsass(
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
            rsass(
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
            rsass(
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
            rsass(
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
            rsass(
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
            rsass(
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
            rsass(
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
            rsass(
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
            rsass(
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
}

// From "sass-spec/spec/css/plain/hacks.hrx"
#[test]
#[ignore] // wrong result
fn hacks() {
    assert_eq!(
        rsass(
            "@import \"plain\";\
            \n"
        )
        .unwrap(),
        ".hacks {\
        \n  *x: y;\
        \n  :x: y;\
        \n  #x: y;\
        \n  .x: y;\
        \n}\
        \n"
    );
}

mod import;

// From "sass-spec/spec/css/plain/null.hrx"
#[test]
#[ignore] // wrong result
fn null() {
    assert_eq!(
        rsass(
            "@import \"plain\";\
            \n"
        )
        .unwrap(),
        "a {\
        \n  x: null;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/css/plain/single_equals.hrx"
#[test]
#[ignore] // wrong result
fn single_equals() {
    assert_eq!(
        rsass(
            "@import \"plain\";\
            \n"
        )
        .unwrap(),
        "a {\
        \n  single-equals: alpha(opacity=65);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/css/plain/slash.hrx"
#[test]
#[ignore] // wrong result
fn slash() {
    assert_eq!(
        rsass(
            "@import \"plain\";\
            \n"
        )
        .unwrap(),
        "a {\
        \n  slash: 1/2/foo/bar;\
        \n}\
        \n"
    );
}
