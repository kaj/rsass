//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1207.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function test($pos) {\
            \n  @return test-#{$pos};\
            \n}\
            \n\
            \n.foo {\
            \n  content: test(str-slice(\'scale-0\', 7));   // Nope\
            \n  content: test-#{str-slice(\'scale-0\', 7)}; // Yep\
            \n}"
        )
        .unwrap(),
        ".foo {\
        \n  content: test-0;\
        \n  content: test-0;\
        \n}\
        \n"
    );
}
