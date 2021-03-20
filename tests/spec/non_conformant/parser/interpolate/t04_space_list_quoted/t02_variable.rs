//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/04_space_list_quoted/02_variable.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: \"alpha\" \'beta\';\
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
        \n  output: \"alpha\" \"beta\";\
        \n  output: alpha beta;\
        \n  output: \"[alpha beta]\";\
        \n  output: \"alpha beta\";\
        \n  output: \"alpha beta\";\
        \n  output: \"[\'alpha beta\']\";\
        \n}\
        \n"
    );
}
