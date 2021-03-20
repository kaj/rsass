//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/27_media_queries.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "a b c {\
            \n  blee: blee;\
            \n  d e f {\
            \n    blah: blah;\
            \n    bloo: bloo;\
            \n  }\
            \n  g h, i j {\
            \n    @media print and (foo: 1 2 3), (bar: 3px hux(muz)), not screen {\
            \n      hey: ho;\
            \n      k l m {\
            \n        hee: fee;\
            \n      }\
            \n    }\
            \n  }\
            \n  blah: blah;\
            \n}\
            \n\
            \n\
            \n"
        )
        .unwrap(),
        "a b c {\
        \n  blee: blee;\
        \n  blah: blah;\
        \n}\
        \na b c d e f {\
        \n  blah: blah;\
        \n  bloo: bloo;\
        \n}\
        \n@media print and (foo: 1 2 3), (bar: 3px hux(muz)), not screen {\
        \n  a b c g h, a b c i j {\
        \n    hey: ho;\
        \n  }\
        \n  a b c g h k l m, a b c i j k l m {\
        \n    hee: fee;\
        \n  }\
        \n}\
        \n"
    );
}
