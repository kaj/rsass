//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/02_double_quoted/05_variable_quoted_double.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: \"dquoted\";\
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
        \n  dquoted: \"dquoted\";\
        \n  dquoted: \"[dquoted]\";\
        \n  dquoted: \"dquoted\";\
        \n  dquoted: \"dquoted\";\
        \n  dquoted: \"[\'dquoted\']\";\
        \n  squoted: \"dquoted\";\
        \n  squoted: \"[dquoted]\";\
        \n  squoted: \"dquoted\";\
        \n  squoted: \"dquoted\";\
        \n  squoted: \"[\'dquoted\']\";\
        \n}\
        \n"
    );
}
