//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/02_double_quoted/06_escape_interpolation.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: \"dquoted\";\
            \n.result {\
            \n  output: \"[\\#{\"dquoted\"}]\";\
            \n  output: \"\\#{\"dquoted\"}\";\
            \n  output: \'\\#{\"dquoted\"}\';\
            \n  output: \"[\'\\#{\"dquoted\"}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \"[#{\" dquoted \"}]\";\
        \n  output: \"#{\" dquoted \"}\";\
        \n  output: \'#{\"dquoted\"}\';\
        \n  output: \"[\'#{\" dquoted \"}\']\";\
        \n}\
        \n"
    );
}
