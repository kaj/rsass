//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/fake-pseudo-element-order/first-line.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("first-line")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%a:first-line {x: y}\
             \nb:c {@extend %a}\n"),
        "b:c:first-line {\
         \n  x: y;\
         \n}\n"
    );
}
