//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/classes-and-ids.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div.foo {\
            \n  color: red;\
            \n  #hux buz {\
            \n    width: auto;\
            \n  }\
            \n  > .mux {\
            \n    text-align: center;\
            \n  }\
            \n}"
        )
        .unwrap(),
        "div.foo {\
        \n  color: red;\
        \n}\
        \ndiv.foo #hux buz {\
        \n  width: auto;\
        \n}\
        \ndiv.foo > .mux {\
        \n  text-align: center;\
        \n}\
        \n"
    );
}
