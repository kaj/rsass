//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1192.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1192")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\n\
             \n$keyword: foobar;\n\
             \n@mixin test($arglist...){\
             \n  $map: meta.keywords($arglist);\
             \n  /*#{meta.inspect($map)}*/\
             \n  /*#{meta.inspect($arglist)}*/\
             \n}\n\
             \n// Works\
             \n@include test(foo, bar, baz);\
             \n// Ruby Sass:  /*foo, bar, baz*/\
             \n// LibSass  :  /*foo, bar, baz*/\n\
             \n// LibSass does not inspect as ()\
             \n@include test;\
             \n// Ruby Sass:  /*()*/\
             \n// LibSass  :  /**/\n\
             \n// Ruby Sass throws error – LibSass shows keywords in arglist\
             \n// (keywords should not show in arglist – see below)\
             \n@include test(foo, bar, baz, $keyword: keyword);\
             \n// Ruby Sass:  \"Mixin test1 doesn\'t have an argument named $keyword.\"\
             \n// LibSass  :  /*foo, bar, baz, $keyword: keyword*/"
        ),
        "/*()*/\
         \n/*foo, bar, baz*/\
         \n/*()*/\
         \n/*()*/\
         \n/*(keyword: keyword)*/\
         \n/*foo, bar, baz*/\n"
    );
}
