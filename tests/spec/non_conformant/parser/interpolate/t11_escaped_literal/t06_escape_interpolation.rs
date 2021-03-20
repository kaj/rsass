//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/11_escaped_literal/06_escape_interpolation.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: l\\\\ite\\ral;\
            \n.result {\
            \n  output: \"[\\#{l\\\\ite\\ral}]\";\
            \n  output: \"\\#{l\\\\ite\\ral}\";\
            \n  output: \'\\#{l\\\\ite\\ral}\';\
            \n  output: \"[\'\\#{l\\\\ite\\ral}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \"[#{l\\\\iteral}]\";\
        \n  output: \"#{l\\\\iteral}\";\
        \n  output: \"#{l\\\\iteral}\";\
        \n  output: \"[\'#{l\\\\iteral}\']\";\
        \n}\
        \n"
    );
}
