//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/fake-pseudo-element-order/first-letter.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("first-letter")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok("%a:first-letter {x: y}\
             \nb:c {@extend %a}\n"),
        "b:c:first-letter {\
         \n  x: y;\
         \n}\n"
    );
}
