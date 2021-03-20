//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/193_test_media_in_placeholder_selector.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "%foo {bar {@media screen {a {b: c}}}}\
            \n.baz {c: d}\
            \n"
        )
        .unwrap(),
        ".baz {\
        \n  c: d;\
        \n}\
        \n"
    );
}
