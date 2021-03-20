//! Tests auto-converted from "sass-spec/spec/non_conformant/misc/selector_interpolation_before_element_name.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "#{\"foo\" + \" bar\"}baz {a: b}\
            \n"
        )
        .unwrap(),
        "foo barbaz {\
        \n  a: b;\
        \n}\
        \n"
    );
}
