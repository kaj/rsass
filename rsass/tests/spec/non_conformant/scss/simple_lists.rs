//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/simple-lists.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("simple-lists")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:string\";\
             \ndiv {\
             \n  hey: a, b, c, d;\
             \n  ho: a b c d;\
             \n  ha: string.unquote(\"a, b, c, d\");\
             \n}"),
        "div {\
         \n  hey: a, b, c, d;\
         \n  ho: a b c d;\
         \n  ha: a, b, c, d;\
         \n}\n"
    );
}
