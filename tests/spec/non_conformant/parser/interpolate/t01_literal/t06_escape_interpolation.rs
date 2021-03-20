//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/01_literal/06_escape_interpolation.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: literal;\
            \n.result {\
            \n  output: \"[\\#{literal}]\";\
            \n  output: \"\\#{literal}\";\
            \n  output: \'\\#{literal}\';\
            \n  output: \"[\'\\#{literal}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \"[#{literal}]\";\
        \n  output: \"#{literal}\";\
        \n  output: \"#{literal}\";\
        \n  output: \"[\'#{literal}\']\";\
        \n}\
        \n"
    );
}
