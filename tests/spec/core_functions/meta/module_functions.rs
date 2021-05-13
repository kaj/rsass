//! Tests auto-converted from "sass-spec/spec/core_functions/meta/module_functions.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .mock_file("_util.scss", "@use \"sass:meta\";\n\n@mixin print-function-map($functions) {\n  a {\n    @each $name, $function in $functions {\n      #{$name}: meta.call($function);\n    }\n  }\n}\n")
        .mock_file("as/_other.scss", "@function c() {@return c value}\n@function d() {@return d value}\n@function e() {@return e value}\n")
        .mock_file("dash_sensitive/_other.scss", "@function b-c() {@return b-c value}\n@function d_e() {@return d_e value}\n")
        .mock_file("empty/_other.scss", "// This module defines no functions.\n")
        .mock_file("error/before_load/_other.scss", "// This module defines no functions.\n")
        .mock_file("error/dash_sensitive/_other-module.scss", "// This module defines no functions.\n")
        .mock_file("error/global/_other.scss", "// This module defines no functions.\n")
        .mock_file("multiple/_other.scss", "@function b() {@return b value}\n@function c() {@return c value}\n@function d() {@return d value}\n")
        .mock_file("named/_other.scss", "@function b() {@return b value}\n@function c() {@return c value}\n@function d() {@return d value}\n")
        .mock_file("through_forward/as/_forwarded.scss", "@function c() {@return c value}\n@function d() {@return d value}\n@function e() {@return e value}\n")
        .mock_file("through_forward/as/_used.scss", "@forward \"forwarded\" as b-*;\n")
        .mock_file("through_forward/bare/_forwarded.scss", "@function b() {@return b value}\n@function c() {@return c value}\n@function d() {@return d value}\n")
        .mock_file("through_forward/bare/_used.scss", "@forward \"forwarded\";\n")
        .mock_file("through_forward/hide/_forwarded.scss", "@function b() {@return b value}\n@function c() {@return c value}\n@function d() {@return d value}\n")
        .mock_file("through_forward/hide/_used.scss", "@forward \"forwarded\" hide b, c;\n")
        .mock_file("through_forward/show/_forwarded.scss", "@function b() {@return b value}\n@function c() {@return c value}\n@function d() {@return d value}\n")
        .mock_file("through_forward/show/_used.scss", "@forward \"forwarded\" show b, c;\n")
        .mock_file("through_import/_imported.scss", "@function b() {@return b value}\n@function c() {@return c value}\n@function d() {@return d value}\n")
        .mock_file("through_import/_used.scss", "@import \"imported\";\n")
}

