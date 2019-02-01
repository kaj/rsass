//! Tests auto-converted from "sass-spec/spec/libsass"
//! version 5717844f, 2019-01-28 20:42:33 -0500.
//! See <https://github.com/sass/sass-spec> for source material.\n
//! The following tests are excluded from conversion:
//! ["Sa\u{301}ss-UT\u{327}F8", "at-error/feature-test", "at-root/ampersand", "at-root/extend", "at-root/138_test_at_root_in_media", "at-root/139_test_at_root_in_bubbled_media", "at-root/140_test_at_root_in_unknown_directive", "at-root/with_without", "at-stuff", "base-level-parent/imported", "base-level-parent/nested/at-root-alone-itpl", "base-level-parent/nested/at-root-postfix-itpl", "base-level-parent/nested/at-root-prefix-itpl", "base-level-parent/root/at-root-postfix-itpl", "base-level-parent/root/at-root-prefix-itpl", "bool", "bourbon", "calc", "charset", "color-functions/opacity/fade-out", "color-functions/opacity/transparentize", "color-functions/other/change-color/a", "color-functions/rgb/rgba/a", "color-functions/saturate", "conversions", "css-import", "css_nth_selectors", "css_unicode", "debug-directive-nested/function", "debug-directive-nested/mixin", "delayed", "div", "env", "features/at-error", "features/extend-selector-pseudoclass", "http_import", "import", "inh", "inheritance", "interpolated-function-call", "interpolated-urls", "list-evaluation", "lists", "media", "mixin", "mixins-and-media-queries", "multi-blocks", "placeholder-mediaquery", "placeholder-nested", "precision/default", "precision/lower", "properties-in-media", "propsets", "rel", "selector-functions/is_superselector", "selector-functions/selector-length", "selector-functions/simple-selector", "selectors/access", "selectors/interpolation", "selectors/mixin-argument", "selectors/simple", "selectors/variables/multiple/bare", "selectors/variables/multiple/interpolated", "selectors/variables/nested/bare", "selectors/variables/nested/interpolated", "test", "unary-ops", "unicode-bom/utf-16-big", "unicode-bom/utf-16-little", "unicode-bom/utf-8", "units/conversion/angle", "units/conversion/frequency", "units/conversion/resolution", "units/conversion/size", "units/conversion/time", "units/simple", "url", "variable-scoping/blead-global/expanding/at-root", "variable-scoping/blead-global/expanding/each", "variable-scoping/blead-global/expanding/else", "variable-scoping/blead-global/expanding/elseif", "variable-scoping/blead-global/expanding/for", "variable-scoping/blead-global/expanding/if", "variable-scoping/blead-global/expanding/while", "variable-scoping/blead-global/functional/each", "variable-scoping/blead-global/functional/else", "variable-scoping/blead-global/functional/elseif", "variable-scoping/blead-global/functional/for", "variable-scoping/blead-global/functional/if", "variable-scoping/defaults", "variable-scoping/lexical-scope", "variable-scoping/root-scope", "variables_in_media", "warn-directive-nested/function", "warn-directive-nested/mixin"]
extern crate rsass;
use rsass::{compile_scss, OutputStyle};

// Ignoring "Sa\u{301}ss-UT\u{327}F8", not expected to work yet.

