//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_555.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "\
            \n@function hello($name) {\
            \n    @return $name;\
            \n}\
            \n\
            \n$foo: (\
            \n  bar() : baz,\
            \n  bar(\"foo\") : blah,\
            \n  hello(\"bob\") : bam,\
            \n);\
            \n\
            \na {\
            \n  foo: map-get($foo, \"bar()\");\
            \n  foo: map-get($foo, \"bar(\\\"foo\\\")\");\
            \n  foo: map-get($foo, \'bar(\"foo\")\');\
            \n  foo: map-get($foo, \"bob\");\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  foo: baz;\
        \n  foo: blah;\
        \n  foo: blah;\
        \n  foo: bam;\
        \n}\
        \n"
    );
}
