//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/01_literal/02_variable.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: literal;\
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
        \n  output: literal;\
        \n  output: literal;\
        \n  output: \"[literal]\";\
        \n  output: \"literal\";\
        \n  output: \"literal\";\
        \n  output: \"[\'literal\']\";\
        \n}\
        \n"
    );
}
