//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2169.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function test($a, $b) {\
            \n  @return \"#{$a}\" + \"#{$b}\" + \"\" + \"\";\
            \n}\
            \n\
            \nfoo {\
            \n  content: test(\'bim\', \'baz\');\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  content: \"bimbaz\";\
        \n}\
        \n"
    );
}
