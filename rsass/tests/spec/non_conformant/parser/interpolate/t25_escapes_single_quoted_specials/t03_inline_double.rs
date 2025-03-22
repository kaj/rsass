//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/25_escapes_single_quoted_specials/03_inline_double.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("03_inline_double")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".result {\
             \n  output: #{#{\'\\0_\\a_\\A\'}};\
             \n  output: #{\"[#{\'\\0_\\a_\\A\'}]\"};\
             \n  output: #{\"#{\'\\0_\\a_\\A\'}\"};\
             \n  output: #{\'#{\'\\0_\\a_\\A\'}\'};\
             \n  output: #{\"[\'#{\'\\0_\\a_\\A\'}\']\"};\
             \n}\n"),
        "@charset \"UTF-8\";\
         \n.result {\
         \n  output: �_ _ ;\
         \n  output: [�_ _ ];\
         \n  output: �_ _ ;\
         \n  output: �_ _ ;\
         \n  output: [\'�_ _ \'];\
         \n}\n"
    );
}
