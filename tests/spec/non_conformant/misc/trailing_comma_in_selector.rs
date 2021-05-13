//! Tests auto-converted from "sass-spec/spec/non_conformant/misc/trailing_comma_in_selector.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("#foo #bar,,\
             \n,#baz #boom, {a: b}\n\
             \n#bip #bop, ,, {c: d}\n"),
        "#foo #bar,\
         \n#baz #boom {\
         \n  a: b;\
         \n}\
         \n#bip #bop {\
         \n  c: d;\
         \n}\n"
    );
}
