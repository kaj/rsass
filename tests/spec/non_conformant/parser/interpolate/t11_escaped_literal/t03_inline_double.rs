//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/11_escaped_literal/03_inline_double.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".result {\
             \n  output: #{#{l\\\\ite\\ral}};\
             \n  output: #{\"[#{l\\\\ite\\ral}]\"};\
             \n  output: #{\"#{l\\\\ite\\ral}\"};\
             \n  output: #{\'#{l\\\\ite\\ral}\'};\
             \n  output: #{\"[\'#{l\\\\ite\\ral}\']\"};\
             \n}\n"),
        ".result {\
         \n  output: l\\\\iteral;\
         \n  output: [l\\\\iteral];\
         \n  output: l\\\\iteral;\
         \n  output: l\\\\iteral;\
         \n  output: [\'l\\\\iteral\'];\
         \n}\n"
    );
}
