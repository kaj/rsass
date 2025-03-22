//! Tests auto-converted from "sass-spec/spec/core_functions/meta/get_mixin/same_module.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("same_module").mock_file(
        "through_import/other.scss",
        "@mixin add-two($v) {b: $v + 2}\n",
    )
}

mod dash_insensitive {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("dash_insensitive")
    }

    #[test]
    #[ignore] // unexepected error
    fn dash_to_underscore() {
        let runner = runner().with_cwd("dash_to_underscore");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@mixin add_two($v) {b: $v + 2}\n\
             \na {@include meta.apply(meta.get-mixin(add-two), 10)}\n"),
            "a {\
         \n  b: 12;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn underscore_to_dash() {
        let runner = runner().with_cwd("underscore_to_dash");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@mixin add-two($v) {b: $v + 2}\n\
             \na {@include meta.apply(meta.get-mixin(add_two), 10)}\n"),
            "a {\
         \n  b: 12;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn redefined() {
    let runner = runner().with_cwd("redefined");
    assert_eq!(
        runner.ok(
            "@use \"sass:meta\";\
             \n@mixin add-two($v) {b: $v + 2}\
             \n$add-two-mixin: meta.get-mixin(add-two);\n\
             \n// The mixin returned by `meta.get-mixin()` is locked in place when it\'s\
             \n// called. Redefining the mixin after the fact shouldn\'t affect the stored\
             \n// value.\
             \n@mixin add-two($v) {@error \"Should not be called\"}\n\
             \na {@include meta.apply($add-two-mixin, 10)}\n"
        ),
        "a {\
         \n  b: 12;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn through_import() {
    let runner = runner().with_cwd("through_import");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \n@import \"other\";\
             \na {@include meta.apply(meta.get-mixin(add-two), 10)}\n"),
        "a {\
         \n  b: 12;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn user_defined() {
    let runner = runner().with_cwd("user_defined");
    assert_eq!(
        runner.ok("@use \"sass:meta\";\
             \n@mixin add-two($v) {b: $v + 2}\
             \n$add-two-mixin: meta.get-mixin(add-two);\n\
             \na {@include meta.apply($add-two-mixin, 10)}\n"),
        "a {\
         \n  b: 12;\
         \n}\n"
    );
}
