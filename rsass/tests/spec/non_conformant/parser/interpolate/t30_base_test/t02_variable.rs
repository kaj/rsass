//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/30_base_test/02_variable.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("02_variable")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \"foo#{\'ba\' + \'r\'}baz\";\
             \n.result {\
             \n  output: $input;\
             \n  output: #{$input};\
             \n  output: \"[#{$input}]\";\
             \n  output: \"#{$input}\";\
             \n  output: \'#{$input}\';\
             \n  output: \"[\'#{$input}\']\";\
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
