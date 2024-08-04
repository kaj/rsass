//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/13_back_references.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("13_back_references")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("hey, ho {\
             \n  & > boo, foo &.goo {\
             \n    bloo: bloo;\
             \n  }\
             \n  blah: blah;\
             \n}"),
        "hey, ho {\
         \n  blah: blah;\
         \n}\
         \nhey > boo, foo hey.goo, ho > boo, foo ho.goo {\
         \n  bloo: bloo;\
         \n}\n"
    );
}
