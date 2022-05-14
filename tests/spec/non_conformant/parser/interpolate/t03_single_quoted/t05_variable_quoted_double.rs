//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/03_single_quoted/05_variable_quoted_double.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("05_variable_quoted_double")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \'squoted\';\
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
         \n  dquoted: \"squoted\";\
         \n  dquoted: \"[squoted]\";\
         \n  dquoted: \"squoted\";\
         \n  dquoted: \"squoted\";\
         \n  dquoted: \"[\'squoted\']\";\
         \n  squoted: \"squoted\";\
         \n  squoted: \"[squoted]\";\
         \n  squoted: \"squoted\";\
         \n  squoted: \"squoted\";\
         \n  squoted: \"[\'squoted\']\";\
         \n}\n"
    );
}
