//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1121.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$foo: \"foo\";\
            \n$bar: \"bar\";\
            \n$baz: \"baz\";\
            \n/*\
            \n * <div class=\"foo #{$foo}\" bar=\"#{$bar}\" baz=\"#{$baz}\">\
            \n */\
            \n"
        )
        .unwrap(),
        "/*\
        \n * <div class=\"foo foo\" bar=\"bar\" baz=\"baz\">\
        \n */\
        \n"
    );
}
