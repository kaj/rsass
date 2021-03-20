//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/each_directive.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  @each $number in 1px 2px 3px 4px {\
            \n    b: $number;\
            \n  }\
            \n}\
            \nc {\
            \n  @each $str in foo, bar, baz, bang {\
            \n    d: $str;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1px;\
        \n  b: 2px;\
        \n  b: 3px;\
        \n  b: 4px;\
        \n}\
        \nc {\
        \n  d: foo;\
        \n  d: bar;\
        \n  d: baz;\
        \n  d: bang;\
        \n}\
        \n"
    );
}