#[test]
#[ignore] // unexepected error
fn test_as() {
    assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@use \"../util\";\
             \n@use \"other\" as b;\n\
             \n@include util.print-function-map(meta.module-functions(\"b\"))\n"
        ),
        "a {\
         \n  c: c value;\
         \n  d: d value;\
         \n  e: e value;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn core_module() {
    assert_eq!(
        runner().ok(
            "@use \"sass:map\";\
             \n@use \"sass:meta\";\n\
             \n// We don\'t want to print every function name in this module, since that would\
             \n// make this test brittle when new functions are added. Instead we just test\
             \n// that a couple functions work.\n\
             \n$functions: meta.module-functions(\"meta\");\
             \na {\
             \n  variable-exists: meta.call(map.get($functions, \"variable-exists\"), \"functions\");\
             \n  inspect: meta.call(map.get($functions, \"inspect\"), ());\
             \n}\n"
        ),
        "a {\
         \n  variable-exists: true;\
         \n  inspect: ();\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn dash_sensitive() {
    assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@use \"../util\";\
             \n@use \"other\";\n\
             \n@include util.print-function-map(meta.module-functions(\"other\"));\n"
        ),
        "a {\
         \n  b-c: b-c value;\
         \n  d-e: d_e value;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn empty() {
    assert_eq!(
        runner().ok("@use \"sass:meta\";\
             \n@use \"other\";\n\
             \na {b: meta.inspect(meta.module-functions(\"other\"))}\n"),
        "a {\
         \n  b: ();\
         \n}\n"
    );
}
mod error {
    #[allow(unused)]
    use super::runner;
    #[test]
    fn before_load() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\n\
             \n$a: meta.module-functions(\"other\");\n\
             \n@use \"other\";\n"
            ),
            "Error: There is no module with namespace \"other\".\
         \n  ,\
         \n3 | $a: meta.module-functions(\"other\");\
         \n  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:5  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn dash_sensitive() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\
             \n@use \"other-module\";\n\
             \n$a: meta.module-functions(\"other_module\");\n"
            ),
            "Error: There is no module with namespace \"other_module\".\
         \n  ,\
         \n4 | $a: meta.module-functions(\"other_module\");\
         \n  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 4:5  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn global() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\
             \n@use \"other\" as *;\n\
             \n$a: meta.module-functions(\"other\");\n"
            ),
            "Error: There is no module with namespace \"other\".\
         \n  ,\
         \n4 | $a: meta.module-functions(\"other\");\
         \n  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 4:5  root stylesheet",
        );
    }
    #[test]
    fn missing() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\
             \n$a: meta.module-functions(\"other\");\n"
            ),
            "Error: There is no module with namespace \"other\".\
         \n  ,\
         \n2 | $a: meta.module-functions(\"other\");\
         \n  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:5  root stylesheet",
        );
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\
             \n$a: meta.module-functions();\n"
            ),
            "Error: Missing argument $module.\
         \n  ,--> input.scss\
         \n2 | $a: meta.module-functions();\
         \n  |     ^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function module-functions($module) {\
         \n  |           ========================= declaration\
         \n  \'\
         \n  input.scss 2:5  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\
             \n$a: meta.module-functions(\"meta\", \"c\");\n"
            ),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | $a: meta.module-functions(\"meta\", \"c\");\
         \n  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function module-functions($module) {\
         \n  |           ========================= declaration\
         \n  \'\
         \n  input.scss 2:5  root stylesheet",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            runner().err(
                "@use \"sass:meta\";\
             \n$a: meta.module-functions(1);\n"
            ),
            "Error: $module: 1 is not a string.\
         \n  ,\
         \n2 | $a: meta.module-functions(1);\
         \n  |     ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:5  root stylesheet",
        );
    }
}
#[test]
#[ignore] // unexepected error
fn multiple() {
    assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@use \"../util\";\
             \n@use \"other\";\n\
             \n@include util.print-function-map(meta.module-functions(\"other\"));\n"
        ),
        "a {\
         \n  b: b value;\
         \n  c: c value;\
         \n  d: d value;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn named() {
    assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@use \"../util\";\
             \n@use \"other\";\n\
             \n@include util.print-function-map(meta.module-functions($module: \"other\"));\n"
        ),
        "a {\
         \n  b: b value;\
         \n  c: c value;\
         \n  d: d value;\
         \n}\n"
    );
}
mod through_forward {
    #[allow(unused)]
    use super::runner;
    #[test]
    #[ignore] // unexepected error
    fn test_as() {
        assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@use \"../../util\";\
             \n@use \"used\";\n\
             \n@include util.print-function-map(meta.module-functions(\"used\"));\n"
        ),
        "a {\
         \n  b-c: c value;\
         \n  b-d: d value;\
         \n  b-e: e value;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn bare() {
        assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@use \"../../util\";\
             \n@use \"used\";\n\
             \n@include util.print-function-map(meta.module-functions(\"used\"));\n"
        ),
        "a {\
         \n  b: b value;\
         \n  c: c value;\
         \n  d: d value;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn hide() {
        assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@use \"../../util\";\
             \n@use \"used\";\n\
             \n@include util.print-function-map(meta.module-functions(\"used\"));\n"
        ),
        "a {\
         \n  d: d value;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn show() {
        assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@use \"../../util\";\
             \n@use \"used\";\n\
             \n@include util.print-function-map(meta.module-functions(\"used\"));\n"
        ),
        "a {\
         \n  b: b value;\
         \n  c: c value;\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn through_import() {
    assert_eq!(
        runner().ok(
            "@use \"sass:meta\";\
             \n@use \"../util\";\
             \n@use \"used\";\n\
             \n@include util.print-function-map(meta.module-functions(\"used\"));\n"
        ),
        "a {\
         \n  b: b value;\
         \n  c: c value;\
         \n  d: d value;\
         \n}\n"
    );
}
