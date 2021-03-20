//! Tests auto-converted from "sass-spec/spec/libsass/unary-ops.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "$x: 20%;\
            \n\
            \ndiv {\
            \n  a: -10;\
            \n  b: -10px + 10px;\
            \n  c: +10;\
            \n  d: +10px + -10px;\
            \n  e: -$x;\
            \n  f: +$x;\
            \n  g: -hello;\
            \n  h: +hello;\
            \n  i: + hello;\
            \n  j: type-of(+ hello);\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  a: -10;\
        \n  b: 0px;\
        \n  c: 10;\
        \n  d: 0px;\
        \n  e: -20%;\
        \n  f: 20%;\
        \n  g: -hello;\
        \n  h: +hello;\
        \n  i: +hello;\
        \n  j: string;\
        \n}\
        \n"
    );
}
