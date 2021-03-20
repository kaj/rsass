//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/01_literal/05_variable_quoted_double.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: literal;\
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
        \n  dquoted: \"literal\";\
        \n  dquoted: \"[literal]\";\
        \n  dquoted: \"literal\";\
        \n  dquoted: \"literal\";\
        \n  dquoted: \"[\'literal\']\";\
        \n  squoted: \"literal\";\
        \n  squoted: \"[literal]\";\
        \n  squoted: \"literal\";\
        \n  squoted: \"literal\";\
        \n  squoted: \"[\'literal\']\";\
        \n}\
        \n"
    );
}
