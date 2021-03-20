//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/extend-loop.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "// Make sure extend loops are handled correctly. Test in all different orderings\
            \n// so we can be sure this works for implementations like Dart Sass where extend\
            \n// order matters.\
            \n\
            \n@media order1 {\
            \n  .x1.y1.a1 {x: y; @extend .b1}\
            \n  .z1.b1 {x: y; @extend .c1}\
            \n  .c1 {x: y; @extend .a1}\
            \n}\
            \n\
            \n@media order2 {\
            \n  .x2.y2.a2 {x: y; @extend .b2}\
            \n  .c2 {x: y; @extend .a2}\
            \n  .z2.b2 {x: y; @extend .c2}\
            \n}\
            \n\
            \n@media order3 {\
            \n  .z3.b3 {x: y; @extend .c3}\
            \n  .x3.y3.a3 {x: y; @extend .b3}\
            \n  .c3 {x: y; @extend .a3}\
            \n}\
            \n\
            \n@media order4 {\
            \n  .z4.b4 {x: y; @extend .c4}\
            \n  .c4 {x: y; @extend .a4}\
            \n  .x4.y4.a4 {x: y; @extend .b4}\
            \n}\
            \n\
            \n@media order5 {\
            \n  .c5 {x: y; @extend .a5}\
            \n  .z5.b5 {x: y; @extend .c5}\
            \n  .x5.y5.a5 {x: y; @extend .b5}\
            \n}\
            \n\
            \n@media order6 {\
            \n  .c6 {x: y; @extend .a6}\
            \n  .x6.y6.a6 {x: y; @extend .b6}\
            \n  .z6.b6 {x: y; @extend .c6}\
            \n}\
            \n"
        )
        .unwrap(),
        "@media order1 {\
        \n  .x1.y1.a1, .x1.y1.c1, .x1.y1.z1.b1 {\
        \n    x: y;\
        \n  }\
        \n  .z1.b1, .z1.x1.y1.a1, .z1.x1.y1.c1, .z1.x1.y1.b1 {\
        \n    x: y;\
        \n  }\
        \n  .c1, .z1.b1, .z1.x1.y1.a1, .z1.x1.y1.c1, .z1.x1.y1.b1 {\
        \n    x: y;\
        \n  }\
        \n}\
        \n@media order2 {\
        \n  .x2.y2.a2, .x2.y2.c2, .x2.y2.z2.b2 {\
        \n    x: y;\
        \n  }\
        \n  .c2, .z2.b2, .z2.x2.y2.a2, .z2.x2.y2.c2, .z2.x2.y2.b2 {\
        \n    x: y;\
        \n  }\
        \n  .z2.b2, .z2.x2.y2.a2, .z2.x2.y2.c2, .z2.x2.y2.b2 {\
        \n    x: y;\
        \n  }\
        \n}\
        \n@media order3 {\
        \n  .z3.b3, .z3.x3.y3.a3, .z3.x3.y3.c3, .z3.x3.y3.b3 {\
        \n    x: y;\
        \n  }\
        \n  .x3.y3.a3, .x3.y3.c3, .x3.y3.z3.b3 {\
        \n    x: y;\
        \n  }\
        \n  .c3, .z3.b3, .z3.x3.y3.a3, .z3.x3.y3.c3, .z3.x3.y3.b3 {\
        \n    x: y;\
        \n  }\
        \n}\
        \n@media order4 {\
        \n  .z4.b4, .z4.x4.y4.a4, .z4.x4.y4.c4, .z4.x4.y4.b4 {\
        \n    x: y;\
        \n  }\
        \n  .c4, .z4.b4, .z4.x4.y4.a4, .z4.x4.y4.c4, .z4.x4.y4.b4 {\
        \n    x: y;\
        \n  }\
        \n  .x4.y4.a4, .x4.y4.c4, .x4.y4.z4.b4 {\
        \n    x: y;\
        \n  }\
        \n}\
        \n@media order5 {\
        \n  .c5, .z5.b5, .z5.x5.y5.a5, .z5.x5.y5.c5, .z5.x5.y5.b5 {\
        \n    x: y;\
        \n  }\
        \n  .z5.b5, .z5.x5.y5.a5, .z5.x5.y5.c5, .z5.x5.y5.b5 {\
        \n    x: y;\
        \n  }\
        \n  .x5.y5.a5, .x5.y5.c5, .x5.y5.z5.b5 {\
        \n    x: y;\
        \n  }\
        \n}\
        \n@media order6 {\
        \n  .c6, .z6.b6, .z6.x6.y6.a6, .z6.x6.y6.c6, .z6.x6.y6.b6 {\
        \n    x: y;\
        \n  }\
        \n  .x6.y6.a6, .x6.y6.c6, .x6.y6.z6.b6 {\
        \n    x: y;\
        \n  }\
        \n  .z6.b6, .z6.x6.y6.a6, .z6.x6.y6.c6, .z6.x6.y6.b6 {\
        \n    x: y;\
        \n  }\
        \n}\
        \n"
    );
}
