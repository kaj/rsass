//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/12_escaped_double_quoted/03_inline_double.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("03_inline_double")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".result {\
             \n  output: #{#{\"l\\\\ite\\ral\"}};\
             \n  output: #{\"[#{\"l\\\\ite\\ral\"}]\"};\
             \n  output: #{\"#{\"l\\\\ite\\ral\"}\"};\
             \n  output: #{\'#{\"l\\\\ite\\ral\"}\'};\
             \n  output: #{\"[\'#{\"l\\\\ite\\ral\"}\']\"};\
             \n}\n"),
        ".result {\
         \n  output: l\\iteral;\
         \n  output: [l\\iteral];\
         \n  output: l\\iteral;\
         \n  output: l\\iteral;\
         \n  output: [\'l\\iteral\'];\
         \n}\n"
    );
}
