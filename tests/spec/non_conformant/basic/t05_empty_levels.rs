//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/05_empty_levels.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  span {\
            \n    color: red;\
            \n    background: blue;\
            \n  }\
            \n}\
            \n\
            \ndiv {\
            \n  color: gray;\
            \n  empty {\
            \n    span {\
            \n      color: red;\
            \n      background: blue;\
            \n    }\
            \n  }\
            \n}\
            \n\
            \nempty1 {\
            \n  empty2 {\
            \n    div {\
            \n      blah: blah;\
            \n    }\
            \n  }\
            \n}\
            \n\
            \nempty1 {\
            \n  empty2 {\
            \n    div {\
            \n      bloo: blee;\
            \n      empty3 {\
            \n        span {\
            \n          blah: blah;\
            \n          blah: blah;\
            \n        }\
            \n      }\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "div span {\
        \n  color: red;\
        \n  background: blue;\
        \n}\
        \ndiv {\
        \n  color: gray;\
        \n}\
        \ndiv empty span {\
        \n  color: red;\
        \n  background: blue;\
        \n}\
        \nempty1 empty2 div {\
        \n  blah: blah;\
        \n}\
        \nempty1 empty2 div {\
        \n  bloo: blee;\
        \n}\
        \nempty1 empty2 div empty3 span {\
        \n  blah: blah;\
        \n  blah: blah;\
        \n}\
        \n"
    );
}
