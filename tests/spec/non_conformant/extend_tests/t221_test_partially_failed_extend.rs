//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/221_test_partially_failed_extend.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "test { @extend .rc; }\
            \n.rc {color: white;}\
            \n.prices span.pill span.rc {color: red;}\
            \n"
        )
        .unwrap(),
        ".rc, test {\
        \n  color: white;\
        \n}\
        \n.prices span.pill span.rc {\
        \n  color: red;\
        \n}\
        \n"
    );
}
