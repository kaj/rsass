//! Tests auto-converted from "sass-spec/spec/css/selector/slotted.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("slotted")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("::slotted(.a) {x: y}\n\
             \n::slotted(.c.d) {x: y}\
             \n.e {@extend .c}\n\
             \n::slotted(.f) {x: y}\
             \n::slotted(.g) {@extend .f}\n"),
        "::slotted(.a) {\
         \n  x: y;\
         \n}\
         \n::slotted(.c.d, .d.e) {\
         \n  x: y;\
         \n}\
         \n::slotted(.f, ::slotted(.g)) {\
         \n  x: y;\
         \n}\n"
    );
}
