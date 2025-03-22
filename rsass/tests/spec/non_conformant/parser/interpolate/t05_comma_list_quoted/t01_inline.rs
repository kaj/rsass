//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/05_comma_list_quoted/01_inline.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("01_inline")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".result {\
             \n  output: \"alpha\", \'beta\';\
             \n  output: #{\"alpha\", \'beta\'};\
             \n  output: \"[#{\"alpha\", \'beta\'}]\";\
             \n  output: \"#{\"alpha\", \'beta\'}\";\
             \n  output: \'#{\"alpha\", \'beta\'}\';\
             \n  output: \"[\'#{\"alpha\", \'beta\'}\']\";\
             \n}\n"),
        ".result {\
         \n  output: \"alpha\", \"beta\";\
         \n  output: alpha, beta;\
         \n  output: \"[alpha, beta]\";\
         \n  output: \"alpha, beta\";\
         \n  output: \"alpha, beta\";\
         \n  output: \"[\'alpha, beta\']\";\
         \n}\n"
    );
}
