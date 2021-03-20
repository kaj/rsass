//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/important.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  color: red ! important;\
            \n  width: 5px ! important;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  color: red !important;\
        \n  width: 5px !important;\
        \n}\
        \n"
    );
}
