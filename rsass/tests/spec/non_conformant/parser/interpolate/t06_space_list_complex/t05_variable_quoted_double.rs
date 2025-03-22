//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/06_space_list_complex/05_variable_quoted_double.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("05_variable_quoted_double")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: gamme \"\'\"delta\"\'\";\
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
         \n  dquoted: \"gamme \' delta \'\";\
         \n  dquoted: \"[gamme \' delta \']\";\
         \n  dquoted: \"gamme \' delta \'\";\
         \n  dquoted: \"gamme \' delta \'\";\
         \n  dquoted: \"[\'gamme \' delta \'\']\";\
         \n  squoted: \"gamme \' delta \'\";\
         \n  squoted: \"[gamme \' delta \']\";\
         \n  squoted: \"gamme \' delta \'\";\
         \n  squoted: \"gamme \' delta \'\";\
         \n  squoted: \"[\'gamme \' delta \'\']\";\
         \n}\n"
    );
}
