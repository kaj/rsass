//! Tests auto-converted from "sass-spec/spec/libsass/selectors/simple.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  span, p, span {\
            \n    color: red;\
            \n  }\
            \n  a.foo.bar.foo {\
            \n    color: green;\
            \n  }\
            \n  &:nth(-3) {\
            \n    color: blue;\
            \n  }\
            \n}\
            \n\
            \n@-webkit-keyframes {\
            \n  from {\
            \n    left: 0px;\
            \n    10% {\
            \n      whatever: hoo;\
            \n    }\
            \n  }\
            \n  to {\
            \n    left: 200px;\
            \n  }\
            \n}\
            \n\
            \ndiv {\
            \n  @whatever {\
            \n    blah: blah;\
            \n    stuff {\
            \n      blah: bloh;\
            \n    }\
            \n  }\
            \n}\
            \n\
            \n\
            \na, b {\
            \n  color: red;\
            \n  c, d {\
            \n    height: 10px;\
            \n    e, f {\
            \n      width: 12px;\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "div span, div p, div span {\
        \n  color: red;\
        \n}\
        \ndiv a.foo.bar.foo {\
        \n  color: green;\
        \n}\
        \ndiv:nth(-3) {\
        \n  color: blue;\
        \n}\
        \n@-webkit-keyframes {\
        \n  from {\
        \n    left: 0px;\
        \n    10% {\
        \n      whatever: hoo;\
        \n    }\
        \n  }\
        \n  to {\
        \n    left: 200px;\
        \n  }\
        \n}\
        \n@whatever {\
        \n  div {\
        \n    blah: blah;\
        \n  }\
        \n  div stuff {\
        \n    blah: bloh;\
        \n  }\
        \n}\
        \na, b {\
        \n  color: red;\
        \n}\
        \na c, a d, b c, b d {\
        \n  height: 10px;\
        \n}\
        \na c e, a c f, a d e, a d f, b c e, b c f, b d e, b d f {\
        \n  width: 12px;\
        \n}\
        \n"
    );
}
