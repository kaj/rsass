//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/29_binary_operation/03_inline_double.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("03_inline_double")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".result {\
             \n  output: #{#{\"foo#{\'ba\' + \'r\'}baz\"}};\
             \n  output: #{\"[#{\"foo#{\'ba\' + \'r\'}baz\"}]\"};\
             \n  output: #{\"#{\"foo#{\'ba\' + \'r\'}baz\"}\"};\
             \n  output: #{\'#{\"foo#{\'ba\' + \'r\'}baz\"}\'};\
             \n  output: #{\"[\'#{\"foo#{\'ba\' + \'r\'}baz\"}\']\"};\
             \n}\n"),
        ".result {\
         \n  output: foobarbaz;\
         \n  output: [foobarbaz];\
         \n  output: foobarbaz;\
         \n  output: foobarbaz;\
         \n  output: [\'foobarbaz\'];\
         \n}\n"
    );
}
