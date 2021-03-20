//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/extend-result-of-extend.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "// The result of :not(.c) being extended should itself be extendable.\
            \n.a {@extend :not(.b)}\
            \n.b {@extend .c}\
            \n:not(.c) {x: y}\
            \n"
        )
        .unwrap(),
        ":not(.c):not(.b), .a:not(.c) {\
        \n  x: y;\
        \n}\
        \n"
    );
}
