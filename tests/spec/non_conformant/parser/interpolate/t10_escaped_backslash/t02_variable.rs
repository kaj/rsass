//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/10_escaped_backslash/02_variable.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: \\\\;\
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
        \n  output: \\\\;\
        \n  output: \\\\;\
        \n  output: \"[\\\\\\\\]\";\
        \n  output: \"\\\\\\\\\";\
        \n  output: \"\\\\\\\\\";\
        \n  output: \"[\'\\\\\\\\\']\";\
        \n}\
        \n"
    );
}
