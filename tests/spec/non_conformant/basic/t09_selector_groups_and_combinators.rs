//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/09_selector_groups_and_combinators.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("a + b, c {\
             \n  blah: blah;\
             \n  bleh: bleh;\
             \n  d e, f ~ g + h, > i {\
             \n    bloo: bloo;\
             \n    blee: blee;\
             \n  }\
             \n}"),
        "a + b, c {\
         \n  blah: blah;\
         \n  bleh: bleh;\
         \n}\
         \na + b d e, a + b f ~ g + h, a + b > i, c d e, c f ~ g + h, c > i {\
         \n  bloo: bloo;\
         \n  blee: blee;\
         \n}\n"
    );
}
