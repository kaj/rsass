//! Tests auto-converted from "sass-spec/spec/libsass/conversions.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  width: 3cm * 2in * 2in / 1cm / 1cm;\
            \n  width: 3cm * 2in / 1cm;\
            \n  width: 4cm * (12in / 3in);\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  width: 30.48in;\
        \n  width: 6in;\
        \n  width: 16cm;\
        \n}\
        \n"
    );
}
