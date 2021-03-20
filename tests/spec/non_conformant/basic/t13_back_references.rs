//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/13_back_references.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "hey, ho {\
            \n  & > boo, foo &.goo {\
            \n    bloo: bloo;\
            \n  }\
            \n  blah: blah;\
            \n}"
        )
        .unwrap(),
        "hey, ho {\
        \n  blah: blah;\
        \n}\
        \nhey > boo, foo hey.goo, ho > boo, foo ho.goo {\
        \n  bloo: bloo;\
        \n}\
        \n"
    );
}
