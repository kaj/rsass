//! Tests auto-converted from "sass-spec/spec/core_functions/get-function"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

/// From "sass-spec/spec/core_functions/get-function/basic.hrx"
#[test]
fn basic() {
    assert_eq!(
        rsass(
            "@function add-two($v) {\n  @return $v + 2;\n}\n\n$add-two-fn: get-function(add-two);\n\n.result {\n  inspect: inspect($add-two-fn);\n  type-of: type-of($add-two-fn);\n  result: call($add-two-fn, 10);\n}\n"
        )
        .unwrap(),
        ".result {\n  inspect: get-function(\"add-two\");\n  type-of: function;\n  result: 12;\n}\n"
    );
}

/// From "sass-spec/spec/core_functions/get-function/builtins.hrx"
#[test]
fn builtins() {
    assert_eq!(
        rsass(
            "$built-in: get-function(lighten);\n\n.result {\n  inspect: inspect($built-in);\n  type-of: type-of($built-in);\n  result: call($built-in, red, 30%);\n}\n"
        )
        .unwrap(),
        ".result {\n  inspect: get-function(\"lighten\");\n  type-of: function;\n  result: #ff9999;\n}\n"
    );
}

/// From "sass-spec/spec/core_functions/get-function/css-fn-reference.hrx"
#[test]
fn css_fn_reference() {
    assert_eq!(
        rsass(
            "@function supports($something) {\n  @return sass;\n}\n\n$sass-fn: get-function(supports);\n$css-fn: get-function(supports, $css: true);\n\n.result {\n  sass-fn: call($sass-fn, 1px);\n  css-fn: call($css-fn, inspect((all: inherit)));\n}\n\n"
        )
        .unwrap(),
        ".result {\n  sass-fn: sass;\n  css-fn: supports((all: inherit));\n}\n"
    );
}

/// From "sass-spec/spec/core_functions/get-function/equality.hrx"
#[test]
#[ignore] // failing
fn equality() {
    assert_eq!(
        rsass(
            "$builtin: get-function(rgb);\n$same-builtin: get-function(rgb);\n$different-builtin: get-function(lighten);\n@function lighten() {\n  @return wut;\n}\n$redefined-builtin: get-function(lighten);\n.should-be-true {\n  identical: $builtin == $builtin;\n  same: $builtin == $same-builtin;\n}\n\n.should-be-false {\n  different: $different-builtin == $builtin;\n  redefined: $different-builtin == $redefined-builtin;\n}\n\n"
        )
        .unwrap(),
        ".should-be-true {\n  identical: true;\n  same: true;\n}\n.should-be-false {\n  different: false;\n  redefined: false;\n}\n"
    );
}

mod errors;

/// From "sass-spec/spec/core_functions/get-function/local-scope.hrx"
#[test]
fn local_scope() {
    assert_eq!(
        rsass(
            "@function foo() {@return global}\n\n.first-scope {\n  @function foo() {@return local}\n  a: call(foo);\n}\n\n.second-scope {\n  a: call(foo);\n}\n\n"
        )
        .unwrap(),
        ".first-scope {\n  a: local;\n}\n.second-scope {\n  a: global;\n}\n"
    );
}

/// From "sass-spec/spec/core_functions/get-function/overrides.hrx"
#[test]
fn overrides() {
    assert_eq!(
        rsass(
            "@function add-two($v) {\n  @return $v + 2;\n}\n\n$add-two-fn: get-function(add-two);\n\n@function add-two($v) {\n  @error \"This should not have been called.\";\n}\n\n$lighten-fn: get-function(lighten) !default;\n\n@function lighten($color, $amount: null) {\n  $amount: 10% !default;\n  @return call($lighten-fn, $color, $amount);\n}\n\n.result {\n  captured-sass-fn: call($add-two-fn, 10);\n  captured-ruby-fn: lighten(red);\n}\n"
        )
        .unwrap(),
        ".result {\n  captured-sass-fn: 12;\n  captured-ruby-fn: #ff3333;\n}\n"
    );
}
