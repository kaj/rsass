//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/24_escapes_double_quoted_specials/01_inline.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("01_inline")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".result {\
             \n  output: \"\\0_\\a_\\A\";\
             \n  output: #{\"\\0_\\a_\\A\"};\
             \n  output: \"[#{\"\\0_\\a_\\A\"}]\";\
             \n  output: \"#{\"\\0_\\a_\\A\"}\";\
             \n  output: \'#{\"\\0_\\a_\\A\"}\';\
             \n  output: \"[\'#{\"\\0_\\a_\\A\"}\']\";\
             \n}\n"),
        "@charset \"UTF-8\";\
         \n.result {\
         \n  output: \"�_\\a_\\a\";\
         \n  output: �_ _ ;\
         \n  output: \"[�_\\a_\\a]\";\
         \n  output: \"�_\\a_\\a\";\
         \n  output: \"�_\\a_\\a\";\
         \n  output: \"[\'�_\\a_\\a\']\";\
         \n}\n"
    );
}
