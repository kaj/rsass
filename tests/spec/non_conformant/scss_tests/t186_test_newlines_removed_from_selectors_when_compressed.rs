//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/186_test_newlines_removed_from_selectors_when_compressed.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "a\
            \n, b {\
            \n  z & {\
            \n    display: block;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "z a,\
        \nz b {\
        \n  display: block;\
        \n}\
        \n"
    );
}
