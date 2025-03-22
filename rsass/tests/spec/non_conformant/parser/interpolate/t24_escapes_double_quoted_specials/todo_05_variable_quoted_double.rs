//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/24_escapes_double_quoted_specials/todo_05_variable_quoted_double.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("todo_05_variable_quoted_double")
        .mock_file("todo.txt", "Fails on shared build (gcc/clang)\r\nLooks like some issue with \"\\r\"")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \"\\0_\\a_\\A\";\
             \n.result {\
             \n  dquoted: \"#{#{$input}}\";\
             \n  dquoted: \"#{\"[#{$input}]\"}\";\
             \n  dquoted: \"#{\"#{$input}\"}\";\
             \n  dquoted: \"#{\'#{$input}\'}\";\
             \n  dquoted: \"#{\"[\'#{$input}\']\"}\";\
             \n  squoted: \'#{#{$input}}\';\
             \n  squoted: \'#{\"[#{$input}]\"}\';\
             \n  squoted: \'#{\"#{$input}\"}\';\
             \n  squoted: \'#{\'#{$input}\'}\';\
             \n  squoted: \'#{\"[\'#{$input}\']\"}\';\
             \n}\n"),
        "@charset \"UTF-8\";\
         \n.result {\
         \n  dquoted: \"�_\\a_\\a\";\
         \n  dquoted: \"[�_\\a_\\a]\";\
         \n  dquoted: \"�_\\a_\\a\";\
         \n  dquoted: \"�_\\a_\\a\";\
         \n  dquoted: \"[\'�_\\a_\\a\']\";\
         \n  squoted: \"�_\\a_\\a\";\
         \n  squoted: \"[�_\\a_\\a]\";\
         \n  squoted: \"�_\\a_\\a\";\
         \n  squoted: \"�_\\a_\\a\";\
         \n  squoted: \"[\'�_\\a_\\a\']\";\
         \n}\n"
    );
}
