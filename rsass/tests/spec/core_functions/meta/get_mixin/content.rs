//! Tests auto-converted from "sass-spec/spec/core_functions/meta/get_mixin/content.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("content")
}

mod error {
    #[allow(unused)]
    use super::runner;

    mod denies_content {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // wrong error
        fn builtin() {
            assert_eq!(
        runner().err(
            "@use \"sass:meta\";\n\
             \na {\
             \n  @include meta.apply(meta.get-mixin(load-css, meta), \"a\") {\
             \n    a: b;\
             \n  }\
             \n}\n"
        ),
        "Error: Mixin doesn\'t accept a content block.\
         \n  ,--> input.scss\
         \n4 |   @include meta.apply(meta.get-mixin(load-css, meta), \"a\") {\
         \n  |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @mixin load-css($url, $with: null) {\
         \n  |        =========================== declaration\
         \n  \'\
         \n  input.scss 4:3  root stylesheet",
    );
        }
        #[test]
        #[ignore] // wrong error
        fn user_defined() {
            assert_eq!(
                runner().err(
                    "@use \"sass:meta\";\
             \n@mixin a {}\n\
             \na {\
             \n  @include meta.apply(meta.get-mixin(\"a\")) {}\
             \n}\n"
                ),
                "Error: Mixin doesn\'t accept a content block.\
         \n    ,\
         \n2   | @mixin a {}\
         \n    |        = declaration\
         \n... |\
         \n5   |   @include meta.apply(meta.get-mixin(\"a\")) {}\
         \n    |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n    \'\
         \n  input.scss 5:3  root stylesheet",
            );
        }
    }
}
#[test]
#[ignore] // unexepected error
fn passes_content() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\n\
             \n@mixin a {\
             \n  b {@content}\
             \n}\n\
             \na {\
             \n  @include meta.apply(meta.get-mixin(\"a\")) {\
             \n    b: red;\
             \n  }\
             \n}\n"),
        "a b {\
         \n  b: red;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn passes_empty_content() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\n\
             \n@mixin a {@content;}\n\
             \na {\
             \n  @include meta.apply(meta.get-mixin(\"a\")) {}\
             \n}\n"),
        ""
    );
}
mod scope {
    #[allow(unused)]
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn fall_through() {
        assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n$global: global;\n\
             \n@mixin a {\
             \n  @content(content-rule-a);\
             \n  global: $global;\
             \n}\n\
             \n@mixin b {\
             \n  $global: in-mixin-b;\
             \n  @include meta.apply(meta.get-mixin(a)) using ($content-arg) {\
             \n    @content($content-arg);\
             \n  }\
             \n}\n\
             \n@mixin c {\
             \n  $global: in-mixin-c;\
             \n  @include meta.apply(meta.get-mixin(b)) using ($content-arg) {\
             \n    @content($content-arg);\
             \n  }\
             \n}\n\
             \na {\
             \n  $global: in-style-rule;\
             \n  @include meta.apply(meta.get-mixin(c)) using ($content-arg) {\
             \n    in-content-body: $content-arg;\
             \n  }\
             \n}\n"
        ),
        "a {\
         \n  in-content-body: content-rule-a;\
         \n  global: global;\
         \n}\n"
    );
    }
    mod redeclare {
        #[allow(unused)]
        use super::runner;

        #[test]
        #[ignore] // unexepected error
        fn using() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n@mixin a($a: param) {@content(content-arg);}\n\
             \n$a: global;\n\
             \na {\
             \n  $a: in-style-rule;\
             \n  @include meta.apply(meta.get-mixin(\"a\")) using ($a) {\
             \n    in-content-body: $a;\
             \n    $a: in-content-body;\
             \n  }\
             \n  in-style-rule: $a;\
             \n}\n"),
                "a {\
         \n  in-content-body: content-arg;\
         \n  in-style-rule: in-style-rule;\
         \n}\n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn vars() {
            assert_eq!(
                runner().ok("@use \"sass:meta\";\
             \n@mixin a($param: param) {\
             \n  $in-mixin: in-mixin;\
             \n  @content;\
             \n  param: $param;\
             \n  in-mixin: $in-mixin;\
             \n}\n\
             \n$global: global;\n\
             \na {\
             \n  $in-style-rule: in-style-rule;\
             \n  @include meta.apply(meta.get-mixin(\"a\")) {\
             \n    $param: in-include;\
             \n    $in-mixin: in-include;\
             \n    $global: in-include;\
             \n    $in-style-rule: in-include;\
             \n  }\
             \n  global: $global;\
             \n  in-style-rule: $in-style-rule;\
             \n}\n"),
                "a {\
         \n  param: param;\
         \n  in-mixin: in-mixin;\
         \n  global: global;\
         \n  in-style-rule: in-include;\
         \n}\n"
            );
        }
    }
}
