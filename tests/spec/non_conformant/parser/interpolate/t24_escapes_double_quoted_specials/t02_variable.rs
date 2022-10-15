//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/24_escapes_double_quoted_specials/02_variable.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("02_variable")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \"\\0_\\a_\\A\";\
             \n.result {\
             \n  output: $input;\
             \n  output: #{$input};\
             \n  output: \"[#{$input}]\";\
             \n  output: \"#{$input}\";\
             \n  output: \'#{$input}\';\
             \n  output: \"[\'#{$input}\']\";\
             \n}\n"),
        "@charset \"UTF-8\";\
         \n.result {\
         \n  output: \"�_\\a_\\a\";\
         \n  output: �_ _ ;\
         \n  output: \"[�_\\a_\\a]\";\
         \n  output: \"�_\\a_\\a\";\
         \n  output: \"�_\\a_\\a\";\
         \n  output: \"[\'�_\\a_\\a\']\";\
         \n}\n"
    );
}
