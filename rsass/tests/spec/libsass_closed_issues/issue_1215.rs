//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1215.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1215")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("foo {\
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
             \n}\n"),
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
         \n}\n"
    );
}
