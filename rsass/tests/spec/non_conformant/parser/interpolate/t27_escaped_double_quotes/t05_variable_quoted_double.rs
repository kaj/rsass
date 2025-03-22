//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/27_escaped_double_quotes/05_variable_quoted_double.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("05_variable_quoted_double")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \"\\\"\";\
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
         \n  dquoted: \'\"\';\
         \n  dquoted: \'[\"]\';\
         \n  dquoted: \'\"\';\
         \n  dquoted: \'\"\';\
         \n  dquoted: \"[\'\\\"\']\";\
         \n  squoted: \'\"\';\
         \n  squoted: \'[\"]\';\
         \n  squoted: \'\"\';\
         \n  squoted: \'\"\';\
         \n  squoted: \"[\'\\\"\']\";\
         \n}\n"
    );
}
