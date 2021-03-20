//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/041_test_newlines_in_selectors.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "foo,\
            \nbar {\
            \n  baz,\
            \n  bang {a: b}}\
            \n"
        )
        .unwrap(),
        "foo baz,\
        \nfoo bang,\
        \nbar baz,\
        \nbar bang {\
        \n  a: b;\
        \n}\
        \n"
    );
}
