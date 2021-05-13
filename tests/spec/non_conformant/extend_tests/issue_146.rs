//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/issue_146.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%btn-style-default {\
             \n  background: green;\
             \n  &:hover{\
             \n    background: black;\
             \n  }\
             \n}\n\
             \nbutton {\
             \n  @extend %btn-style-default;\
             \n}"),
        "button {\
         \n  background: green;\
         \n}\
         \nbutton:hover {\
         \n  background: black;\
         \n}\n"
    );
}
