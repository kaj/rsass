//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/07_comma_list_complex/05_variable_quoted_double.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: gamma, \"\'\"delta\"\'\";\
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
            \n}\
            \n"
        )
        .unwrap(),
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
        \n}\
        \n"
    );
}
