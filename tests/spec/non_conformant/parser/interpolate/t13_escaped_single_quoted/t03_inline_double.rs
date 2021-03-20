//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/13_escaped_single_quoted/03_inline_double.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".result {\
            \n  output: #{#{\'l\\\\ite\\ral\'}};\
            \n  output: #{\"[#{\'l\\\\ite\\ral\'}]\"};\
            \n  output: #{\"#{\'l\\\\ite\\ral\'}\"};\
            \n  output: #{\'#{\'l\\\\ite\\ral\'}\'};\
            \n  output: #{\"[\'#{\'l\\\\ite\\ral\'}\']\"};\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: l\\iteral;\
        \n  output: [l\\iteral];\
        \n  output: l\\iteral;\
        \n  output: l\\iteral;\
        \n  output: [\'l\\iteral\'];\
        \n}\
        \n"
    );
}
