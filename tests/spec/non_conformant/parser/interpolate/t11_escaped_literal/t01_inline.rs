//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/11_escaped_literal/01_inline.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("01_inline")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(".result {\
             \n  output: l\\\\ite\\ral;\
             \n  output: #{l\\\\ite\\ral};\
             \n  output: \"[#{l\\\\ite\\ral}]\";\
             \n  output: \"#{l\\\\ite\\ral}\";\
             \n  output: \'#{l\\\\ite\\ral}\';\
             \n  output: \"[\'#{l\\\\ite\\ral}\']\";\
             \n}\n"),
        ".result {\
         \n  output: l\\\\iteral;\
         \n  output: l\\\\iteral;\
         \n  output: \"[l\\\\\\\\iteral]\";\
         \n  output: \"l\\\\\\\\iteral\";\
         \n  output: \"l\\\\\\\\iteral\";\
         \n  output: \"[\'l\\\\\\\\iteral\']\";\
         \n}\n"
    );
}
