//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/176_test_newline_selector_rendered_multiple_times.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@for $i from 1 through 2 {\
             \n  form {\
             \n    input,\
             \n    select {\
             \n      color: white;\
             \n    }\
             \n  }\
             \n}\n"),
        "form input,\
         \nform select {\
         \n  color: white;\
         \n}\
         \nform input,\
         \nform select {\
         \n  color: white;\
         \n}\n"
    );
}
