//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/44_bem_selectors.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("44_bem_selectors")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("div {\n\
             \n  &_foo {\
             \n    blah: blah;\
             \n  }\
             \n  &--modifier {\
             \n    blach: blah;\
             \n  }\
             \n  &hux {\
             \n    blah: blah;\
             \n  }\
             \n  &div.foo#bar[hux] {\
             \n    blah: blah;\
             \n  }\n\
             \n}"),
        "div_foo {\
         \n  blah: blah;\
         \n}\
         \ndiv--modifier {\
         \n  blach: blah;\
         \n}\
         \ndivhux {\
         \n  blah: blah;\
         \n}\
         \ndivdiv.foo#bar[hux] {\
         \n  blah: blah;\
         \n}\n"
    );
}
