//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_555.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_555")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("@use \"sass:map\";\n\
             \n@function hello($name) {\
             \n    @return $name;\
             \n}\n\
             \n$foo: (\
             \n  bar() : baz,\
             \n  bar(\"foo\") : blah,\
             \n  hello(\"bob\") : bam,\
             \n);\n\
             \na {\
             \n  foo: map.get($foo, \"bar()\");\
             \n  foo: map.get($foo, \"bar(\\\"foo\\\")\");\
             \n  foo: map.get($foo, \'bar(\"foo\")\');\
             \n  foo: map.get($foo, \"bob\");\
             \n}\n"),
        "a {\
         \n  foo: baz;\
         \n  foo: blah;\
         \n  foo: blah;\
         \n  foo: bam;\
         \n}\n"
    );
}
