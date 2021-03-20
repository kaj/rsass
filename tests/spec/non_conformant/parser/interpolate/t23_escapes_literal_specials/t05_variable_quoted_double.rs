//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/23_escapes_literal_specials/05_variable_quoted_double.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: \\0_\\a_\\A;\
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
        \n  dquoted: \"\\\\0 _\\\\a _\\\\a \";\
        \n  dquoted: \"[\\\\0 _\\\\a _\\\\a ]\";\
        \n  dquoted: \"\\\\0 _\\\\a _\\\\a \";\
        \n  dquoted: \"\\\\0 _\\\\a _\\\\a \";\
        \n  dquoted: \"[\'\\\\0 _\\\\a _\\\\a \']\";\
        \n  squoted: \"\\\\0 _\\\\a _\\\\a \";\
        \n  squoted: \"[\\\\0 _\\\\a _\\\\a ]\";\
        \n  squoted: \"\\\\0 _\\\\a _\\\\a \";\
        \n  squoted: \"\\\\0 _\\\\a _\\\\a \";\
        \n  squoted: \"[\'\\\\0 _\\\\a _\\\\a \']\";\
        \n}\
        \n"
    );
}
