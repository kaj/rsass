//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_829.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".foo {\
             \n    @media (foo: bar), (bar: baz) {\
             \n        foo: bar;\n\
             \n        @media (foo: bar) {\
             \n            bar: baz;\
             \n        }\n\
             \n        .bar {\
             \n            baz: bam;\
             \n        }\
             \n    }\
             \n }\n\n"),
        "@media (foo: bar), (bar: baz) {\
         \n  .foo {\
         \n    foo: bar;\
         \n  }\
         \n}\
         \n@media (foo: bar) and (foo: bar), (bar: baz) and (foo: bar) {\
         \n  .foo {\
         \n    bar: baz;\
         \n  }\
         \n}\
         \n@media (foo: bar), (bar: baz) {\
         \n  .foo .bar {\
         \n    baz: bam;\
         \n  }\
         \n}\n"
    );
}
