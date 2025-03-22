//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/30_base_test/01_inline.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("01_inline")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".result {\
             \n  output: \"foo#{\'ba\' + \'r\'}baz\";\
             \n  output: #{\"foo#{\'ba\' + \'r\'}baz\"};\
             \n  output: \"[#{\"foo#{\'ba\' + \'r\'}baz\"}]\";\
             \n  output: \"#{\"foo#{\'ba\' + \'r\'}baz\"}\";\
             \n  output: \'#{\"foo#{\'ba\' + \'r\'}baz\"}\';\
             \n  output: \"[\'#{\"foo#{\'ba\' + \'r\'}baz\"}\']\";\
             \n}\n"),
        ".result {\
         \n  output: \"foobarbaz\";\
         \n  output: foobarbaz;\
         \n  output: \"[foobarbaz]\";\
         \n  output: \"foobarbaz\";\
         \n  output: \"foobarbaz\";\
         \n  output: \"[\'foobarbaz\']\";\
         \n}\n"
    );
}
