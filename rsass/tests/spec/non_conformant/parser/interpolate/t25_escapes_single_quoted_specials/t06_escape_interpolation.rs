//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/25_escapes_single_quoted_specials/06_escape_interpolation.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("06_escape_interpolation")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \'\\0_\\a_\\A\';\
             \n.result {\
             \n  output: \"[\\#{\'\\0_\\a_\\A\'}]\";\
             \n  output: \"\\#{\'\\0_\\a_\\A\'}\";\
             \n  output: \'\\#{\'\\0_\\a_\\A\'}\';\
             \n  output: \"[\'\\#{\'\\0_\\a_\\A\'}\']\";\
             \n}\n"),
        "@charset \"UTF-8\";\
         \n.result {\
         \n  output: \"[#{\'�_\\a_\\a\'}]\";\
         \n  output: \"#{\'�_\\a_\\a\'}\";\
         \n  output: \"#{\" \\0 _\\a _\\a  \"}\";\
         \n  output: \"[\'#{\'�_\\a_\\a\'}\']\";\
         \n}\n"
    );
}
