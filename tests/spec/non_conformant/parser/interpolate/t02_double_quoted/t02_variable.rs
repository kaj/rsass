//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/02_double_quoted/02_variable.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: \"dquoted\";\
            \n.result {\
            \n  output: $input;\
            \n  output: #{$input};\
            \n  output: \"[#{$input}]\";\
            \n  output: \"#{$input}\";\
            \n  output: \'#{$input}\';\
            \n  output: \"[\'#{$input}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \"dquoted\";\
        \n  output: dquoted;\
        \n  output: \"[dquoted]\";\
        \n  output: \"dquoted\";\
        \n  output: \"dquoted\";\
        \n  output: \"[\'dquoted\']\";\
        \n}\
        \n"
    );
}