/// From "sass-spec/spec/libsass/Sáss-UŢF8"
#[test]
fn sass_utf8() {
    assert_eq!(
        rsass("span.utf8-in-path {\n  margin: auto;\n}\n").unwrap(),
        "span.utf8-in-path {\n  margin: auto;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/arg-eval"
#[test]
fn arg_eval() {
    assert_eq!(
        rsass(
            "@function foo() {\n  @return 1+2 3/4 5+6;\n}\n\n@mixin bar($x: 3/4) {\n  bar-content: $x;\n}\n\ndiv {\n  content: foobar(1+2 3/4 5+6, orange);\n  content: append(1+2 2/3 5+6, orange);\n  content: 1+2 2/3 5+6;\n  content: type-of(2/3);\n  content: type-of(orange);\n  content: foo();\n  @include bar();\n}"
        )
        .unwrap(),
        "div {\n  content: foobar(3 3/4 11, orange);\n  content: 3 2/3 11 orange;\n  content: 3 2/3 11;\n  content: number;\n  content: color;\n  content: 3 3/4 11;\n  bar-content: 0.75;\n}\n"
    );
}

// Ignoring "arithmetic", end_version is 3.5.

mod at_error;

mod at_root;

// Ignoring "at-stuff", not expected to work yet.

mod base_level_parent;

mod basic;

// Ignoring "bool", not expected to work yet.

// Ignoring "bourbon", not expected to work yet.

// Ignoring "calc", not expected to work yet.

// Ignoring "charset", not expected to work yet.

mod color_functions;

// Ignoring "color-names", end_version is 3.5.

// Ignoring "conversions", not expected to work yet.

// Ignoring "css-import", not expected to work yet.

// Ignoring "css_nth_selectors", not expected to work yet.

// Ignoring "css_unicode", not expected to work yet.

mod debug_directive_nested;

// Ignoring "delayed", not expected to work yet.

// Ignoring "div", not expected to work yet.

// Ignoring "env", not expected to work yet.

/// From "sass-spec/spec/libsass/eq"
#[test]
fn eq() {
    assert_eq!(
        rsass(
            "div {\n  foo: center == \"center\";\n  foo: (a b c) == (a b c);\n  foo: a b c == a b c;\n}\n"
        )
        .unwrap(),
        "div {\n  foo: true;\n  foo: true;\n  foo: a b false b c;\n}\n"
    );
}

mod error_directive_nested;

mod features;

/// From "sass-spec/spec/libsass/filter-functions"
#[test]
fn filter_functions() {
    assert_eq!(
        rsass(
            "div {\n  hoo: grayscale(0.3) grayscale(200%);\n  moo: opacity(0.3) opacity(200%);\n  poo: invert(0.3) invert(200%);\n  goo: saturate(0.3) saturate(200%);\n}\n"
        )
        .unwrap(),
        "div {\n  hoo: grayscale(0.3) grayscale(200%);\n  moo: opacity(0.3) opacity(200%);\n  poo: invert(0.3) invert(200%);\n  goo: saturate(0.3) saturate(200%);\n}\n"
    );
}

// Ignoring "http_import", not expected to work yet.

/// From "sass-spec/spec/libsass/image-url"
#[test]
fn image_url() {
    assert_eq!(
        rsass(
            "div {\n  blah: image-url(\"hello.png\", false);\n  blah: image-url(\"hello.png\", true);\n}"
        )
        .unwrap(),
        "div {\n  blah: image-url(\"hello.png\", false);\n  blah: image-url(\"hello.png\", true);\n}\n"
    );
}

// Ignoring "import", not expected to work yet.

// Ignoring "inh", not expected to work yet.

// Ignoring "inheritance", not expected to work yet.

// Ignoring "interpolated-function-call", not expected to work yet.

// Ignoring "interpolated-function-call-4.0", start_version is 4.0.

// Ignoring "interpolated-urls", not expected to work yet.

// Ignoring "interpolated-urls-4.0", start_version is 4.0.

/// From "sass-spec/spec/libsass/keyframes"
#[test]
fn keyframes() {
    assert_eq!(
        rsass(
            "div {\n  color: #181818;\n}\n\n@-webkit-keyframes uiDelayedFadeIn {\n\t0% { opacity: 0; }\n\t50% { opacity: .5; }\n\t100% { opacity: 1; }\n}\n\n@-webkit-keyframes bounce {\n\tfrom {\n\t\tleft: 0px;\n\t}\n\tto {\n\t\tleft: 200px;\n\t}\n}\n\n$name: bounce;\n\n@-webkit-keyframes #{$name} {\n  blah: blee;\n}\n\n@mixin fudge() {\n  @content;\n}\n\nfoo {\n  @include fudge() {\n    div {\n      color: red;\n    }\n  }\n}\n"
        )
        .unwrap(),
        "div {\n  color: #181818;\n}\n\n@-webkit-keyframes uiDelayedFadeIn {\n  0% {\n    opacity: 0;\n  }\n  50% {\n    opacity: .5;\n  }\n  100% {\n    opacity: 1;\n  }\n}\n@-webkit-keyframes bounce {\n  from {\n    left: 0px;\n  }\n  to {\n    left: 200px;\n  }\n}\n@-webkit-keyframes bounce {\n  blah: blee;\n}\nfoo div {\n  color: red;\n}\n"
    );
}

/// From "sass-spec/spec/libsass/length"
#[test]
fn length() {
    assert_eq!(
        rsass(
            "div {\n\n  foo: length(null);\n  foo: length(true);\n  foo: length(false);\n\n  foo: length(\"protégé\");\n  foo: length(protégé);\n  foo: length(\"\");\n  foo: length(\"hello there\");\n  foo: length(\"Façade\");\n  foo: length(\"Tromsø\");\n  foo: length(\"Ãlso\");\n\n  foo: length((foo: foo, bar: bar));\n  foo: length((foo, bar, baz, bang));\n\n}\n"
        )
        .unwrap(),
        "div {\n  foo: 1;\n  foo: 1;\n  foo: 1;\n  foo: 1;\n  foo: 1;\n  foo: 1;\n  foo: 1;\n  foo: 1;\n  foo: 1;\n  foo: 1;\n  foo: 2;\n  foo: 4;\n}\n"
    );
}

// Ignoring "list-evaluation", not expected to work yet.

// Ignoring "lists", not expected to work yet.

// Ignoring "media", not expected to work yet.

/// From "sass-spec/spec/libsass/media-hoisting"
#[test]
fn media_hoisting() {
    assert_eq!(
        rsass(
            "@media screen {\n  a {\n    color: black;\n    height: 8px;\n  }\n}\n\na {\n  color: red;\n  @media screen {\n    color: blue;\n    height: 10px;\n  }\n}\n\na {\n  color: beige;\n  b {\n    color: teal;\n    @media screen {\n      color: orange;\n      c {\n        height: 12px;\n      }\n    }\n  }\n}\n"
        )
        .unwrap(),
        "@media screen {\n  a {\n    color: black;\n    height: 8px;\n  }\n}\na {\n  color: red;\n}\n@media screen {\n  a {\n    color: blue;\n    height: 10px;\n  }\n}\n\na {\n  color: beige;\n}\na b {\n  color: teal;\n}\n@media screen {\n  a b {\n    color: orange;\n  }\n  a b c {\n    height: 12px;\n  }\n}\n"
    );
}

// Ignoring "mixin", not expected to work yet.

// Ignoring "mixins-and-media-queries", not expected to work yet.

// Ignoring "multi-blocks", not expected to work yet.

mod parent_selector;

// Ignoring "placeholder-mediaquery", not expected to work yet.

// Ignoring "placeholder-nested", not expected to work yet.

mod precision;

// Ignoring "properties-in-media", not expected to work yet.

// Ignoring "propsets", not expected to work yet.

// Ignoring "rel", not expected to work yet.

/// From "sass-spec/spec/libsass/scale"
#[test]
fn scale() {
    assert_eq!(
        rsass(
            "div {\n  color: scale-color(red, $red: -23%);\n  color: scale-color(hsl(120, 70, 80), $lightness: 50%);\n  color: scale-color(rgb(200, 150, 170), $green: -40%, $blue: 70%);\n  color: scale-color(hsl(200, 70, 80), $saturation: -90%, $alpha: 10%);\n  blah: #d4f7d4;\n}"
        )
        .unwrap(),
        "div {\n  color: #c40000;\n  color: #d4f7d4;\n  color: #c85ae6;\n  color: #c8cdd0;\n  blah: #d4f7d4;\n}\n"
    );
}

mod selector_functions;

/// From "sass-spec/spec/libsass/selector_interpolation_in_string"
#[test]
fn selector_interpolation_in_string() {
    assert_eq!(
        rsass("foo[val=\"bar #{\"foo\" + \" bar\"} baz\"] {a: b}\n").unwrap(),
        "foo[val=\"bar foo bar baz\"] {\n  a: b;\n}\n"
    );
}

mod selectors;

// Ignoring "test", not expected to work yet.

// Ignoring "unary-ops", not expected to work yet.

mod unicode_bom;

/// From "sass-spec/spec/libsass/unitless"
#[test]
fn unitless() {
    assert_eq!(
        rsass(
            "div {\n  hoo: unitless(42);\n  hee: unitless(42px);\n  foo: unitless(3.14in);\n}"
        )
        .unwrap(),
        "div {\n  hoo: true;\n  hee: false;\n  foo: false;\n}\n"
    );
}

mod units;

/// From "sass-spec/spec/libsass/unquote"
#[test]
fn unquote() {
    assert_eq!(
        rsass(
            "div {\n  a: unquote(\"foo\");\n  b: unquote(\"I\'m a \\\"fashion\\\" \\\"expert\\\".\");\n  c: unquote(\\\"wha);\n  d: unquote(\"column1\\tcolumn2\");\n  e: unquote(23+1);\n}\n"
        )
        .unwrap(),
        "div {\n  a: foo;\n  b: I\'m a \"fashion\" \"expert\".;\n  c: \\\"wha;\n  d: column1tcolumn2;\n  e: 24;\n}\n"
    );
}

// Ignoring "url", not expected to work yet.

mod variable_scoping;

// Ignoring "variables_in_media", not expected to work yet.

mod warn_directive_nested;

fn rsass(input: &str) -> Result<String, String> {
    compile_scss(input.as_bytes(), OutputStyle::Expanded)
        .map_err(|e| format!("rsass failed: {}", e))
        .and_then(|s| String::from_utf8(s).map_err(|e| format!("{:?}", e)))
}
