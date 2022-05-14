//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/05_comma_list_quoted/06_escape_interpolation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("06_escape_interpolation")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \"alpha\", \'beta\';\
             \n.result {\
             \n  output: \"[\\#{\"alpha\", \'beta\'}]\";\
             \n  output: \"\\#{\"alpha\", \'beta\'}\";\
             \n  output: \'\\#{\"alpha\", \'beta\'}\';\
             \n  output: \"[\'\\#{\"alpha\", \'beta\'}\']\";\
             \n}\n"),
        ".result {\
         \n  output: \"[#{\" alpha \", \'beta\'}]\";\
         \n  output: \"#{\" alpha \", \'beta\'}\";\
         \n  output: \'#{\"alpha\", \' beta \"}\";\
         \n  output: \"[\'#{\" alpha \", \'beta\'}\']\";\
         \n}\n"
    );
}
