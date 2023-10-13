//! Tests auto-converted from "sass-spec/spec/core_functions/meta/get_mixin/different_module.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("different_module")
        .mock_file(
            "chosen_prefix/_other.scss",
            "@mixin a($color) {c: red($color)}\n",
        )
        .mock_file(
            "defined/_other.scss",
            "@mixin a($color) {b: red($color)}\n",
        )
        .mock_file("named/_other.scss", "@mixin a($color) {b: red($color)}\n")
        .mock_file(
            "through_forward/as/_midstream.scss",
            "@forward \"upstream\" as c-*;\n",
        )
        .mock_file("through_forward/as/_upstream.scss", "@mixin d() {b: d}\n")
        .mock_file(
            "through_forward/bare/_midstream.scss",
            "@forward \"upstream\";\n",
        )
        .mock_file(
            "through_forward/bare/_upstream.scss",
            "@mixin c() {b: c}\n",
        )
        .mock_file(
            "through_forward/hide/_midstream.scss",
            "@forward \"upstream\" hide c;\n",
        )
        .mock_file(
            "through_forward/hide/_upstream.scss",
            "@mixin d() {b: d}\n",
        )
        .mock_file(
            "through_forward/show/_midstream.scss",
            "@forward \"upstream\" show c;\n",
        )
        .mock_file(
            "through_forward/show/_upstream.scss",
            "@mixin c() {b: c}\n",
        )
        .mock_file(
            "through_use/other.scss",
            "@mixin add-two($v) {b: $v + 2}\n",
        )
}

#[test]
#[ignore] // unexepected error
fn chosen_prefix() {
    let runner = runner().with_cwd("chosen_prefix");
    assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@use \"other\" as a;\
             \nb {@include meta.apply(meta.get-mixin(\"a\", $module: \"a\"), #abcdef)}\n"
        ),
        "b {\
         \n  c: 171;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn defined() {
    let runner = runner().with_cwd("defined");
    assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@use \"other\";\
             \na {@include meta.apply(meta.get-mixin(\"a\", $module: \"other\"), #abcdef)}\n"
        ),
        "a {\
         \n  b: 171;\
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
             \n@use \"other\";\
             \na {@include meta.apply(meta.get-mixin($name: \"a\", $module: \"other\"), #abcdef)}\n"
        ),
        "a {\
         \n  b: 171;\
         \n}\n"
    );
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
            runner.ok("@use \"sass:meta\";\
             \n@use \"midstream\" as *;\
             \na {@include meta.apply(meta.get-mixin(c-d))}\n"),
            "a {\
         \n  b: d;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn bare() {
        let runner = runner().with_cwd("bare");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@use \"midstream\" as *;\
             \na {@include meta.apply(meta.get-mixin(c))}\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hide() {
        let runner = runner().with_cwd("hide");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@use \"midstream\" as *;\
             \na {@include meta.apply(meta.get-mixin(d))}\n"),
            "a {\
         \n  b: d;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn show() {
        let runner = runner().with_cwd("show");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@use \"midstream\" as *;\
             \na {@include meta.apply(meta.get-mixin(c))}\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn through_use() {
    let runner = runner().with_cwd("through_use");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \n@use \"other\" as *;\
             \na {@include meta.apply(meta.get-mixin(add-two), 10)}\n"),
        "a {\
         \n  b: 12;\
         \n}\n"
    );
}
