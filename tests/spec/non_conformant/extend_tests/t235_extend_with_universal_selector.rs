//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/235_extend_with_universal_selector.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "%-a *.foo1 {a: b}\
            \na {@extend .foo1}\
            \n-a {@extend %-a}\
            \n\
            \n%-b *|*.foo2 {b: b}\
            \nb {@extend .foo2}\
            \n-b {@extend %-b}\
            \n"
        )
        .unwrap(),
        "-a *.foo1, -a a {\
        \n  a: b;\
        \n}\
        \n-b *|*.foo2, -b b {\
        \n  b: b;\
        \n}\
        \n"
    );
}
