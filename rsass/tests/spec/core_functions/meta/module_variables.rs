//! Tests auto-converted from "sass-spec/spec/core_functions/meta/module_variables.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("module_variables")
        .mock_file(
            "as/_other.scss",
            "$d: d value;\n$e: e value;\n$f: f value;\n",
        )
        .mock_file(
            "dash_sensitive/_other.scss",
            "$c-d: c-d value;\n$e_f: e_f value;\n",
        )
        .mock_file(
            "empty/_other.scss",
            "// This module defines no variables.\n",
        )
        .mock_file(
            "error/before_load/_other.scss",
            "// This module defines no variables.\n",
        )
        .mock_file(
            "error/dash_sensitive/_other-module.scss",
            "// This module defines no variables.\n",
        )
        .mock_file(
            "error/global/_other.scss",
            "// This module defines no variables.\n",
        )
        .mock_file(
            "multiple/_other.scss",
            "$c: c value;\n$d: d value;\n$e: e value;\n",
        )
        .mock_file(
            "named/_other.scss",
            "$c: c value;\n$d: d value;\n$e: e value;\n",
        )
        .mock_file(
            "through_forward/as/_forwarded.scss",
            "$d: d value;\n$e: e value;\n$f: f value;\n",
        )
        .mock_file(
            "through_forward/as/_used.scss",
            "@forward \"forwarded\" as c-*;\n",
        )
        .mock_file(
            "through_forward/bare/_forwarded.scss",
            "$c: c value;\n$d: d value;\n$e: e value;\n",
        )
        .mock_file(
            "through_forward/bare/_used.scss",
            "@forward \"forwarded\";\n",
        )
        .mock_file(
            "through_forward/hide/_forwarded.scss",
            "$c: c value;\n$d: d value;\n$e: e value;\n",
        )
        .mock_file(
            "through_forward/hide/_used.scss",
            "@forward \"forwarded\" hide $c, $d;\n",
        )
        .mock_file(
            "through_forward/show/_forwarded.scss",
            "$c: c value;\n$d: d value;\n$e: e value;\n",
        )
        .mock_file(
            "through_forward/show/_used.scss",
            "@forward \"forwarded\" show $c, $d;\n",
        )
        .mock_file(
            "through_import/_imported.scss",
            "$c: c value;\n$d: d value;\n$e: e value;\n",
        )
        .mock_file("through_import/_used.scss", "@import \"imported\";\n")
}

