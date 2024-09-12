//! Tests auto-converted from "sass-spec/spec/core_functions/meta/get_function/different_module.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("different_module")
        .mock_file(
            "through_forward/as/_midstream.scss",
            "@forward \"upstream\" as c-*;\n",
        )
        .mock_file(
            "through_forward/as/_upstream.scss",
            "@function d() {@return d}\n",
        )
        .mock_file(
            "through_forward/bare/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "through_forward/bare/_upstream.scss",
            "@function c() {@return c}\n",
        )
        .mock_file(
            "through_forward/hide/_midstream.scss",
            "@forward \"upstream\" hide c;\n",
        )
        .mock_file(
            "through_forward/hide/_upstream.scss",
            "@function d() {@return d}\n",
        )
        .mock_file(
            "through_forward/show/_midstream.scss",
            "@forward \"upstream\" show c;\n",
        )
        .mock_file(
            "through_forward/show/_upstream.scss",
            "@function c() {@return c}\n",
        )
        .mock_file(
            "through_use/other.scss",
            "@function add-two($v) {@return $v + 2}\n",
        )
}

#[test]
fn chosen_prefix() {
    let runner = runner().with_cwd("chosen_prefix");
    assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@use \"sass:math\" as a;\
             \nb {c: meta.call(meta.get-function(\"round\", $module: \"a\"), 0.6)}\n"
        ),
        "b {\
         \n  c: 1;\
         \n}\n"
    );
}
#[test]
fn defined() {
    let runner = runner().with_cwd("defined");
    assert_eq!(
        runner.ok(
            "@use \"sass:math\";\
             \n@use \"sass:meta\";\
             \na {b: meta.call(meta.get-function(\"round\", $module: \"math\"), 0.6)}\n"
        ),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
#[test]
fn named() {
    let runner = runner().with_cwd("named");
    assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@use \"sass:math\";\
             \na {b: meta.call(meta.get-function($name: \"round\", $module: \"math\"), 0.6)}\n"
        ),
        "a {\
         \n  b: 1;\
         \n}\n"
    );
}
mod through_forward {
    #[allow(unused)]
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
             \n  b: meta.call(meta.get-function(c-d));\
             \n}\n"),
            "a {\
         \n  b: d;\
         \n}\n"
        );
    }
    #[test]
    fn bare() {
        let runner = runner().with_cwd("bare");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@use \"midstream\" as *;\
             \na {b: meta.call(meta.get-function(c))}\n"),
            "a {\
         \n  b: c;\
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
             \n  b: meta.call(meta.get-function(d));\
             \n}\n"),
            "a {\
         \n  b: d;\
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
             \n  b: meta.call(meta.get-function(c));\
             \n}\n"),
            "a {\
         \n  b: c;\
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
             \na {b: meta.call(meta.get-function(add-two), 10)}\n"),
        "a {\
         \n  b: 12;\
         \n}\n"
    );
}
