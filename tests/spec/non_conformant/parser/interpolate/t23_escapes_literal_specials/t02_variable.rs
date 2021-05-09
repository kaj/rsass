//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/23_escapes_literal_specials/02_variable.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \\0_\\a_\\A;\
             \n.result {\
             \n  output: $input;\
             \n  output: #{$input};\
             \n  output: \"[#{$input}]\";\
             \n  output: \"#{$input}\";\
             \n  output: \'#{$input}\';\
             \n  output: \"[\'#{$input}\']\";\
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