#[test]
fn test_as() {
    let runner = runner().with_cwd("as");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \n@use \"other\" as a;\n\
             \nb {c: meta.inspect(meta.module-variables(\"a\"))}\n"),
        "b {\
         \n  c: (\"d\": d value, \"e\": e value, \"f\": f value);\
         \n}\n"
    );
}
#[test]
fn core_module() {
    let runner = runner().with_cwd("core_module");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\n\
             \na {b: meta.inspect(meta.module-variables(\"meta\"))}\n"),
        "a {\
         \n  b: ();\
         \n}\n"
    );
}
#[test]
fn dash_sensitive() {
    let runner = runner().with_cwd("dash_sensitive");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \n@use \"other\";\n\
             \na {b: meta.inspect(meta.module-variables(\"other\"))}\n"),
        "a {\
         \n  b: (\"c-d\": c-d value, \"e-f\": e_f value);\
         \n}\n"
    );
}
#[test]
fn empty() {
    let runner = runner().with_cwd("empty");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \n@use \"other\";\n\
             \na {b: meta.inspect(meta.module-variables(\"other\"))}\n"),
        "a {\
         \n  b: ();\
         \n}\n"
    );
}
mod error {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("error")
    }

    #[test]
    fn before_load() {
        let runner = runner().with_cwd("before_load");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\n\
             \n$variables: meta.module-variables(\"other\");\n\
             \n@use \"other\";\n"
            ),
            "Error: There is no module with namespace \"other\".\
         \n  ,\
         \n3 | $variables: meta.module-variables(\"other\");\
         \n  |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 3:13  root stylesheet",
        );
    }
    #[test]
    fn dash_sensitive() {
        let runner = runner().with_cwd("dash_sensitive");
        assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \n@use \"other-module\";\n\
             \na {b: meta.inspect(meta.module-variables(\"other_module\"))}\n"
        ),
        "Error: There is no module with namespace \"other_module\".\
         \n  ,\
         \n4 | a {b: meta.inspect(meta.module-variables(\"other_module\"))}\
         \n  |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 4:20  root stylesheet",
    );
    }
    #[test]
    fn global() {
        let runner = runner().with_cwd("global");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \n@use \"other\" as *;\n\
             \na {b: meta.inspect(meta.module-variables(\"other\"))}\n"
            ),
            "Error: There is no module with namespace \"other\".\
         \n  ,\
         \n4 | a {b: meta.inspect(meta.module-variables(\"other\"))}\
         \n  |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 4:20  root stylesheet",
        );
    }
    #[test]
    fn missing() {
        let runner = runner().with_cwd("missing");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \na {b: meta.inspect(meta.module-variables(\"other\"))}\n"
            ),
            "Error: There is no module with namespace \"other\".\
         \n  ,\
         \n2 | a {b: meta.inspect(meta.module-variables(\"other\"))}\
         \n  |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:20  root stylesheet",
        );
    }
    #[test]
    fn too_few_args() {
        let runner = runner().with_cwd("too_few_args");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \na {b: meta.inspect(meta.module-variables())}\n"
            ),
            "Error: Missing argument $module.\
         \n  ,--> input.scss\
         \n2 | a {b: meta.inspect(meta.module-variables())}\
         \n  |                    ^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function module-variables($module) {\
         \n  |           ========================= declaration\
         \n  \'\
         \n  input.scss 2:20  root stylesheet",
        );
    }
    #[test]
    fn too_many_args() {
        let runner = runner().with_cwd("too_many_args");
        assert_eq!(
        runner.err(
            "@use \"sass:meta\";\
             \na {b: meta.inspect(meta.module-variables(\"meta\", \"c\"))}\n"
        ),
        "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n2 | a {b: meta.inspect(meta.module-variables(\"meta\", \"c\"))}\
         \n  |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:meta\
         \n1 | @function module-variables($module) {\
         \n  |           ========================= declaration\
         \n  \'\
         \n  input.scss 2:20  root stylesheet",
    );
    }
    #[test]
    fn test_type() {
        let runner = runner().with_cwd("type");
        assert_eq!(
            runner.err(
                "@use \"sass:meta\";\
             \na {b: meta.inspect(meta.module-variables(1))}\n"
            ),
            "Error: $module: 1 is not a string.\
         \n  ,\
         \n2 | a {b: meta.inspect(meta.module-variables(1))}\
         \n  |                    ^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:20  root stylesheet",
        );
    }
}
#[test]
fn multiple() {
    let runner = runner().with_cwd("multiple");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \n@use \"other\";\n\
             \na {b: meta.inspect(meta.module-variables(\"other\"))}\n"),
        "a {\
         \n  b: (\"c\": c value, \"d\": d value, \"e\": e value);\
         \n}\n"
    );
}
#[test]
fn named() {
    let runner = runner().with_cwd("named");
    assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@use \"other\";\n\
             \na {b: meta.inspect(meta.module-variables($module: \"other\"))}\n"
        ),
        "a {\
         \n  b: (\"c\": c value, \"d\": d value, \"e\": e value);\
         \n}\n"
    );
}
mod through_forward {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("through_forward")
    }

    #[test]
    fn test_as() {
        let runner = runner().with_cwd("as");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@use \"used\";\n\
             \na {b: meta.inspect(meta.module-variables(\"used\"))}\n"),
            "a {\
         \n  b: (\"c-d\": d value, \"c-e\": e value, \"c-f\": f value);\
         \n}\n"
        );
    }
    #[test]
    fn bare() {
        let runner = runner().with_cwd("bare");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@use \"used\";\n\
             \na {b: meta.inspect(meta.module-variables(\"used\"))}\n"),
            "a {\
         \n  b: (\"c\": c value, \"d\": d value, \"e\": e value);\
         \n}\n"
        );
    }
    #[test]
    fn hide() {
        let runner = runner().with_cwd("hide");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@use \"used\";\n\
             \na {b: meta.inspect(meta.module-variables(\"used\"))}\n"),
            "a {\
         \n  b: (\"e\": e value);\
         \n}\n"
        );
    }
    #[test]
    fn show() {
        let runner = runner().with_cwd("show");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@use \"used\";\n\
             \na {b: meta.inspect(meta.module-variables(\"used\"))}\n"),
            "a {\
         \n  b: (\"c\": c value, \"d\": d value);\
         \n}\n"
        );
    }
}
#[test]
fn through_import() {
    let runner = runner().with_cwd("through_import");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \n@use \"used\";\n\
             \na {b: meta.inspect(meta.module-variables(\"used\"))}\n"),
        "a {\
         \n  b: (\"c\": c value, \"d\": d value, \"e\": e value);\
         \n}\n"
    );
}
