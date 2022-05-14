//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/40_pseudo_class_identifier_starting_with_n.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("40_pseudo_class_identifier_starting_with_n")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("div:lang(nb) {\
             \n  color: blue;\
             \n}"),
        "div:lang(nb) {\
         \n  color: blue;\
         \n}\n"
    );
}
