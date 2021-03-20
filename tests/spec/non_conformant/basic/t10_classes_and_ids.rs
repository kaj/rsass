//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/10_classes_and_ids.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "a + b, .class {\
            \n  blah: blah;\
            \n  bleh: bleh;\
            \n  d #id, f ~ g.other + h, > i#grar {\
            \n    bloo: bloo;\
            \n    blee: blee;\
            \n  }\
            \n}"
        )
        .unwrap(),
        "a + b, .class {\
        \n  blah: blah;\
        \n  bleh: bleh;\
        \n}\
        \na + b d #id, a + b f ~ g.other + h, a + b > i#grar, .class d #id, .class f ~ g.other + h, .class > i#grar {\
        \n  bloo: bloo;\
        \n  blee: blee;\
        \n}\
        \n"
    );
}
