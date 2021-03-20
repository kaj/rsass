//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/blimp.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "blimp { color: green }\
            \n"
        )
        .unwrap(),
        "blimp {\
        \n  color: green;\
        \n}\
        \n"
    );
}
