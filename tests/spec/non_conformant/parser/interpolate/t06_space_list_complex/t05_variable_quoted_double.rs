//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/06_space_list_complex/05_variable_quoted_double.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: gamme \"\'\"delta\"\'\";\
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
        \n}\
        \n"
    );
}
