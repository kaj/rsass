//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/24_namespace_properties.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  a: {\
            \n    p1: q;\
            \n    b: {\
            \n      p2: q;\
            \n    }\
            \n    p3: q;\
            \n  }\
            \n}\
            \n\
            \nfoo {\
            \n  bar: baz {\
            \n    bip: bop;\
            \n    bing: type-of(\"hello\");\
            \n    bang: 1 + 2;\
            \n    bung: bap;\
            \n    bong: bup {\
            \n      x: x;\
            \n      y: y;\
            \n      z: z;\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  a-p1: q;\
        \n  a-b-p2: q;\
        \n  a-p3: q;\
        \n}\
        \nfoo {\
        \n  bar: baz;\
        \n  bar-bip: bop;\
        \n  bar-bing: string;\
        \n  bar-bang: 3;\
        \n  bar-bung: bap;\
        \n  bar-bong: bup;\
        \n  bar-bong-x: x;\
        \n  bar-bong-y: y;\
        \n  bar-bong-z: z;\
        \n}\
        \n"
    );
}
