//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/31_schema_simple/05_variable_quoted_double.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: \"[\"\'foo\'\"]\";\
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
        \n  dquoted: \"[ foo ]\";\
        \n  dquoted: \"[[ foo ]]\";\
        \n  dquoted: \"[ foo ]\";\
        \n  dquoted: \"[ foo ]\";\
        \n  dquoted: \"[\'[ foo ]\']\";\
        \n  squoted: \"[ foo ]\";\
        \n  squoted: \"[[ foo ]]\";\
        \n  squoted: \"[ foo ]\";\
        \n  squoted: \"[ foo ]\";\
        \n  squoted: \"[\'[ foo ]\']\";\
        \n}\
        \n"
    );
}
