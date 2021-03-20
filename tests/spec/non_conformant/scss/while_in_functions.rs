//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/while_in_functions.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function test-while() {\
            \n  $x : true;\
            \n  @while $x {\
            \n    @return $x\
            \n  }\
            \n}\
            \n\
            \ndiv {\
            \n  y: test-while();\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  y: true;\
        \n}\
        \n"
    );
}
