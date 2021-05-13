//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/07_comma_list_complex/05_variable_quoted_double.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: gamma, \"\'\"delta\"\'\";\
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
         \n  dquoted: \"gamma, \' delta \'\";\
         \n  dquoted: \"[gamma, \' delta \']\";\
         \n  dquoted: \"gamma, \' delta \'\";\
         \n  dquoted: \"gamma, \' delta \'\";\
         \n  dquoted: \"[\'gamma, \' delta \'\']\";\
         \n  squoted: \"gamma, \' delta \'\";\
         \n  squoted: \"[gamma, \' delta \']\";\
         \n  squoted: \"gamma, \' delta \'\";\
         \n  squoted: \"gamma, \' delta \'\";\
         \n  squoted: \"[\'gamma, \' delta \'\']\";\
         \n}\n"
    );
}
