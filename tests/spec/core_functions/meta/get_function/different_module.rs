//! Tests auto-converted from "sass-spec/spec/core_functions/meta/get_function/different_module.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
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
    assert_eq!(
        runner().ok(
            "@use \"sass:color\" as a;\
             \nb {c: call(get-function(\"red\", $module: \"a\"), #abcdef)}\n"
        ),
        "b {\
         \n  c: 171;\
         \n}\n"
    );
}
#[test]
fn defined() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: call(get-function(\"red\", $module: \"color\"), #abcdef)}\n"
        ),
        "a {\
         \n  b: 171;\
         \n}\n"
    );
}
#[test]
fn named() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: call(get-function($name: \"red\", $module: \"color\"), #abcdef)}\n"
        ),
        "a {\
         \n  b: 171;\
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
            runner().ok("@use \"midstream\" as *;\
             \na {\
             \n  b: call(get-function(c-d));\
             \n}\n"),
            "a {\
         \n  b: d;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn bare() {
        assert_eq!(
            runner().ok("@use \"midstream\" as *;\
             \na {b: call(get-function(c))}\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn hide() {
        assert_eq!(
            runner().ok("@use \"midstream\" as *;\
             \na {\
             \n  b: call(get-function(d));\
             \n}\n"),
            "a {\
         \n  b: d;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn show() {
        assert_eq!(
            runner().ok("@use \"midstream\" as *;\
             \na {\
             \n  b: call(get-function(c));\
             \n}\n"),
            "a {\
         \n  b: c;\
         \n}\n"
        );
    }
}
#[test]
#[ignore] // unexepected error
fn through_use() {
    assert_eq!(
        runner().ok("@use \"other\" as *;\
             \na {b: call(get-function(add-two), 10)}\n"),
        "a {\
         \n  b: 12;\
         \n}\n"
    );
}
