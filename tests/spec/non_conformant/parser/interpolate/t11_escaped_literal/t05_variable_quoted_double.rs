//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/11_escaped_literal/05_variable_quoted_double.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: l\\\\ite\\ral;\
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
        \n  dquoted: \"l\\\\\\\\iteral\";\
        \n  dquoted: \"[l\\\\\\\\iteral]\";\
        \n  dquoted: \"l\\\\\\\\iteral\";\
        \n  dquoted: \"l\\\\\\\\iteral\";\
        \n  dquoted: \"[\'l\\\\\\\\iteral\']\";\
        \n  squoted: \"l\\\\\\\\iteral\";\
        \n  squoted: \"[l\\\\\\\\iteral]\";\
        \n  squoted: \"l\\\\\\\\iteral\";\
        \n  squoted: \"l\\\\\\\\iteral\";\
        \n  squoted: \"[\'l\\\\\\\\iteral\']\";\
        \n}\
        \n"
    );
}
