//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/25_basic_string_interpolation.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  blah: \"hello #{2+2} world #{unit(23px)} #{\'bloo\\n\'} blah\";\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  blah: \"hello 4 world px bloon blah\";\
        \n}\
        \n"
    );
}
