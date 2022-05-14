//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/and_and.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("and_and")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".and-and {\
             \n  value: true && false;\
             \n}\n"),
        ".and-and {\
         \n  value: true .and-and .and-and false;\
         \n}\n"
    );
}
