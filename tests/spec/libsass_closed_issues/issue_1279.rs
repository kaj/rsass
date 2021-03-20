//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1279.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function noop($string) {\
            \n  @return $string;\
            \n}\
            \n\
            \n.foo {\
            \n  upper: to-upper-case(\'f\') + str-slice(\'foo\', 2);\
            \n  lower: to-lower-case(\'f\') + str-slice(\'foo\', 2);\
            \n  user-upper: to-upper-case(\'f\') + noop(\'oo\');\
            \n  user-lower: to-lower-case(\'f\') + noop(\'oo\');\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  upper: \"Foo\";\
        \n  lower: \"foo\";\
        \n  user-upper: \"Foo\";\
        \n  user-lower: \"foo\";\
        \n}\
        \n"
    );
}
