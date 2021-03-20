//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/01_literal/01_inline.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".result {\
            \n  output: literal;\
            \n  output: #{literal};\
            \n  output: \"[#{literal}]\";\
            \n  output: \"#{literal}\";\
            \n  output: \'#{literal}\';\
            \n  output: \"[\'#{literal}\']\";\
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
