//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/23_escapes_literal_specials/01_inline.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("01_inline")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".result {\
             \n  output: \\0_\\a_\\A;\
             \n  output: #{\\0_\\a_\\A};\
             \n  output: \"[#{\\0_\\a_\\A}]\";\
             \n  output: \"#{\\0_\\a_\\A}\";\
             \n  output: \'#{\\0_\\a_\\A}\';\
             \n  output: \"[\'#{\\0_\\a_\\A}\']\";\
             \n}\n"),
        ".result {\
         \n  output: \\0 _\\a _\\a ;\
         \n  output: \\0 _\\a _\\a ;\
         \n  output: \"[\\\\0 _\\\\a _\\\\a ]\";\
         \n  output: \"\\\\0 _\\\\a _\\\\a \";\
         \n  output: \"\\\\0 _\\\\a _\\\\a \";\
         \n  output: \"[\'\\\\0 _\\\\a _\\\\a \']\";\
         \n}\n"
    );
}
