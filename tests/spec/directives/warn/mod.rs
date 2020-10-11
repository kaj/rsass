//! Tests auto-converted from "sass-spec/spec/directives/warn"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/directives/warn/escaped.hrx"
#[test]
fn escaped() {
    assert_eq!(
        rsass(
            "@w\\61rn warning;\
            \na {b: c}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/directives/warn/functions_in_stack.hrx"
#[test]
fn functions_in_stack() {
    assert_eq!(
        rsass(
            "@function issues-warning($a) {\
            \n  @warn \"From function: #{inspect($a)}\";\
            \n  @return $a;\
            \n}\
            \n\
            \n@mixin calls-function-that-warns($a) {\
            \n  warned: issues-warning($a);\
            \n}\
            \n\
            \n.test {\
            \n  @include calls-function-that-warns(testing);\
            \n}\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  warned: testing;\
        \n}\
        \n"
    );
}
