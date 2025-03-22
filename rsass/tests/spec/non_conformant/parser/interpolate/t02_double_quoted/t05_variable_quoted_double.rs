//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/02_double_quoted/05_variable_quoted_double.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("05_variable_quoted_double")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \"dquoted\";\
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
         \n  dquoted: \"dquoted\";\
         \n  dquoted: \"[dquoted]\";\
         \n  dquoted: \"dquoted\";\
         \n  dquoted: \"dquoted\";\
         \n  dquoted: \"[\'dquoted\']\";\
         \n  squoted: \"dquoted\";\
         \n  squoted: \"[dquoted]\";\
         \n  squoted: \"dquoted\";\
         \n  squoted: \"dquoted\";\
         \n  squoted: \"[\'dquoted\']\";\
         \n}\n"
    );
}
