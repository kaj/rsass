//! Tests auto-converted from "sass-spec/spec/libsass/rel.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  less: 3px < 3pt;\
            \n  less: (1px / 1pt);\
            \n  less: 23fu < 120;\
            \n  eq: hello == hello;\
            \n  eq: \"hello\" == hello;\
            \n  eq: (1 2 3) == (1 2 3);\
            \n  eq: (1 2 3) == (1, 2, 3);\
            \n  eq: 23px == 23fu;\
            \n  eq: 3.1in == 2.54cm;\
            \n  eq: 2.54cm == 3.1in;\
            \n  eq: (1in) == (1cm*1in/1cm);\
            \n  x: 1in, (1cm*1in/1cm);\
            \n  y: 1cm*1in/1in;\
            \n  eq: (2cm*1in/2cm) == (1in*2cm/2cm);\
            \n  blah: (1cm/1in);\
            \n  in: (1in*2.54cm/1in);\
            \n  lt: 1in < 2.54cm;\
            \n  lt: 2.54cm < 1in;\
            \n  lt: 5 < 4;\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  less: true;\
        \n  less: 0.75;\
        \n  less: true;\
        \n  eq: true;\
        \n  eq: true;\
        \n  eq: true;\
        \n  eq: false;\
        \n  eq: false;\
        \n  eq: false;\
        \n  eq: false;\
        \n  eq: true;\
        \n  x: 1in, 1in;\
        \n  y: 0.3937007874in;\
        \n  eq: true;\
        \n  blah: 0.3937007874;\
        \n  in: 2.54cm;\
        \n  lt: false;\
        \n  lt: false;\
        \n  lt: false;\
        \n}\
        \n"
    );
}
