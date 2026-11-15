//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/23_escapes_literal_specials/05_variable_quoted_double.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("05_variable_quoted_double")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("$input: \\0_\\a_\\A;\
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
         \n  dquoted: \"�_\\\\a _\\\\a \";\
         \n  dquoted: \"[�_\\\\a _\\\\a ]\";\
         \n  dquoted: \"�_\\\\a _\\\\a \";\
         \n  dquoted: \"�_\\\\a _\\\\a \";\
         \n  dquoted: \"[\'�_\\\\a _\\\\a \']\";\
         \n  squoted: \"�_\\\\a _\\\\a \";\
         \n  squoted: \"[�_\\\\a _\\\\a ]\";\
         \n  squoted: \"�_\\\\a _\\\\a \";\
         \n  squoted: \"�_\\\\a _\\\\a \";\
         \n  squoted: \"[\'�_\\\\a _\\\\a \']\";\
         \n}\n"
    );
}
