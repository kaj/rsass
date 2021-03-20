//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/interpolated-strings.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$x: ecks;\
            \n$y: why;\
            \n\
            \ndiv {\
            \n  blah: \"hey #{$x} ho\";\
            \n  blee: hey#{$y}ho;\
            \n  bluh: \"foo #{$x}\";\
            \n  bleg: foo#{\"hey\"}bar;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  blah: \"hey ecks ho\";\
        \n  blee: heywhyho;\
        \n  bluh: \"foo ecks\";\
        \n  bleg: fooheybar;\
        \n}\
        \n"
    );
}
