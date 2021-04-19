//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/negation.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".asdf {\
            \n  $bwidth: 52px;\
            \n  left: -$bwidth/3;\
            \n  right: (1/3);\
            \n  center: (10000/3);\
            \n  blah: (20/8);\
            \n}"
        )
        .unwrap(),
        ".asdf {\
        \n  left: -17.3333333333px;\
        \n  right: 0.3333333333;\
        \n  center: 3333.3333333333;\
        \n  blah: 2.5;\
        \n}\
        \n"
    );
}
