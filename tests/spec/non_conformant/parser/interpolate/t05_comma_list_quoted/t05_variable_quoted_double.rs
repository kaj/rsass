//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/05_comma_list_quoted/05_variable_quoted_double.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: \"alpha\", \'beta\';\
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
        \n  dquoted: \"alpha, beta\";\
        \n  dquoted: \"[alpha, beta]\";\
        \n  dquoted: \"alpha, beta\";\
        \n  dquoted: \"alpha, beta\";\
        \n  dquoted: \"[\'alpha, beta\']\";\
        \n  squoted: \"alpha, beta\";\
        \n  squoted: \"[alpha, beta]\";\
        \n  squoted: \"alpha, beta\";\
        \n  squoted: \"alpha, beta\";\
        \n  squoted: \"[\'alpha, beta\']\";\
        \n}\
        \n"
    );
}
