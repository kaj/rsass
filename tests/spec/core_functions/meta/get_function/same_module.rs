//! Tests auto-converted from "sass-spec/spec/core_functions/meta/get_function/same_module.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().mock_file(
        "through_import/other.scss",
        "@function add-two($v) {@return $v + 2}\n",
    )
}

#[test]
fn built_in() {
    let runner = runner().with_cwd("built_in");
    assert_eq!(
        runner.ok("$lighten-fn: get-function(lighten);\n\
             \na {b: call($lighten-fn, red, 30%)}\n"),
        "a {\
         \n  b: #ff9999;\
         \n}\n"
    );
}
mod dash_insensitive {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("dash_insensitive")
    }

    #[test]
    fn dash_to_underscore() {
        let runner = runner().with_cwd("dash_to_underscore");
        assert_eq!(
            runner.ok("@function add_two($v) {@return $v + 2}\n\
             \na {b: call(get-function(add-two), 10)}\n"),
            "a {\
         \n  b: 12;\
         \n}\n"
        );
    }
    #[test]
    fn underscore_to_dash() {
        let runner = runner().with_cwd("underscore_to_dash");
        assert_eq!(
            runner.ok("@function add-two($v) {@return $v + 2}\n\
             \na {b: call(get-function(add_two), 10)}\n"),
            "a {\
         \n  b: 12;\
         \n}\n"
        );
    }
}
#[test]
fn plain_css() {
    let runner = runner().with_cwd("plain_css");
    assert_eq!(
        runner.ok("$sass-fn: get-function(lighten);\
             \n$css-fn: get-function(lighten, $css: true);\n\
             \na {\
             \n  sass-fn: call($sass-fn, red, 30%);\
             \n  css-fn: call($css-fn, red, 30%);\
             \n}\n"),
        "a {\
         \n  sass-fn: #ff9999;\
         \n  css-fn: lighten(red, 30%);\
         \n}\n"
    );
}
#[test]
fn redefined() {
    let runner = runner().with_cwd("redefined");
    assert_eq!(
        runner.ok(
            "@function add-two($v) {@return $v + 2}\
             \n$add-two-fn: get-function(add-two);\n\
             \n// The function returned by `get-function()` is locked in place when it\'s\
             \n// called. Redefining the function after the fact shouldn\'t affect the stored\
             \n// value.\
             \n@function add-two($v) {@error \"Should not be called\"}\n\
             \na {b: call($add-two-fn, 10)}\n"
        ),
        "a {\
         \n  b: 12;\
         \n}\n"
    );
}
#[test]
fn through_import() {
    let runner = runner().with_cwd("through_import");
    assert_eq!(
        runner.ok("@import \"other\";\
             \na {b: call(get-function(add-two), 10)}\n"),
        "a {\
         \n  b: 12;\
         \n}\n"
    );
}
#[test]
fn user_defined() {
    let runner = runner().with_cwd("user_defined");
    assert_eq!(
        runner.ok("@function add-two($v) {@return $v + 2}\
             \n$add-two-fn: get-function(add-two);\n\
             \na {b: call($add-two-fn, 10)}\n"),
        "a {\
         \n  b: 12;\
         \n}\n"
    );
}
