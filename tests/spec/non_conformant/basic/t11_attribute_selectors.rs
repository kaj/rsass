//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/11_attribute_selectors.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("[hey  =  \'ho\'], a > b {\
             \n  blah: blah;\
             \n  c, [hoo *=    \"ha\" ] {\
             \n    bloo: bloo;\
             \n  }\
             \n}"),
        "[hey=ho], a > b {\
         \n  blah: blah;\
         \n}\
         \n[hey=ho] c, [hey=ho] [hoo*=ha], a > b c, a > b [hoo*=ha] {\
         \n  bloo: bloo;\
         \n}\n"
    );
}
