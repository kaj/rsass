//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/10_classes_and_ids.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("10_classes_and_ids")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "a + b, .class {\
             \n  blah: blah;\
             \n  bleh: bleh;\
             \n  d #id, f ~ g.other + h, > i#grar {\
             \n    bloo: bloo;\
             \n    blee: blee;\
             \n  }\
             \n}"
        ),
        "a + b, .class {\
         \n  blah: blah;\
         \n  bleh: bleh;\
         \n}\
         \na + b d #id, a + b f ~ g.other + h, a + b > i#grar, .class d #id, .class f ~ g.other + h, .class > i#grar {\
         \n  bloo: bloo;\
         \n  blee: blee;\
         \n}\n"
    );
}
