//! Tests auto-converted from "sass-spec/spec/directives/use/escaped.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "@u\\73 e \"other\"\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c;\
        \n}\
        \n"
    );
}
