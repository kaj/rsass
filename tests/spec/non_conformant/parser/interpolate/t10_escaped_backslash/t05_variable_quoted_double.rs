//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/10_escaped_backslash/05_variable_quoted_double.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: \\\\;\
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
        \n  dquoted: \"\\\\\\\\\";\
        \n  dquoted: \"[\\\\\\\\]\";\
        \n  dquoted: \"\\\\\\\\\";\
        \n  dquoted: \"\\\\\\\\\";\
        \n  dquoted: \"[\'\\\\\\\\\']\";\
        \n  squoted: \"\\\\\\\\\";\
        \n  squoted: \"[\\\\\\\\]\";\
        \n  squoted: \"\\\\\\\\\";\
        \n  squoted: \"\\\\\\\\\";\
        \n  squoted: \"[\'\\\\\\\\\']\";\
        \n}\
        \n"
    );
}
