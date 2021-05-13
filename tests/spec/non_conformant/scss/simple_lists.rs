//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/simple-lists.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div {\
             \n  hey: a, b, c, d;\
             \n  ho: a b c d;\
             \n  ha: unquote(\"a, b, c, d\");\
             \n}"),
        "div {\
         \n  hey: a, b, c, d;\
         \n  ho: a b c d;\
         \n  ha: a, b, c, d;\
         \n}\n"
    );
}
