//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/08_selector_combinators.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "a   +   b  >  c {\
            \n  d e {\
            \n    color: blue;\
            \n    background: white;\
            \n  }\
            \n  color: red;\
            \n  background: gray;\
            \n}"
        )
        .unwrap(),
        "a + b > c {\
        \n  color: red;\
        \n  background: gray;\
        \n}\
        \na + b > c d e {\
        \n  color: blue;\
        \n  background: white;\
        \n}\
        \n"
    );
}
