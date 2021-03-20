//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1610.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function foo() {\
            \n  @return \"bar\";\
            \n}\
            \n\
            \n@function bar() {\
            \n    @return \"foo\" + \",\" + bar;\
            \n}\
            \n\
            \na {\
            \n  b: foo(), \"bar\";\
            \n  b: foo(), \"bar\"\
            \n}\
            \n\
            \nb {\
            \n  b: #{foo(), \"bar\"};\
            \n  b: #{foo(), \"bar\"}\
            \n}\
            \n\
            \nc {\
            \n  b: \"foo\", bar;\
            \n}\
            \n\
            \nd {\
            \n  b: #{\"foo\", bar};\
            \n  b: #{\"foo\", bar}\
            \n}\
            \n\
            \ne {\
            \n  b: #{bar()};\
            \n  b: #{bar()}\
            \n}\
            \n\
            \nf {\
            \n  b: \"foo\" + \",\" + bar;\
            \n  b: \"foo\" + \",\" + bar\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"bar\", \"bar\";\
        \n  b: \"bar\", \"bar\";\
        \n}\
        \nb {\
        \n  b: bar, bar;\
        \n  b: bar, bar;\
        \n}\
        \nc {\
        \n  b: \"foo\", bar;\
        \n}\
        \nd {\
        \n  b: foo, bar;\
        \n  b: foo, bar;\
        \n}\
        \ne {\
        \n  b: foo,bar;\
        \n  b: foo,bar;\
        \n}\
        \nf {\
        \n  b: \"foo,bar\";\
        \n  b: \"foo,bar\";\
        \n}\
        \n"
    );
}
