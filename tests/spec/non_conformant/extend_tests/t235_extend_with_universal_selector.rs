//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/235_extend_with_universal_selector.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%-a *.foo1 {a: b}\
             \na {@extend .foo1}\
             \n-a {@extend %-a}\n\
             \n%-b *|*.foo2 {b: b}\
             \nb {@extend .foo2}\
             \n-b {@extend %-b}\n"),
        "-a *.foo1, -a a {\
         \n  a: b;\
         \n}\
         \n-b *|*.foo2, -b b {\
         \n  b: b;\
         \n}\n"
    );
}
