//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/193_test_media_in_placeholder_selector.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("193_test_media_in_placeholder_selector")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%foo {bar {@media screen {a {b: c}}}}\
             \n.baz {c: d}\n"),
        ".baz {\
         \n  c: d;\
         \n}\n"
    );
}
