//! Tests auto-converted from "sass-spec/spec/core_functions/meta/global_variable_exists/different_module.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("different_module")
        .mock_file("chosen_prefix/_other.scss", "$d: null;\n")
        .mock_file("defined/module_value/_other.scss", "$c: null;\n")
        .mock_file("defined/string/_other.scss", "$c: null;\n")
        .mock_file(
            "through_forward/as/_midstream.scss",
            "@forward \"upstream\" as b-*;\n",
        )
        .mock_file("through_forward/as/_upstream.scss", "$c: null;\n")
        .mock_file(
            "through_forward/bare/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file("through_forward/bare/_upstream.scss", "$c: null;\n")
        .mock_file(
            "through_forward/hide/_midstream.scss",
            "@forward \"upstream\" hide $b;\n",
        )
        .mock_file(
            "through_forward/hide/_upstream.scss",
            "$b: null;\n$c: null;\n",
        )
        .mock_file(
            "through_forward/show/_midstream.scss",
            "@forward \"upstream\" show $b;\n",
        )
        .mock_file(
            "through_forward/show/_upstream.scss",
            "$b: null;\n$c: null;\n",
        )
        .mock_file("through_use/other.scss", "$global-variable: null;\n")
}

#[test]
fn chosen_prefix() {
    let runner = runner().with_cwd("chosen_prefix");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \n@use \"other\" as a;\
             \nb {c: meta.global-variable-exists(\"d\", \"a\")}\n"),
        "b {\
         \n  c: true;\
         \n}\n"
    );
}
mod defined {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("defined")
    }

    #[test]
    #[ignore] // unexepected error
    fn module_value() {
        let runner = runner().with_cwd("module_value");
        assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@use \"other\";\
             \na {b: meta.global-variable-exists(\"c\", meta.get-module(\"other\"))}\n"
        ),
        "a {\
         \n  b: true;\
         \n}\n"
    );
    }
    #[test]
    fn string() {
        let runner = runner().with_cwd("string");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@use \"other\";\
             \na {b: meta.global-variable-exists(\"c\", \"other\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
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
             \n@use \"midstream\" as *;\
             \na {\
             \n  with-prefix: meta.global-variable-exists(b-c);\
             \n  without-prefix: meta.global-variable-exists(c);\
             \n}\n"),
            "a {\
         \n  with-prefix: true;\
         \n  without-prefix: false;\
         \n}\n"
        );
    }
    #[test]
    fn bare() {
        let runner = runner().with_cwd("bare");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@use \"midstream\" as *;\
             \na {b: meta.variable-exists(c)}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
    #[test]
    fn hide() {
        let runner = runner().with_cwd("hide");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@use \"midstream\" as *;\
             \na {\
             \n  hidden: meta.global-variable-exists(b);\
             \n  not-hidden: meta.global-variable-exists(c);\
             \n}\n"),
            "a {\
         \n  hidden: false;\
         \n  not-hidden: true;\
         \n}\n"
        );
    }
    #[test]
    fn show() {
        let runner = runner().with_cwd("show");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@use \"midstream\" as *;\
             \na {\
             \n  shown: meta.global-variable-exists(b);\
             \n  not-shown: meta.global-variable-exists(c);\
             \n}\n"),
            "a {\
         \n  shown: true;\
         \n  not-shown: false;\
         \n}\n"
        );
    }
}
#[test]
fn through_use() {
    let runner = runner().with_cwd("through_use");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \n@use \"other\" as *;\
             \na {b: meta.global-variable-exists(global-variable)}\n"),
        "a {\
         \n  b: true;\
         \n}\n"
    );
}
mod undefined {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("undefined")
    }

    #[test]
    #[ignore] // unexepected error
    fn module_value() {
        let runner = runner().with_cwd("module_value");
        assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@use \"sass:color\";\
             \na {b: meta.global-variable-exists(\"c\", meta.get-module(\"color\"))}\n"
        ),
        "a {\
         \n  b: false;\
         \n}\n"
    );
    }
    #[test]
    fn string() {
        let runner = runner().with_cwd("string");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@use \"sass:color\";\
             \na {b: meta.global-variable-exists(\"c\", \"color\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
}
