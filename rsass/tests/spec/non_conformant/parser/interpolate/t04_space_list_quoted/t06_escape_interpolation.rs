//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/04_space_list_quoted/06_escape_interpolation.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("06_escape_interpolation")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$input: \"alpha\" \'beta\';\
             \n.result {\
             \n  output: \"[\\#{\"alpha\" \'beta\'}]\";\
             \n  output: \"\\#{\"alpha\" \'beta\'}\";\
             \n  output: \'\\#{\"alpha\" \'beta\'}\';\
             \n  output: \"[\'\\#{\"alpha\" \'beta\'}\']\";\
             \n}\n"),
        ".result {\
         \n  output: \"[#{\" alpha \" \'beta\'}]\";\
         \n  output: \"#{\" alpha \" \'beta\'}\";\
         \n  output: \'#{\"alpha\" \' beta \"}\";\
         \n  output: \"[\'#{\" alpha \" \'beta\'}\']\";\
         \n}\n"
    );
}
