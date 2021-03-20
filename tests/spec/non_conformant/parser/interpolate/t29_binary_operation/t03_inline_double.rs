//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/29_binary_operation/03_inline_double.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".result {\
            \n  output: #{#{\"foo#{\'ba\' + \'r\'}baz\"}};\
            \n  output: #{\"[#{\"foo#{\'ba\' + \'r\'}baz\"}]\"};\
            \n  output: #{\"#{\"foo#{\'ba\' + \'r\'}baz\"}\"};\
            \n  output: #{\'#{\"foo#{\'ba\' + \'r\'}baz\"}\'};\
            \n  output: #{\"[\'#{\"foo#{\'ba\' + \'r\'}baz\"}\']\"};\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: foobarbaz;\
        \n  output: [foobarbaz];\
        \n  output: foobarbaz;\
        \n  output: foobarbaz;\
        \n  output: [\'foobarbaz\'];\
        \n}\
        \n"
    );
}
