//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1279.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1279")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:string\";\
             \n@function noop($string) {\
             \n  @return $string;\
             \n}\n\
             \n.foo {\
             \n  upper: string.to-upper-case(\'f\') + string.slice(\'foo\', 2);\
             \n  lower: string.to-lower-case(\'f\') + string.slice(\'foo\', 2);\
             \n  user-upper: string.to-upper-case(\'f\') + noop(\'oo\');\
             \n  user-lower: string.to-lower-case(\'f\') + noop(\'oo\');\
             \n}\n"
        ),
        ".foo {\
         \n  upper: \"Foo\";\
         \n  lower: \"foo\";\
         \n  user-upper: \"Foo\";\
         \n  user-lower: \"foo\";\
         \n}\n"
    );
}
