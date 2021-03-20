//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/02_double_quoted/03_inline_double.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".result {\
            \n  output: #{#{\"dquoted\"}};\
            \n  output: #{\"[#{\"dquoted\"}]\"};\
            \n  output: #{\"#{\"dquoted\"}\"};\
            \n  output: #{\'#{\"dquoted\"}\'};\
            \n  output: #{\"[\'#{\"dquoted\"}\']\"};\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: dquoted;\
        \n  output: [dquoted];\
        \n  output: dquoted;\
        \n  output: dquoted;\
        \n  output: [\'dquoted\'];\
        \n}\
        \n"
    );
}
