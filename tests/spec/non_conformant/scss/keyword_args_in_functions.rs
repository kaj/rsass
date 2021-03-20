//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/keyword_args_in_functions.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".keyed { color: rgba($color: #a7c, $alpha: 0.4) }\
            \n"
        )
        .unwrap(),
        ".keyed {\
        \n  color: rgba(170, 119, 204, 0.4);\
        \n}\
        \n"
    );
}
