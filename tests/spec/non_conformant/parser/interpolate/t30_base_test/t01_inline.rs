//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/30_base_test/01_inline.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".result {\
            \n  output: \"foo#{\'ba\' + \'r\'}baz\";\
            \n  output: #{\"foo#{\'ba\' + \'r\'}baz\"};\
            \n  output: \"[#{\"foo#{\'ba\' + \'r\'}baz\"}]\";\
            \n  output: \"#{\"foo#{\'ba\' + \'r\'}baz\"}\";\
            \n  output: \'#{\"foo#{\'ba\' + \'r\'}baz\"}\';\
            \n  output: \"[\'#{\"foo#{\'ba\' + \'r\'}baz\"}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \"foobarbaz\";\
        \n  output: foobarbaz;\
        \n  output: \"[foobarbaz]\";\
        \n  output: \"foobarbaz\";\
        \n  output: \"foobarbaz\";\
        \n  output: \"[\'foobarbaz\']\";\
        \n}\
        \n"
    );
}
