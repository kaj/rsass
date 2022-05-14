//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/30_base_test/05_variable_quoted_double.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("05_variable_quoted_double")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \"foo#{\'ba\' + \'r\'}baz\";\
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
         \n  dquoted: \"foobarbaz\";\
         \n  dquoted: \"[foobarbaz]\";\
         \n  dquoted: \"foobarbaz\";\
         \n  dquoted: \"foobarbaz\";\
         \n  dquoted: \"[\'foobarbaz\']\";\
         \n  squoted: \"foobarbaz\";\
         \n  squoted: \"[foobarbaz]\";\
         \n  squoted: \"foobarbaz\";\
         \n  squoted: \"foobarbaz\";\
         \n  squoted: \"[\'foobarbaz\']\";\
         \n}\n"
    );
}
