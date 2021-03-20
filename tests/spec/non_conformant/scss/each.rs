//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/each.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "div {\
            \n  $x: 3;\
            \n  @each $x in singleton {\
            \n    color: $x;\
            \n  }\
            \n  divider: $x;\
            \n  @each $x in foo, bar, hux {\
            \n    span {\
            \n      msg: $x;\
            \n    }\
            \n  }\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  color: singleton;\
        \n  divider: 3;\
        \n}\
        \ndiv span {\
        \n  msg: foo;\
        \n}\
        \ndiv span {\
        \n  msg: bar;\
        \n}\
        \ndiv span {\
        \n  msg: hux;\
        \n}\
        \n"
    );
}
