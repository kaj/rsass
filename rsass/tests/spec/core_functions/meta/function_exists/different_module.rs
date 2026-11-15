//! Tests auto-converted from "sass-spec/spec/core_functions/meta/function_exists/different_module.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("different_module")
        .mock_file(
            "through_forward/as/_midstream.scss",
            "@forward \"upstream\" as b-*;\n",
        )
        .mock_file(
            "through_forward/as/_upstream.scss",
            "@function c() {@return null}\n",
        )
        .mock_file(
            "through_forward/bare/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "through_forward/bare/_upstream.scss",
            "@function c() {@return null}\n",
        )
        .mock_file(
            "through_forward/hide/_midstream.scss",
            "@forward \"upstream\" hide b;\n",
        )
        .mock_file(
            "through_forward/hide/_upstream.scss",
            "@function b() {@return null}\n@function c() {@return null}\n",
        )
        .mock_file(
            "through_forward/show/_midstream.scss",
            "@forward \"upstream\" show b;\n",
        )
        .mock_file(
            "through_forward/show/_upstream.scss",
            "@function b() {@return null}\n@function c() {@return null}\n",
        )
        .mock_file(
            "through_use/other.scss",
            "@function global-function() {@return null}\n",
        )
}

#[test]
fn chosen_prefix() {
    let runner = runner().with_cwd("chosen_prefix");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \n@use \"sass:color\" as a;\
             \nb {c: meta.function-exists(\"red\", \"a\")}\n"),
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
             \n@use \"sass:color\";\
             \na {b: meta.function-exists(\"red\", meta.get-module(\"color\"))}\n"
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
             \n@use \"sass:color\";\
             \na {b: meta.function-exists(\"red\", \"color\")}\n"),
            "a {\
         \n  b: true;\
         \n}\n"
        );
    }
}
mod global {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("global")
    }

    #[test]
    fn string() {
        let runner = runner().with_cwd("string");
        assert_eq!(
            runner.ok("// Regression test\
             \n@use \"sass:meta\";\
             \n@use \"sass:list\";\
             \na {b: meta.function-exists(\"rgb\", \"list\")}\n"),
            "a {\
         \n  b: false;\
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
             \n  with-prefix: meta.function-exists(b-c);\
             \n  without-prefix: meta.function-exists(c);\
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
             \na {b: meta.function-exists(c)}\n"),
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
             \n  hidden: meta.function-exists(b);\
             \n  not-hidden: meta.function-exists(c);\
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
             \n  shown: meta.function-exists(b);\
             \n  not-shown: meta.function-exists(c);\
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
             \na {b: meta.function-exists(global-function)}\n"),
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
             \na {b: meta.function-exists(\"c\", meta.get-module(\"color\"))}\n"
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
             \na {b: meta.function-exists(\"c\", \"color\")}\n"),
            "a {\
         \n  b: false;\
         \n}\n"
        );
    }
}
