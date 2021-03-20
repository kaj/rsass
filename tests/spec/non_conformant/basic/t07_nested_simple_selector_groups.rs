//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/07_nested_simple_selector_groups.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "a, b {\
            \n  color: red;\
            \n  background: blue;\
            \n}\
            \n\
            \nc, d {\
            \n  color: gray;\
            \n  e, f {\
            \n    background: blue;\
            \n    padding: 10px 5px;\
            \n  }\
            \n  g, h {\
            \n    blah: blah;\
            \n    bloo: bloo;\
            \n  }\
            \n  i, j {\
            \n    foo: goo;\
            \n    k, l {\
            \n      m, n, o {\
            \n        wow: we are far inside;\
            \n        but: it still works;\
            \n      }\
            \n      hoo: boo;\
            \n    }\
            \n  }\
            \n}"
        )
        .unwrap(),
        "a, b {\
        \n  color: red;\
        \n  background: blue;\
        \n}\
        \nc, d {\
        \n  color: gray;\
        \n}\
        \nc e, c f, d e, d f {\
        \n  background: blue;\
        \n  padding: 10px 5px;\
        \n}\
        \nc g, c h, d g, d h {\
        \n  blah: blah;\
        \n  bloo: bloo;\
        \n}\
        \nc i, c j, d i, d j {\
        \n  foo: goo;\
        \n}\
        \nc i k, c i l, c j k, c j l, d i k, d i l, d j k, d j l {\
        \n  hoo: boo;\
        \n}\
        \nc i k m, c i k n, c i k o, c i l m, c i l n, c i l o, c j k m, c j k n, c j k o, c j l m, c j l n, c j l o, d i k m, d i k n, d i k o, d i l m, d i l n, d i l o, d j k m, d j k n, d j k o, d j l m, d j l n, d j l o {\
        \n  wow: we are far inside;\
        \n  but: it still works;\
        \n}\
        \n"
    );
}
