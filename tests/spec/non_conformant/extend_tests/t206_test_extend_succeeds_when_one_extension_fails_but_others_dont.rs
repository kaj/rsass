//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/206_test_extend_succeeds_when_one_extension_fails_but_others_dont.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("a.bar {a: b}\
             \n.bar {c: d}\
             \nb.foo {@extend .bar}\n"),
        "a.bar {\
         \n  a: b;\
         \n}\
         \n.bar, b.foo {\
         \n  c: d;\
         \n}\n"
    );
}
