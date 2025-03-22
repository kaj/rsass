//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/186_test_control_flow_while.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("186_test_control_flow_while")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".base-0  { color: green; }\
             \n.base-1  { display: block; }\
             \n.base-2  { border: 1px solid blue; }\
             \n.added {\
             \n$i : 0;\
             \n@while $i < 3 {\
             \n  @extend .base-#{$i};\
             \n  $i : $i + 1;\
             \n}\
             \n}\n"),
        ".base-0, .added {\
         \n  color: green;\
         \n}\
         \n.base-1, .added {\
         \n  display: block;\
         \n}\
         \n.base-2, .added {\
         \n  border: 1px solid blue;\
         \n}\n"
    );
}
