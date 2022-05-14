//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_694.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_694")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("// test for libsass 694:\
             \n// parser should be smarter about handling quoted quotes\n\
             \n$str: \'{\' + \'\"foo\": \"bar\"\' + \'}\';\
             \n$str2: \'\"hello world\"\';\
             \n$str3: \"hello world\";\
             \n.interpolation-test {\
             \n  test: \"#{$str}\";\
             \n  test: \"#{$str2}\";\
             \n  test: \"#{$str3}\";\
             \n}\n"),
        ".interpolation-test {\
         \n  test: \'{\"foo\": \"bar\"}\';\
         \n  test: \'\"hello world\"\';\
         \n  test: \"hello world\";\
         \n}\n"
    );
}
