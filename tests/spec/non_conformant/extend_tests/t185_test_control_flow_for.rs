//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/185_test_control_flow_for.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".base-0  { color: green; }\
            \n.base-1  { display: block; }\
            \n.base-2  { border: 1px solid blue; }\
            \n.added {\
            \n@for $i from 0 to 3 {\
            \n  @extend .base-#{$i};\
            \n}\
            \n}\
            \n"
        )
        .unwrap(),
        ".base-0, .added {\
        \n  color: green;\
        \n}\
        \n.base-1, .added {\
        \n  display: block;\
        \n}\
        \n.base-2, .added {\
        \n  border: 1px solid blue;\
        \n}\
        \n"
    );
}
