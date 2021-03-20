//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/numbers.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  width: 10px;\
            \n  height: 20%;\
            \n  blah: 12;\
            \n  color: #abc;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  width: 10px;\
        \n  height: 20%;\
        \n  blah: 12;\
        \n  color: #abc;\
        \n}\
        \n"
    );
}
