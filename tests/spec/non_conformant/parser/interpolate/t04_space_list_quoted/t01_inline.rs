//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/04_space_list_quoted/01_inline.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".result {\
            \n  output: \"alpha\" \'beta\';\
            \n  output: #{\"alpha\" \'beta\'};\
            \n  output: \"[#{\"alpha\" \'beta\'}]\";\
            \n  output: \"#{\"alpha\" \'beta\'}\";\
            \n  output: \'#{\"alpha\" \'beta\'}\';\
            \n  output: \"[\'#{\"alpha\" \'beta\'}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \"alpha\" \"beta\";\
        \n  output: alpha beta;\
        \n  output: \"[alpha beta]\";\
        \n  output: \"alpha beta\";\
        \n  output: \"alpha beta\";\
        \n  output: \"[\'alpha beta\']\";\
        \n}\
        \n"
    );
}
