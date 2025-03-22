//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_1322.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("issue_1322")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("$foo: 400px;\
             \n$bar: \"min-width:400px\";\
             \n@import url(foo.css) (min-width:400px);\
             \n@import url(foo.css) (min-width:$foo);\
             \n@import url(foo.css) (min-width:#{$foo});\
             \n@import url(foo.css) ($bar);\
             \n@import url(foo.css) (#{$bar});\n"),
        "@import url(foo.css) (min-width: 400px);\
         \n@import url(foo.css) (min-width: 400px);\
         \n@import url(foo.css) (min-width: 400px);\
         \n@import url(foo.css) (min-width:400px);\
         \n@import url(foo.css) (min-width:400px);\n"
    );
}
