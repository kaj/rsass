//! Tests auto-converted from "sass-spec/spec/libsass/propsets.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "$x: ground;\
            \n$y: e;\
            \n$z: it;\
            \n\
            \ndiv {\
            \n  back#{$x}: {\
            \n    imag#{$y}: url(foo.png);\
            \n    pos#{$z}ion: 50%;\
            \n  }\
            \n}\
            \n\
            \nspan {\
            \n  background: {\
            \n    image: url(bar.png);\
            \n    position: 100%;\
            \n  }\
            \n}\
            \n\
            \np {\
            \n  border: {\
            \n    upper: {\
            \n      left: 2px;\
            \n      right: 3px;\
            \n    }\
            \n  }\
            \n}\
            \n\
            \n@warn 2 + 3;\
            \n\
            \n/* 2 + 3 */\
            \n/* #{2 + 3} */\
            \n\
            \nfoo|div {\
            \n  color: red;\
            \n}\
            \n\
            \n$-hey : hey;\
            \n\
            \ndiv sp\\ ,#abcan {\
            \n  color: red;\
            \n  p, |q {\
            \n    background: blue;\
            \n    color: \\hey;\
            \n    width: \\10 + \\20 \\ ;\
            \n    a {\
            \n      height: 1;\
            \n    }\
            \n  }\
            \n}\
            \n\
            \nd\\ v, sp\\ n {\
            \n  a {\
            \n    color: blue;\
            \n  }\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  background-image: url(foo.png);\
        \n  background-position: 50%;\
        \n}\
        \nspan {\
        \n  background-image: url(bar.png);\
        \n  background-position: 100%;\
        \n}\
        \np {\
        \n  border-upper-left: 2px;\
        \n  border-upper-right: 3px;\
        \n}\
        \n/* 2 + 3 */\
        \n/* 5 */\
        \nfoo|div {\
        \n  color: red;\
        \n}\
        \ndiv sp\\ , #abcan {\
        \n  color: red;\
        \n}\
        \ndiv sp\\  p, div sp\\  |q, #abcan p, #abcan |q {\
        \n  background: blue;\
        \n  color: hey;\
        \n  width: \\10 \\ \\ ;\
        \n}\
        \ndiv sp\\  p a, div sp\\  |q a, #abcan p a, #abcan |q a {\
        \n  height: 1;\
        \n}\
        \nd\\ v a, sp\\ n a {\
        \n  color: blue;\
        \n}\
        \n"
    );
}
