//! Tests auto-converted from "sass-spec/spec/directives/warn/functions_in_stack.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
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
