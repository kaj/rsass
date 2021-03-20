//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_558.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@function is_gold($c) {\r\
            \n    @if ($c == gold) {\r\
            \n        @return \'yes\';\r\
            \n    }\r\
            \n    @return \'no\';\r\
            \n}\r\
            \n\r\
            \ndiv {\r\
            \n    foo: is_gold(gold);\r\
            \n    bar: is_gold(white);\r\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  foo: \"yes\";\
        \n  bar: \"no\";\
        \n}\
        \n"
    );
}
