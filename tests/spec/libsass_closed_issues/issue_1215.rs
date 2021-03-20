//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1215.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "foo {\
            \n  -quotes: \'this-string\' == \'this-string\';\
            \n  -quotes: this-string == \'this-string\';\
            \n  -quotes: \'this-string\' == \"this-string\";\
            \n  -quotes: \'this-string\' == \'\"this-string\"\';\
            \n  -quotes: \'\"this-string\"\' == \"\'this-string\'\";\
            \n  foo: this-string;\
            \n  foo: \'this-string\';\
            \n  foo: \"this-string\";\
            \n  foo: \'\"this-string\"\';\
            \n  foo: \"\'this-string\'\";\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  -quotes: true;\
        \n  -quotes: true;\
        \n  -quotes: true;\
        \n  -quotes: false;\
        \n  -quotes: false;\
        \n  foo: this-string;\
        \n  foo: \"this-string\";\
        \n  foo: \"this-string\";\
        \n  foo: \'\"this-string\"\';\
        \n  foo: \"\'this-string\'\";\
        \n}\
        \n"
    );
}
