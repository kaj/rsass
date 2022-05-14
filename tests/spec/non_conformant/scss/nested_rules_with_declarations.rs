//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/nested_rules_with_declarations.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("nested_rules_with_declarations")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
             \n  ump: nump;\
             \n  grump: clump;\
             \n  bar {\
             \n    blat: bang;\
             \n    habit: rabbit;\
             \n    baz {a: b}\
             \n    bip {c: d}}\
             \n  bibble {\
             \n    bap {e: f}}}\n"),
        "foo {\
         \n  ump: nump;\
         \n  grump: clump;\
         \n}\
         \nfoo bar {\
         \n  blat: bang;\
         \n  habit: rabbit;\
         \n}\
         \nfoo bar baz {\
         \n  a: b;\
         \n}\
         \nfoo bar bip {\
         \n  c: d;\
         \n}\
         \nfoo bibble bap {\
         \n  e: f;\
         \n}\n"
    );
}
