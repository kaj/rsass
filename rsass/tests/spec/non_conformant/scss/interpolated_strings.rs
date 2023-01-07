//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/interpolated-strings.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("interpolated-strings")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$x: ecks;\
             \n$y: why;\n\
             \ndiv {\
             \n  blah: \"hey #{$x} ho\";\
             \n  blee: hey#{$y}ho;\
             \n  bluh: \"foo #{$x}\";\
             \n  bleg: foo#{\"hey\"}bar;\
             \n}"),
        "div {\
         \n  blah: \"hey ecks ho\";\
         \n  blee: heywhyho;\
         \n  bluh: \"foo ecks\";\
         \n  bleg: fooheybar;\
         \n}\n"
    );
}
