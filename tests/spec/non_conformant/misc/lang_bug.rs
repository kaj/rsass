//! Tests auto-converted from "sass-spec/spec/non_conformant/misc/lang-bug.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div:lang(nb) {\
            \n  color: red;\
            \n}"
        )
        .unwrap(),
        "div:lang(nb) {\
        \n  color: red;\
        \n}\
        \n"
    );
}
