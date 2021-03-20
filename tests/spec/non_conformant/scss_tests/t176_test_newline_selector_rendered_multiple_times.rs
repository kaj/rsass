//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests/176_test_newline_selector_rendered_multiple_times.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@for $i from 1 through 2 {\
            \n  form {\
            \n    input,\
            \n    select {\
            \n      color: white;\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "form input,\
        \nform select {\
        \n  color: white;\
        \n}\
        \nform input,\
        \nform select {\
        \n  color: white;\
        \n}\
        \n"
    );
}
