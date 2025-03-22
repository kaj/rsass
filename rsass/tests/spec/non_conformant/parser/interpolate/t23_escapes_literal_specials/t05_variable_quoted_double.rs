//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/23_escapes_literal_specials/05_variable_quoted_double.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("05_variable_quoted_double")
}

#[test]
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
        ".result {\
         \n  dquoted: \"\\\\0 _\\\\a _\\\\a \";\
         \n  dquoted: \"[\\\\0 _\\\\a _\\\\a ]\";\
         \n  dquoted: \"\\\\0 _\\\\a _\\\\a \";\
         \n  dquoted: \"\\\\0 _\\\\a _\\\\a \";\
         \n  dquoted: \"[\'\\\\0 _\\\\a _\\\\a \']\";\
         \n  squoted: \"\\\\0 _\\\\a _\\\\a \";\
         \n  squoted: \"[\\\\0 _\\\\a _\\\\a ]\";\
         \n  squoted: \"\\\\0 _\\\\a _\\\\a \";\
         \n  squoted: \"\\\\0 _\\\\a _\\\\a \";\
         \n  squoted: \"[\'\\\\0 _\\\\a _\\\\a \']\";\
         \n}\n"
    );
}
