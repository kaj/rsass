//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/04_space_list_quoted/01_inline.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".result {\
             \n  output: \"alpha\" \'beta\';\
             \n  output: #{\"alpha\" \'beta\'};\
             \n  output: \"[#{\"alpha\" \'beta\'}]\";\
             \n  output: \"#{\"alpha\" \'beta\'}\";\
             \n  output: \'#{\"alpha\" \'beta\'}\';\
             \n  output: \"[\'#{\"alpha\" \'beta\'}\']\";\
             \n}\n"),
        ".result {\
         \n  output: \"alpha\" \"beta\";\
         \n  output: alpha beta;\
         \n  output: \"[alpha beta]\";\
         \n  output: \"alpha beta\";\
         \n  output: \"alpha beta\";\
         \n  output: \"[\'alpha beta\']\";\
         \n}\n"
    );
}
