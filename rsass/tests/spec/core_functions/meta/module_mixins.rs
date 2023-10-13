//! Tests auto-converted from "sass-spec/spec/core_functions/meta/module_mixins.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("module_mixins")
        .mock_file("_utils.scss", "@use \"sass:meta\";\n\n@mixin print-mixin-map($mixins) {\n  a {\n    @each $name, $mixin in $mixins {\n      #{$name}: {@include meta.apply($mixin)};\n    }\n  }\n}\n")
        .mock_file("as/_other.scss", "@mixin c() {c: value}\n@mixin d() {d: value}\n@mixin e() {e: value}\n")
        .mock_file("dash_sensitive/_other.scss", "@mixin b-c() {b-c: value}\n@mixin d_e() {d_e: value}\n")
        .mock_file("empty/_other.scss", "// This module defines no mixins.\n")
        .mock_file("error/before_load/_other.scss", "// This module defines no mixins.\n")
        .mock_file("error/dash_sensitive/_other-module.scss", "// This module defines no mixins.\n")
        .mock_file("error/global/_other.scss", "// This module defines no mixins.\n")
        .mock_file("multiple/_other.scss", "@mixin b() {b: value}\n@mixin c() {c: value}\n@mixin d() {d: value}\n")
        .mock_file("named/_other.scss", "@mixin b() {b: value}\n@mixin c() {c: value}\n@mixin d() {d: value}\n")
        .mock_file("return_type/user_defined/_other.scss", "// This module defines no mixins.\n")
        .mock_file("through_forward/as/_forwarded.scss", "@mixin c() {c: value}\n@mixin d() {d: value}\n@mixin e() {e: value}\n")
        .mock_file("through_forward/as/_used.scss", "@forward \"forwarded\" as b-*;\n")
        .mock_file("through_forward/bare/_forwarded.scss", "@mixin b() {b: value}\n@mixin c() {c: value}\n@mixin d() {d: value}\n")
        .mock_file("through_forward/bare/_used.scss", "@forward \"forwarded\";\n")
        .mock_file("through_forward/hide/_forwarded.scss", "@mixin b() {b: value}\n@mixin c() {c: value}\n@mixin d() {d: value}\n")
        .mock_file("through_forward/hide/_used.scss", "@forward \"forwarded\" hide b, c;\n")
        .mock_file("through_forward/show/_forwarded.scss", "@mixin b() {b: value}\n@mixin c() {c: value}\n@mixin d() {d: value}\n")
        .mock_file("through_forward/show/_used.scss", "@forward \"forwarded\" show b, c;\n")
        .mock_file("through_import/_imported.scss", "@mixin b() {b: value}\n@mixin c() {c: value}\n@mixin d() {d: value}\n")
        .mock_file("through_import/_used.scss", "@import \"imported\";\n")
}

