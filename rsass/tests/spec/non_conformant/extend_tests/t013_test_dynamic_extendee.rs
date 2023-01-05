//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/013_test_dynamic_extendee.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("013_test_dynamic_extendee")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("[baz^=\"blip12px\"] {a: b}\
             \n.bar {@extend [baz^=\"blip#{12px}\"]}\n"),
        "[baz^=blip12px], .bar {\
         \n  a: b;\
         \n}\n"
    );
}
