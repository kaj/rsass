//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/29_binary_operation/05_variable_quoted_double.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: \"foo#{\'ba\' + \'r\'}baz\";\
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
        \n  dquoted: \"foobarbaz\";\
        \n  dquoted: \"[foobarbaz]\";\
        \n  dquoted: \"foobarbaz\";\
        \n  dquoted: \"foobarbaz\";\
        \n  dquoted: \"[\'foobarbaz\']\";\
        \n  squoted: \"foobarbaz\";\
        \n  squoted: \"[foobarbaz]\";\
        \n  squoted: \"foobarbaz\";\
        \n  squoted: \"foobarbaz\";\
        \n  squoted: \"[\'foobarbaz\']\";\
        \n}\
        \n"
    );
}