#[test]
#[ignore] // unexepected error
fn test_as() {
    let runner = runner().with_cwd("as");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \n@use \"core_functions/meta/module_mixins/utils\";\
             \n@use \"other\" as b;\n\
             \n@include utils.print-mixin-map(meta.module-mixins(\"b\"))\n"),
        "a {\
         \n  c-c: value;\
         \n  d-d: value;\
         \n  e-e: value;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn core_module() {
    let runner = runner().with_cwd("core_module");
    assert_eq!(
        runner.ok(
            "@use \"sass:map\";\
             \n@use \"sass:meta\";\n\
             \n// We don\'t want to print every mixin name in this module, since that would\
             \n// make this test brittle when new mixins are added. Instead we just test\
             \n// that a couple mixins work.\n\
             \n$mixins: meta.module-mixins(\"meta\");\
             \na {\
             \n  load-css-exists: map.has-key($mixins, \"load-css\");\
             \n}\n"
        ),
        "a {\
         \n  load-css-exists: true;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn dash_sensitive() {
    let runner = runner().with_cwd("dash_sensitive");
    assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@use \"core_functions/meta/module_mixins/utils\";\
             \n@use \"other\";\n\
             \n@include utils.print-mixin-map(meta.module-mixins(\"other\"));\n"
        ),
        "a {\
         \n  b-c-b-c: value;\
         \n  d-e-d_e: value;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn empty() {
    let runner = runner().with_cwd("empty");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \n@use \"other\";\n\
             \na {b: meta.inspect(meta.module-mixins(\"other\"))}\n"),
        "a {\
         \n  b: ();\
         \n}\n"
    );
}
mod error {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("error")
    }

    #[test]
    #[ignore] // wrong error
    fn before_load() {
        let runner = runner().with_cwd("before_load");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\n\
             \n$a: meta.module-mixins(\"other\");\n\
             \n@use \"other\";\n"
            ),
            "Error: There is no module with namespace \"other\".\
         \n  ,\
         \n3 | $a: meta.module-mixins(\"other\");\
         \n  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:5  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn dash_sensitive() {
        let runner = runner().with_cwd("dash_sensitive");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \n@use \"other-module\";\n\
             \n$a: meta.module-mixins(\"other_module\");\n"
            ),
            "Error: There is no module with namespace \"other_module\".\
         \n  ,\
         \n4 | $a: meta.module-mixins(\"other_module\");\
         \n  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 4:5  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn global() {
        let runner = runner().with_cwd("global");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \n@use \"other\" as *;\n\
             \n$a: meta.module-mixins(\"other\");\n"
            ),
            "Error: There is no module with namespace \"other\".\
         \n  ,\
         \n4 | $a: meta.module-mixins(\"other\");\
         \n  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 4:5  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn missing() {
        let runner = runner().with_cwd("missing");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \n$a: meta.module-mixins(\"other\");\n"
            ),
            "Error: There is no module with namespace \"other\".\
         \n  ,\
         \n2 | $a: meta.module-mixins(\"other\");\
         \n  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:5  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn too_few_args() {
        let runner = runner().with_cwd("too_few_args");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \n$a: meta.module-mixins();\n"
            ),
            "Error: Missing argument $module.\
         \n  ,--> input.scss\
         \n2 | $a: meta.module-mixins();\
         \n  |     ^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function module-mixins($module) {\
         \n  |           ====================== declaration\
         \n  \'\
         \n  input.scss 2:5  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn too_many_args() {
        let runner = runner().with_cwd("too_many_args");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \n$a: meta.module-mixins(\"meta\", \"c\");\n"
            ),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | $a: meta.module-mixins(\"meta\", \"c\");\
         \n  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function module-mixins($module) {\
         \n  |           ====================== declaration\
         \n  \'\
         \n  input.scss 2:5  root stylesheet",
        );
    }
    #[test]
    #[ignore] // wrong error
    fn test_type() {
        let runner = runner().with_cwd("type");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \n$a: meta.module-mixins(1);\n"
            ),
            "Error: $module: 1 is not a string.\
         \n  ,\
         \n2 | $a: meta.module-mixins(1);\
         \n  |     ^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:5  root stylesheet",
        );
    }
}
#[test]
#[ignore] // unexepected error
fn multiple() {
    let runner = runner().with_cwd("multiple");
    assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@use \"core_functions/meta/module_mixins/utils\";\
             \n@use \"other\";\n\
             \n@include utils.print-mixin-map(meta.module-mixins(\"other\"));\n"
        ),
        "a {\
         \n  b-b: value;\
         \n  c-c: value;\
         \n  d-d: value;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn named() {
    let runner = runner().with_cwd("named");
    assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@use \"core_functions/meta/module_mixins/utils\";\
             \n@use \"other\";\n\
             \n@include utils.print-mixin-map(meta.module-mixins($module: \"other\"));\n"
        ),
        "a {\
         \n  b-b: value;\
         \n  c-c: value;\
         \n  d-d: value;\
         \n}\n"
    );
}
mod return_type {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("return_type")
    }

    #[test]
    #[ignore] // unexepected error
    fn builtin() {
        let runner = runner().with_cwd("builtin");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\n\
             \na {b: meta.type-of(meta.module-mixins(\"meta\"))}\n"),
            "a {\
         \n  b: map;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn user_defined() {
        let runner = runner().with_cwd("user_defined");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@use \"other\";\n\
             \na {b: meta.type-of(meta.module-mixins(\"other\"))}\n"),
            "a {\
         \n  b: map;\
         \n}\n"
        );
    }
}
mod through_forward {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("through_forward")
    }

    #[test]
    #[ignore] // unexepected error
    fn test_as() {
        let runner = runner().with_cwd("as");
        assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@use \"../../utils\";\
             \n@use \"used\";\n\
             \n@include utils.print-mixin-map(meta.module-mixins(\"used\"));\n"
        ),
        "a {\
         \n  b-c-c: value;\
         \n  b-d-d: value;\
         \n  b-e-e: value;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn bare() {
        let runner = runner().with_cwd("bare");
        assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@use \"../../utils\";\
             \n@use \"used\";\n\
             \n@include utils.print-mixin-map(meta.module-mixins(\"used\"));\n"
        ),
        "a {\
         \n  b-b: value;\
         \n  c-c: value;\
         \n  d-d: value;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn hide() {
        let runner = runner().with_cwd("hide");
        assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@use \"../../utils\";\
             \n@use \"used\";\n\
             \n@include utils.print-mixin-map(meta.module-mixins(\"used\"));\n"
        ),
        "a {\
         \n  d-d: value;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn show() {
        let runner = runner().with_cwd("show");
        assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@use \"../../utils\";\
             \n@use \"used\";\n\
             \n@include utils.print-mixin-map(meta.module-mixins(\"used\"));\n"
        ),
        "a {\
         \n  b-b: value;\
         \n  c-c: value;\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn through_import() {
    let runner = runner().with_cwd("through_import");
    assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@use \"core_functions/meta/module_mixins/utils\";\
             \n@use \"used\";\n\
             \n@include utils.print-mixin-map(meta.module-mixins(\"used\"));\n"
        ),
        "a {\
         \n  b-b: value;\
         \n  c-c: value;\
         \n  d-d: value;\
         \n}\n"
    );
}
