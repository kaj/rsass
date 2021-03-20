//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/12_escaped_double_quoted/02_variable.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: \"l\\\\ite\\ral\";\
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
        \n  output: \"l\\\\iteral\";\
        \n  output: l\\iteral;\
        \n  output: \"[l\\\\iteral]\";\
        \n  output: \"l\\\\iteral\";\
        \n  output: \"l\\\\iteral\";\
        \n  output: \"[\'l\\\\iteral\']\";\
        \n}\
        \n"
    );
}
