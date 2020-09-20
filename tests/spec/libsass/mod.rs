//! Tests auto-converted from "sass-spec/spec/libsass"
#[allow(unused)]
use super::rsass;

// Ignoring "Sa\u{301}ss-UT\u{327}F8.hrx", not expected to work yet.

// From "sass-spec/spec/libsass/Sáss-UŢF8.hrx"
#[test]
fn sass_utf8() {
    assert_eq!(
        rsass(
            "span.utf8-in-path {\
            \n  margin: auto;\
            \n}\
            \n"
        )
        .unwrap(),
        "span.utf8-in-path {\
        \n  margin: auto;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/arg-eval.hrx"
#[test]
fn arg_eval() {
    assert_eq!(
        rsass(
            "@function foo() {\
            \n  @return 1+2 3/4 5+6;\
            \n}\
            \n\
            \n@mixin bar($x: 3/4) {\
            \n  bar-content: $x;\
            \n}\
            \n\
            \ndiv {\
            \n  content: foobar(1+2 3/4 5+6, orange);\
            \n  content: append(1+2 2/3 5+6, orange);\
            \n  content: 1+2 2/3 5+6;\
            \n  content: type-of(2/3);\
            \n  content: type-of(orange);\
            \n  content: foo();\
            \n  @include bar();\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  content: foobar(3 3/4 11, orange);\
        \n  content: 3 2/3 11 orange;\
        \n  content: 3 2/3 11;\
        \n  content: number;\
        \n  content: color;\
        \n  content: 3 3/4 11;\
        \n  bar-content: 0.75;\
        \n}\
        \n"
    );
}

mod at_error;

mod at_root;

// From "sass-spec/spec/libsass/at-stuff.hrx"
#[test]
#[ignore] // wrong result
fn at_stuff() {
    assert_eq!(
        rsass(
            "@fudge hux bloo;\
            \n\
            \ndiv {\
            \n\tcolor: red;\
            \n\t@fudge {\
            \n\t\tspan {\
            \n\t\t\twidth: 10px;\
            \n\t\t\ta {\
            \n\t\t\t\tfont: whatever;\
            \n\t\t\t}\
            \n\t\t}\
            \n\t}\
            \n\theight: 20px;\
            \n\t@-webkit-keyframes SOMETHING {\
            \n\t\t0%   { opacity: 0; }\
            \n\t\t50%  { opacity: 0.5; }\
            \n\t\t100% { opacity: 1.0; }\
            \n\t}\
            \n\t@-webkit-keyframes BOUNCE {\
            \n\t\tfrom {\
            \n\t\t\tleft: 0px;\
            \n\t\t}\
            \n\t\tto {\
            \n\t\t\tleft: 200px;\
            \n\t\t}\
            \n\t}\
            \n}\
            \n\
            \ndiv {\
            \n\tspan {\
            \n\t\tfont: whatever;\
            \n\t}\
            \n\tborder: {\
            \n\t\tupper: {\
            \n\t\t\tleft: 10px;\
            \n\t\t\tright: 9px;\
            \n\t\t}\
            \n\t\tlower: {\
            \n\t\t\tleft: 8px;\
            \n\t\t\tright: 7px;\
            \n\t\t}\
            \n\t}\
            \n\tbackground: gray;\
            \n}\
            \n\
            \n@fudge HEY, HOO, HA:first-child {\
            \n\tcolor: blue;\
            \n}\
            \n\
            \n@mudge div span, a:visited;\
            \n\
            \n@fu#{dge} foo {\
            \n\tcolor: red;\
            \n}\
            \n"
        )
        .unwrap(),
        "@fudge hux bloo;\
        \ndiv {\
        \n  color: red;\
        \n  height: 20px;\
        \n}\
        \n@fudge {\
        \n  div span {\
        \n    width: 10px;\
        \n  }\
        \n  div span a {\
        \n    font: whatever;\
        \n  }\
        \n}\
        \n@-webkit-keyframes SOMETHING {\
        \n  0% {\
        \n    opacity: 0;\
        \n  }\
        \n  50% {\
        \n    opacity: 0.5;\
        \n  }\
        \n  100% {\
        \n    opacity: 1;\
        \n  }\
        \n}\
        \n@-webkit-keyframes BOUNCE {\
        \n  from {\
        \n    left: 0px;\
        \n  }\
        \n  to {\
        \n    left: 200px;\
        \n  }\
        \n}\
        \ndiv {\
        \n  border-upper-left: 10px;\
        \n  border-upper-right: 9px;\
        \n  border-lower-left: 8px;\
        \n  border-lower-right: 7px;\
        \n  background: gray;\
        \n}\
        \ndiv span {\
        \n  font: whatever;\
        \n}\
        \n@fudge HEY, HOO, HA:first-child {\
        \n  color: blue;\
        \n}\
        \n@mudge div span, a:visited;\
        \n@fudge foo {\
        \n  color: red;\
        \n}\
        \n"
    );
}

mod base_level_parent;

mod basic;

// From "sass-spec/spec/libsass/bool.hrx"
#[test]
#[ignore] // wrong result
fn bool() {
    assert_eq!(
        rsass(
            "div {\
            \n  a: (false and \"hey\");\
            \n  b: (\"hey\" and \"ho\");\
            \n  b: (\"hey\" or \"ho\");\
            \n  a: false and \"hey\";\
            \n  b: \"hey\" and \"ho\";\
            \n  b: unquote(\"hey\") or \"ho\";\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  a: false;\
        \n  b: \"ho\";\
        \n  b: \"hey\";\
        \n  a: false;\
        \n  b: \"ho\";\
        \n  b: hey;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/bourbon.hrx"
#[test]
#[ignore] // wrong result
fn bourbon() {
    assert_eq!(
        rsass(
            "@import \"lib/_bourbon.scss\";\
            \n\
            \nbox:hover {\
            \n  @include animation-name(scale, slide);\
            \n  @include animation-duration(2s);\
            \n  @include animation-timing-function(ease);\
            \n  @include animation-iteration-count(infinite);\
            \n\
            \n  // Animation shorthand works the same as the CSS3 animation shorthand\
            \n  @include animation(scale 1s ease-in, slide 2s ease);\
            \n}\
            \n\
            \ndiv {\
            \n  @include appearance(none);\
            \n\
            \n   /* boo boo boo */\
            \n  // Multiple image assets\
            \n  @include background-image(url(\"/images/a.png\"), url(\"images/b.png\"));\
            \n\
            \n  // Image asset with a linear-gradient\
            \n  @include background-image(url(\"/images/a.png\"), linear-gradient(white 0, yellow 50%, transparent 50%));\
            \n\
            \n  // Multiple linear-gradients - Demo\
            \n  @include background-image(linear-gradient(hsla(0, 100%, 100%, 0.25) 0%, hsla(0, 100%, 100%, 0.08) 50%, transparent 50%),\
            \n                             linear-gradient(#4e7ba3, darken(#4e7ba4, 10%)));\
            \n\
            \n  // NOT SUPPORTED - Multiple image assets with shorthand notation\
            \n  @include background-image(url(\"/images/a.png\") center no-repeat, url(\"images/b.png\") left repeat);\
            \n}\
            \n"
        )
        .unwrap(),
        "box:hover {\
        \n  -webkit-animation-name: scale, slide;\
        \n  -moz-animation-name: scale, slide;\
        \n  animation-name: scale, slide;\
        \n  -webkit-animation-duration: 2s;\
        \n  -moz-animation-duration: 2s;\
        \n  animation-duration: 2s;\
        \n  -webkit-animation-timing-function: ease;\
        \n  -moz-animation-timing-function: ease;\
        \n  animation-timing-function: ease;\
        \n  -webkit-animation-iteration-count: infinite;\
        \n  -moz-animation-iteration-count: infinite;\
        \n  animation-iteration-count: infinite;\
        \n  -webkit-animation: scale 1s ease-in, slide 2s ease;\
        \n  -moz-animation: scale 1s ease-in, slide 2s ease;\
        \n  animation: scale 1s ease-in, slide 2s ease;\
        \n}\
        \ndiv {\
        \n  -webkit-appearance: none;\
        \n  -moz-appearance: none;\
        \n  -ms-appearance: none;\
        \n  -o-appearance: none;\
        \n  appearance: none;\
        \n  /* boo boo boo */\
        \n  background-image: url(\"/images/a.png\"), url(\"images/b.png\");\
        \n  background-image: url(\"/images/a.png\"), url(\"images/b.png\");\
        \n  background-image: url(\"/images/a.png\"), -webkit-linear-gradient(white 0, yellow 50%, transparent 50%);\
        \n  background-image: url(\"/images/a.png\"), linear-gradient( white 0, yellow 50%, transparent 50%);\
        \n  background-image: -webkit-linear-gradient(rgba(255, 255, 255, 0.25) 0%, rgba(255, 255, 255, 0.08) 50%, transparent 50%), -webkit-linear-gradient(#4e7ba3, #3e6181);\
        \n  background-image: linear-gradient( rgba(255, 255, 255, 0.25) 0%, rgba(255, 255, 255, 0.08) 50%, transparent 50%), linear-gradient( #4e7ba3, #3e6181);\
        \n  background-image: -webkit-url(\"/images/a.png\")-gradient(center), -webkit-url(\"images/b.png\")-gradient(left);\
        \n  background-image: url(\"/images/a.png\")-gradient( center), url(\"images/b.png\")-gradient( left);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/calc.hrx"
#[test]
fn calc() {
    assert_eq!(
        rsass(
            "$x: 2;\
            \nbody {\
            \n  width: calc($x + 2 - 3em / hoolabaloo);\
            \n  width: -moz-calc($x + 2 - 3em / hoolabaloo);\
            \n  width: -webkit-calc($x + 2 - 3em / hoolabaloo);\
            \n  width: -ms-calc($x + 2 - 3em / hoolabaloo);\
            \n  height: foo(2 + 2);\
            \n}\
            \n"
        )
        .unwrap(),
        "body {\
        \n  width: calc($x + 2 - 3em / hoolabaloo);\
        \n  width: -moz-calc($x + 2 - 3em / hoolabaloo);\
        \n  width: -webkit-calc($x + 2 - 3em / hoolabaloo);\
        \n  width: -ms-calc($x + 2 - 3em / hoolabaloo);\
        \n  height: foo(4);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/charset"
#[test]
#[ignore] // wrong result
fn charset() {
    assert_eq!(
        rsass(
            "div {\
            \n  content: to-upper-case(\"øáéíóúüñ¿éàŤǅǂɊɱʭʬѪ\u{488}ݓ\");\
            \n}\
            \n"
        )
        .unwrap(),
        "@charset \"UTF-8\";\
        \ndiv {\
        \n  content: \"øáéíóúüñ¿éàŤǅǂɊɱʭʬѪ\u{488}ݓ\";\
        \n}\
        \n"
    );
}

mod color_functions;

// From "sass-spec/spec/libsass/conversions.hrx"
#[test]
#[ignore] // wrong result
fn conversions() {
    assert_eq!(
        rsass(
            "div {\
            \n  width: 3cm * 2in * 2in / 1cm / 1cm;\
            \n  width: 3cm * 2in / 1cm;\
            \n  width: 4cm * (12in / 3in);\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  width: 30.48in;\
        \n  width: 6in;\
        \n  width: 16cm;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/css-import.hrx"
#[test]
fn css_import() {
    assert_eq!(
        rsass(
            "@import \'foo.css\', \"bar.css\";\
            \n\
            \ndiv {\
            \n  color: red;\
            \n}"
        )
        .unwrap(),
        "@import \'foo.css\';\
        \n@import \"bar.css\";\
        \ndiv {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/css_nth_selectors.hrx"
#[test]
fn css_nth_selectors() {
    assert_eq!(
        rsass(
            ":nth-child(2n + 3) {\
            \n  outer-whitespace: false;\
            \n}\
            \n\
            \n// Regression test for sass/dart-sass#465.\
            \n:nth-child( 2n + 3 ) {\
            \n  outer-whitespace: true;\
            \n}\
            \n"
        )
        .unwrap(),
        ":nth-child(2n+3) {\
        \n  outer-whitespace: false;\
        \n}\
        \n:nth-child(2n+3) {\
        \n  outer-whitespace: true;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/css_unicode"
#[test]
#[ignore] // wrong result
fn css_unicode() {
    assert_eq!(
        rsass(
            "@charset \"UTF-8\";\
            \nfoo {\
            \n  bar: föö bâr; }\
            \n"
        )
        .unwrap(),
        "@charset \"UTF-8\";\
        \nfoo {\
        \n  bar: föö bâr;\
        \n}\
        \n"
    );
}

mod debug_directive_nested;

// From "sass-spec/spec/libsass/delayed.hrx"
#[test]
#[ignore] // unexepected error
fn delayed() {
    assert_eq!(
        rsass(
            "$x: a 3/4 b;\
            \n$y: hey;\
            \n\
            \n@function foo() {\
            \n  @return 3/4;\
            \n}\
            \n\
            \ndiv {\
            \n  hoo: 3/4;\
            \n  goo: nth($x, 2);\
            \n  foo: 15 / nth($x, 2);\
            \n  foo: .25 + nth($x, 2);\
            \n  coo: 2/3 / nth($x, 2);\
            \n  bar: $y and true;\
            \n  bar: false and true;\
            \n  bar: (false) and true;\
            \n  @each $elem in $x {\
            \n    blah: $elem;\
            \n  }\
            \n  bloo: foo();\
            \n  @warn 2/3;\
            \n  blix: \"hey #{nth($x, 2)} ho\";\
            \n}\
            \n\
            \n@media screen and (hux: 3/4) {\
            \n  div {\
            \n    color: red;\
            \n  }\
            \n}\
            \n\
            \n@warn \"blah blah\";\
            \n\
            \ndiv {\
            \n  blah: \"ho #{nth($x, 2) } ho\";\
            \n}\
            \n\
            \nspan {\
            \n  fludge: (true and 3/4);\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  hoo: 3/4;\
        \n  goo: 0.75;\
        \n  foo: 20;\
        \n  foo: 1;\
        \n  coo: 0.8888888889;\
        \n  bar: true;\
        \n  bar: false;\
        \n  bar: false;\
        \n  blah: a;\
        \n  blah: 0.75;\
        \n  blah: b;\
        \n  bloo: 0.75;\
        \n  blix: \"hey 0.75 ho\";\
        \n}\
        \n@media screen and (hux: 3/4) {\
        \n  div {\
        \n    color: red;\
        \n  }\
        \n}\
        \ndiv {\
        \n  blah: \"ho 0.75 ho\";\
        \n}\
        \nspan {\
        \n  fludge: 0.75;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/div.hrx"
#[test]
#[ignore] // wrong result
fn div() {
    assert_eq!(
        rsass(
            "$x: 3/4;\
            \n$xs: hey 3/4 ho;\
            \n\
            \ndiv {\
            \n  /* $x: 3/4 */\
            \n  a: $x;\
            \n  b: hey $x ho;\
            \n  /* $xs: hey 3/4 ho */\
            \n  c: $xs;\
            \n  d: nth($xs, 2);\
            \n  e: nth($xs, 2) == 0.75;\
            \n  f: type-of(nth($xs, 2));\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  /* $x: 3/4 */\
        \n  a: 0.75;\
        \n  b: hey 0.75 ho;\
        \n  /* $xs: hey 3/4 ho */\
        \n  c: hey 3/4 ho;\
        \n  d: 0.75;\
        \n  e: true;\
        \n  f: number;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/env.hrx"
#[test]
#[ignore] // wrong result
fn env() {
    assert_eq!(
        rsass(
            "$x: 0;\
            \n\
            \ndiv {\
            \n  /* 0 */\
            \n  font: $x;\
            \n  $x: 1 !global;\
            \n  /* 1 */\
            \n  font: $x;\
            \n  span {\
            \n    $x: 2 !global;\
            \n    /* 2 */\
            \n    font: $x;\
            \n  }\
            \n  /* 2 */\
            \n  font: $x;\
            \n  p {\
            \n    /* 2 */\
            \n    font: $x;\
            \n  }\
            \n}\
            \n\
            \ndiv {\
            \n  @foo {\
            \n    $y: 2;\
            \n    font: $y;\
            \n  }\
            \n  @bar {\
            \n    $y: 3;\
            \n    font: $y;\
            \n  }\
            \n}\
            \n\
            \n@mixin foo() {\
            \n  content: \"foo\";\
            \n  @content;\
            \n}\
            \n\
            \ndiv {\
            \n  $z: \"whatever\";\
            \n  @include foo() {\
            \n    $z: \"block for foo!\";\
            \n    font: fudge;\
            \n  }\
            \n  width: $z;\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  /* 0 */\
        \n  font: 0;\
        \n  /* 1 */\
        \n  font: 1;\
        \n  /* 2 */\
        \n  font: 2;\
        \n}\
        \ndiv span {\
        \n  /* 2 */\
        \n  font: 2;\
        \n}\
        \ndiv p {\
        \n  /* 2 */\
        \n  font: 2;\
        \n}\
        \n@foo {\
        \n  div {\
        \n    font: 2;\
        \n  }\
        \n}\
        \n@bar {\
        \n  div {\
        \n    font: 3;\
        \n  }\
        \n}\
        \ndiv {\
        \n  content: \"foo\";\
        \n  font: fudge;\
        \n  width: \"block for foo!\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/eq.hrx"
#[test]
fn eq() {
    assert_eq!(
        rsass(
            "div {\
            \n  foo: center == \"center\";\
            \n  foo: (a b c) == (a b c);\
            \n  foo: a b c == a b c;\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: a b false b c;\
        \n}\
        \n"
    );
}

mod error_directive_nested;

mod features;

// From "sass-spec/spec/libsass/http_import.hrx"
#[test]
fn http_import() {
    assert_eq!(
        rsass(
            "@import \"http://fonts.googleapis.com/css?family=Droid+Sans\";"
        )
        .unwrap(),
        "@import \"http://fonts.googleapis.com/css?family=Droid+Sans\";\
        \n"
    );
}

// From "sass-spec/spec/libsass/image-url.hrx"
#[test]
fn image_url() {
    assert_eq!(
        rsass(
            "div {\
            \n  blah: image-url(\"hello.png\", false);\
            \n  blah: image-url(\"hello.png\", true);\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  blah: image-url(\"hello.png\", false);\
        \n  blah: image-url(\"hello.png\", true);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/import.hrx"
#[test]
fn import() {
    assert_eq!(
        rsass(
            "@import \"hey1.css\", \"cookie.css\", url(\"hey2.css\"), \"fudge.css\";\
            \n\
            \n$foo:\"goodbye\";\
            \ndiv[name=\"hello\"] {\
            \n  color: blue;\
            \n}\
            \n\
            \n@import \"bludge.css\";"
        )
        .unwrap(),
        "@import \"hey1.css\";\
        \n@import \"cookie.css\";\
        \n@import url(\"hey2.css\");\
        \n@import \"fudge.css\";\
        \n@import \"bludge.css\";\
        \ndiv[name=hello] {\
        \n  color: blue;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/inh.hrx"
#[test]
#[ignore] // wrong result
fn inh() {
    assert_eq!(
        rsass(
            "foo.a {\
            \n  width: 10px;\
            \n}\
            \n\
            \nbar {\
            \n  color: red;\
            \n  @extend foo;\
            \n}"
        )
        .unwrap(),
        "foo.a, bar.a {\
        \n  width: 10px;\
        \n}\
        \nbar {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/interpolated-function-call.hrx"
#[test]
fn interpolated_function_call() {
    assert_eq!(
        rsass(
            "$f: foo;\
            \n\
            \ndiv {\
            \n  color: #{$f}(a, 1+2, c);\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  color: foo(a, 3, c);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/interpolated-urls.hrx"
#[test]
fn interpolated_urls() {
    assert_eq!(
        rsass(
            "$base_url: \"/static_loc/\";\
            \ndiv {\
            \n  background-image: \"url(\"#{$base_url}\"img/beta.png)\";\
            \n}\
            \n\
            \nspan {\
            \n  background-image: url(#{$base_url}img/beta.png);\
            \n}\
            \n\
            \nfudge {\
            \n  walnuts: blix\"fludge\"#{hey now}123;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  background-image: \"url(\" /static_loc/ \"img/beta.png)\";\
        \n}\
        \nspan {\
        \n  background-image: url(/static_loc/img/beta.png);\
        \n}\
        \nfudge {\
        \n  walnuts: blix \"fludge\" hey now123;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/keyframes.hrx"
#[test]
fn keyframes() {
    assert_eq!(
        rsass(
            "div {\
            \n  color: #181818;\
            \n}\
            \n\
            \n@-webkit-keyframes uiDelayedFadeIn {\
            \n\t0% { opacity: 0; }\
            \n\t50% { opacity: .5; }\
            \n\t100% { opacity: 1; }\
            \n}\
            \n\
            \n@-webkit-keyframes bounce {\
            \n\tfrom {\
            \n\t\tleft: 0px;\
            \n\t}\
            \n\tto {\
            \n\t\tleft: 200px;\
            \n\t}\
            \n}\
            \n\
            \n$name: bounce;\
            \n\
            \n@-webkit-keyframes #{$name} {\
            \n  blah: blee;\
            \n}\
            \n\
            \n@mixin fudge() {\
            \n  @content;\
            \n}\
            \n\
            \nfoo {\
            \n  @include fudge() {\
            \n    div {\
            \n      color: red;\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  color: #181818;\
        \n}\
        \n@-webkit-keyframes uiDelayedFadeIn {\
        \n  0% {\
        \n    opacity: 0;\
        \n  }\
        \n  50% {\
        \n    opacity: 0.5;\
        \n  }\
        \n  100% {\
        \n    opacity: 1;\
        \n  }\
        \n}\
        \n@-webkit-keyframes bounce {\
        \n  from {\
        \n    left: 0px;\
        \n  }\
        \n  to {\
        \n    left: 200px;\
        \n  }\
        \n}\
        \n@-webkit-keyframes bounce {\
        \n  blah: blee;\
        \n}\
        \nfoo div {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/list-evaluation.hrx"
#[test]
#[ignore] // wrong result
fn list_evaluation() {
    assert_eq!(
        rsass(
            "div {\
            \n  $things: red 2/3 blue;\
            \n  content: $things;\
            \n  content: nth($things, 2);\
            \n  content: type-of(nth($things, 2));\
            \n  content: type-of(nth($things, 3));\
            \n  /**** #{2+2} ****/\
            \n  content: (1 / 2 3 / 4) + (5/6 7/8);\
            \n  content: (1/2 3/4), (5/6 7/8);\
            \n  /**** ****/\
            \n  @each $x in 1 2 3/4 {\
            \n    foo: $x;\
            \n    bar: $x + 1;\
            \n  }\
            \n  /*** ***/\
            \n  stuff: 1, (2 3/4 5), 6;\
            \n  stuff: ((1 + 2)/3/4);\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  content: red 2/3 blue;\
        \n  content: 0.6666666667;\
        \n  content: number;\
        \n  content: color;\
        \n  /**** 4 ****/\
        \n  content: 1/2 3/45/6 7/8;\
        \n  content: 1/2 3/4, 5/6 7/8;\
        \n  /**** ****/\
        \n  foo: 1;\
        \n  bar: 2;\
        \n  foo: 2;\
        \n  bar: 3;\
        \n  foo: 0.75;\
        \n  bar: 1.75;\
        \n  /*** ***/\
        \n  stuff: 1, 2 3/4 5, 6;\
        \n  stuff: 0.25;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/lists.hrx"
#[test]
#[ignore] // wrong result
fn lists() {
    assert_eq!(
        rsass(
            "div {\
            \n  $list: append(1/2 3, 4);\
            \n  content: (1 2 3) == (1, 2, 3);\
            \n  content: (1 2 3) == (1 2 3);\
            \n  content: var $list;\
            \n  content: lit (1/2 3 4);\
            \n  content: (1/2 3 4) == $list;\
            \n  a: length((1/2 3 4)), length($list);\
            \n  b: nth((1/2 3 4), 1), nth($list, 1);\
            \n  content: (1/2 3 4) == (1/2 3 4);\
            \n  /***/\
            \n  content: length($list);\
            \n  content: type-of(nth($list, 1));\
            \n  content: nth($list, 1);\
            \n  content: nth(1/2 3 4, 1);\
            \n  $a: 1 2 3;\
            \n  $b: (1 2 3);\
            \n  content: $a == $b;\
            \n  content: 1 2 () 3;\
            \n  color: red == #ff0000;\
            \n  $color-list : fudge red blue;\
            \n  color: nth($color-list, 2) == #ff0000;\
            \n  color: nth($color-list, 2) == red;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  content: false;\
        \n  content: true;\
        \n  content: var 1/2 3 4;\
        \n  content: lit 1/2 3 4;\
        \n  content: true;\
        \n  a: 3, 3;\
        \n  b: 0.5, 0.5;\
        \n  content: true;\
        \n  /***/\
        \n  content: 3;\
        \n  content: number;\
        \n  content: 0.5;\
        \n  content: 0.5;\
        \n  content: true;\
        \n  content: 1 2 3;\
        \n  color: true;\
        \n  color: true;\
        \n  color: true;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/media-hoisting.hrx"
#[test]
fn media_hoisting() {
    assert_eq!(
        rsass(
            "@media screen {\
            \n  a {\
            \n    color: black;\
            \n    height: 8px;\
            \n  }\
            \n}\
            \n\
            \na {\
            \n  color: red;\
            \n  @media screen {\
            \n    color: blue;\
            \n    height: 10px;\
            \n  }\
            \n}\
            \n\
            \na {\
            \n  color: beige;\
            \n  b {\
            \n    color: teal;\
            \n    @media screen {\
            \n      color: orange;\
            \n      c {\
            \n        height: 12px;\
            \n      }\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@media screen {\
        \n  a {\
        \n    color: black;\
        \n    height: 8px;\
        \n  }\
        \n}\
        \na {\
        \n  color: red;\
        \n}\
        \n@media screen {\
        \n  a {\
        \n    color: blue;\
        \n    height: 10px;\
        \n  }\
        \n}\
        \na {\
        \n  color: beige;\
        \n}\
        \na b {\
        \n  color: teal;\
        \n}\
        \n@media screen {\
        \n  a b {\
        \n    color: orange;\
        \n  }\
        \n  a b c {\
        \n    height: 12px;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/media.hrx"
#[test]
fn media() {
    assert_eq!(
        rsass(
            "@media all and (min-width: 960px) {\
            \n  b {\
            \n    font-weight: normal;\
            \n  }\
            \n}\
            \n\
            \n@media (min-width: 980px) {\
            \n  a {\
            \n    color: red;\
            \n  }\
            \n}\
            \n\
            \n@media all {\
            \n  /* hey */\
            \n  p {\
            \n    color: blue;\
            \n    a {\
            \n      color: green;\
            \n      &:after {\
            \n        content: \">>\";\
            \n      }\
            \n    }\
            \n  }\
            \n  span {\
            \n    display: inline-block;\
            \n  }\
            \n}\
            \n\
            \na b c {\
            \n  /* a */\
            \n  blee: blee;\
            \n  /* b */\
            \n  d e f {\
            \n    blah: blah;\
            \n    bloo: bloo;\
            \n  }\
            \n  /* c */\
            \n  g h, i j {\
            \n    @media print and (foo: 1 2 3), (bar: 3px hux(muz)), not screen {\
            \n      /* aa */\
            \n      hey: ho;\
            \n      /* bb */\
            \n      k l m {\
            \n        hee: fee;\
            \n      }\
            \n      /* cc */\
            \n      haa: hoo;\
            \n      /* dd */\
            \n    }\
            \n  }\
            \n  /* d */\
            \n  blah: blah;\
            \n}\
            \n\
            \n@mixin simple-media-query($max-width, $min-width) {\
            \n  @media only screen and (max-width : $max-width) and (min-width : $min-width) {\
            \n    @content;\
            \n  }\
            \n}\
            \n\
            \n@mixin test($value) {\
            \n  border-color: $value;\
            \n}\
            \n\
            \nbody {\
            \n  @include test(\"#ccc\");\
            \n  @include simple-media-query(900px, 400px) {\
            \n    border-color: black;\
            \n  }\
            \n}\
            \n\
            \n$foo: 23;\
            \n$bar: 45;\
            \n\
            \n@media only screen and (max-width: $foo) and (min-width: $bar) {\
            \n  hey {\
            \n    ho: hoo;\
            \n  }\
            \n}\
            \n\
            \n@media (max-width: 200) and (min-width: 100) {\
            \n  div {\
            \n    color: red;\
            \n  }\
            \n}\
            \n\
            \n@media not bl#{ah} and (width: 200px) {\
            \n  div {\
            \n    color: brown, blue, black;\
            \n  }\
            \n}\
            \n\
            \n@mixin media($var1, $var2) {\
            \n  @media screen and ($var1: $var2) {\
            \n    @content;\
            \n  }\
            \n}\
            \n\
            \n@include media(max-device-width, 500px) {\
            \n  foo {\
            \n    bar: \"works\";\
            \n  }\
            \n}\
            \n\
            \ndiv {\
            \n  color: red;\
            \n  span {\
            \n    color: blue;\
            \n    @media screen {\
            \n      p {\
            \n        color: green;\
            \n      }\
            \n    }\
            \n  }\
            \n}"
        )
        .unwrap(),
        "@media all and (min-width: 960px) {\
        \n  b {\
        \n    font-weight: normal;\
        \n  }\
        \n}\
        \n@media (min-width: 980px) {\
        \n  a {\
        \n    color: red;\
        \n  }\
        \n}\
        \n@media all {\
        \n  /* hey */\
        \n  p {\
        \n    color: blue;\
        \n  }\
        \n  p a {\
        \n    color: green;\
        \n  }\
        \n  p a:after {\
        \n    content: \">>\";\
        \n  }\
        \n  span {\
        \n    display: inline-block;\
        \n  }\
        \n}\
        \na b c {\
        \n  /* a */\
        \n  blee: blee;\
        \n  /* b */\
        \n  /* c */\
        \n  /* d */\
        \n  blah: blah;\
        \n}\
        \na b c d e f {\
        \n  blah: blah;\
        \n  bloo: bloo;\
        \n}\
        \n@media print and (foo: 1 2 3), (bar: 3px hux(muz)), not screen {\
        \n  a b c g h, a b c i j {\
        \n    /* aa */\
        \n    hey: ho;\
        \n    /* bb */\
        \n    /* cc */\
        \n    haa: hoo;\
        \n    /* dd */\
        \n  }\
        \n  a b c g h k l m, a b c i j k l m {\
        \n    hee: fee;\
        \n  }\
        \n}\
        \nbody {\
        \n  border-color: \"#ccc\";\
        \n}\
        \n@media only screen and (max-width: 900px) and (min-width: 400px) {\
        \n  body {\
        \n    border-color: black;\
        \n  }\
        \n}\
        \n@media only screen and (max-width: 23) and (min-width: 45) {\
        \n  hey {\
        \n    ho: hoo;\
        \n  }\
        \n}\
        \n@media (max-width: 200) and (min-width: 100) {\
        \n  div {\
        \n    color: red;\
        \n  }\
        \n}\
        \n@media not blah and (width: 200px) {\
        \n  div {\
        \n    color: brown, blue, black;\
        \n  }\
        \n}\
        \n@media screen and (max-device-width: 500px) {\
        \n  foo {\
        \n    bar: \"works\";\
        \n  }\
        \n}\
        \ndiv {\
        \n  color: red;\
        \n}\
        \ndiv span {\
        \n  color: blue;\
        \n}\
        \n@media screen {\
        \n  div span p {\
        \n    color: green;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/mixin.hrx"
#[test]
#[ignore] // wrong result
fn mixin() {
    assert_eq!(
        rsass(
            "// @charset \"UTF-8\";\
            \n\
            \n@mixin background-image-retina($file, $type, $width, $height) {\
            \n  background-image: unquote(image-url(\"#{$file}.#{$type}\", true));\
            \n  hey: length(a b c d);\
            \n  ho: unquote(\"hello\");\
            \n  hee: unquote(unit(10fudge));\
            \n\
            \n  @media (-webkit-min-device-pixel-ratio: 2), (-moz-min-device-pixel-ratio: 2) {\
            \n    & {\
            \n      background-image: image-url(\"#{$file}@2x.#{$type}\");\
            \n      -webkit-background-size: $width $height;\
            \n    }\
            \n  }\
            \n}\
            \n\
            \n$x: foo;\
            \n$y: bar;\
            \n\
            \ndiv {\
            \n  @include background-image-retina(test, png, 100px, 300px);\
            \n  fudge: walnut;\
            \n}\
            \n\
            \nspan {\
            \n  blah: \"#{$x} #{$y}\";\
            \n  bleh: image-url(\"#{$x} #{$y}\");\
            \n}\
            \n\
            \n\
            \n@mixin foo($x, $y) {\
            \n  foo-x: $x;\
            \n  foo-y: $y;\
            \n}\
            \n\
            \ndiv {\
            \n  @include foo(1, 2);\
            \n  @include foo($y: 2, $x: 1);\
            \n}\
            \n\
            \n@mixin bar($x, $y: default) {\
            \n  bar-x: $x;\
            \n  bar-y: $y;\
            \n}\
            \n\
            \ndiv {\
            \n  @include bar(1, 2);\
            \n  @include bar(1);\
            \n  @include bar($x: n1, $y: n2);\
            \n  @include bar($x: n1);\
            \n  blah: unquote(\"hello\");\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  background-image: image-url(\"test.png\", true);\
        \n  hey: 4;\
        \n  ho: hello;\
        \n  hee: fudge;\
        \n  fudge: walnut;\
        \n}\
        \n@media (-webkit-min-device-pixel-ratio: 2), (-moz-min-device-pixel-ratio: 2) {\
        \n  div {\
        \n    background-image: image-url(\"test@2x.png\");\
        \n    -webkit-background-size: 100px 300px;\
        \n  }\
        \n}\
        \nspan {\
        \n  blah: \"foo bar\";\
        \n  bleh: image-url(\"foo bar\");\
        \n}\
        \ndiv {\
        \n  foo-x: 1;\
        \n  foo-y: 2;\
        \n  foo-x: 1;\
        \n  foo-y: 2;\
        \n}\
        \ndiv {\
        \n  bar-x: 1;\
        \n  bar-y: 2;\
        \n  bar-x: 1;\
        \n  bar-y: default;\
        \n  bar-x: n1;\
        \n  bar-y: n2;\
        \n  bar-x: n1;\
        \n  bar-y: default;\
        \n  blah: hello;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/mixins-and-media-queries.hrx"
#[test]
#[ignore] // wrong result
fn mixins_and_media_queries() {
    assert_eq!(
        rsass(
            "@media screen and (orientation:landscape) {\
            \n  span {\
            \n    background: blue;\
            \n  }\
            \n  /* fudge */\
            \n  // @include foo;\
            \n  /* budge */\
            \n  div {\
            \n    color: red;\
            \n  }\
            \n}\
            \n\
            \n@mixin testComments {\
            \n  /* crash */\
            \n  p {\
            \n    width: 100px;\
            \n  }\
            \n}\
            \n\
            \n@media screen and (orientation:landscape) {\
            \n  @include testComments;\
            \n}"
        )
        .unwrap(),
        "@media screen and (orientation: landscape) {\
        \n  span {\
        \n    background: blue;\
        \n  }\
        \n  /* fudge */\
        \n  /* budge */\
        \n  div {\
        \n    color: red;\
        \n  }\
        \n}\
        \n@media screen and (orientation: landscape) {\
        \n  /* crash */\
        \n  p {\
        \n    width: 100px;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/multi-blocks.hrx"
#[test]
fn multi_blocks() {
    assert_eq!(
        rsass(
            "a b {\
            \n  color: red;\
            \n  c d {\
            \n    height: 10;\
            \n  }\
            \n  e f {\
            \n    width: 12;\
            \n  }\
            \n}\
            \n\
            \n@media all and (min-width: 960px) {\
            \n  b {\
            \n    font-weight: normal;\
            \n  }\
            \n}\
            \n\
            \n@media (min-width: 980px) {\
            \n  a {\
            \n    color: red;\
            \n  }\
            \n}\
            \n\
            \n@media screen and (all) {\
            \n  /* hey */\
            \n  p {\
            \n    color: blue;\
            \n    a {\
            \n      color: green;\
            \n      &:after {\
            \n        content: \">>\";\
            \n      }\
            \n    }\
            \n  }\
            \n  span {\
            \n    display: inline-block;\
            \n  }\
            \n}\
            \n\
            \na b c {\
            \n  /* a */\
            \n  blee: blee;\
            \n  /* b */\
            \n  d e f {\
            \n    blah: blah;\
            \n    bloo: bloo;\
            \n  }\
            \n  /* c */\
            \n  g h, i j {\
            \n    @media print and (foo: 1 2 3), (bar: 3px hux(muz)), not screen {\
            \n      /* aa */\
            \n      hey: ho;\
            \n      /* bb */\
            \n      k l m {\
            \n        hee: fee;\
            \n      }\
            \n      /* cc */\
            \n      haa: hoo;\
            \n      /* dd */\
            \n    }\
            \n  }\
            \n  /* d */\
            \n  blah: blah;\
            \n}"
        )
        .unwrap(),
        "a b {\
        \n  color: red;\
        \n}\
        \na b c d {\
        \n  height: 10;\
        \n}\
        \na b e f {\
        \n  width: 12;\
        \n}\
        \n@media all and (min-width: 960px) {\
        \n  b {\
        \n    font-weight: normal;\
        \n  }\
        \n}\
        \n@media (min-width: 980px) {\
        \n  a {\
        \n    color: red;\
        \n  }\
        \n}\
        \n@media screen and (all) {\
        \n  /* hey */\
        \n  p {\
        \n    color: blue;\
        \n  }\
        \n  p a {\
        \n    color: green;\
        \n  }\
        \n  p a:after {\
        \n    content: \">>\";\
        \n  }\
        \n  span {\
        \n    display: inline-block;\
        \n  }\
        \n}\
        \na b c {\
        \n  /* a */\
        \n  blee: blee;\
        \n  /* b */\
        \n  /* c */\
        \n  /* d */\
        \n  blah: blah;\
        \n}\
        \na b c d e f {\
        \n  blah: blah;\
        \n  bloo: bloo;\
        \n}\
        \n@media print and (foo: 1 2 3), (bar: 3px hux(muz)), not screen {\
        \n  a b c g h, a b c i j {\
        \n    /* aa */\
        \n    hey: ho;\
        \n    /* bb */\
        \n    /* cc */\
        \n    haa: hoo;\
        \n    /* dd */\
        \n  }\
        \n  a b c g h k l m, a b c i j k l m {\
        \n    hee: fee;\
        \n  }\
        \n}\
        \n"
    );
}

mod parent_selector;

// From "sass-spec/spec/libsass/placeholder-mediaquery.hrx"
#[test]
#[ignore] // wrong result
fn placeholder_mediaquery() {
    assert_eq!(
        rsass(
            "%foo {\
            \n\t@media screen and (min-width: 300px) {\
            \n\t\tmax-width: 80%;\
            \n\t}\
            \n}\
            \n\
            \nbar {\
            \n\t@extend %foo;\
            \n}\
            \n"
        )
        .unwrap(),
        "@media screen and (min-width: 300px) {\
        \n  bar {\
        \n    max-width: 80%;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/placeholder-nested.hrx"
#[test]
#[ignore] // unexepected error
fn placeholder_nested() {
    assert_eq!(
        rsass(
            "%x {\
            \n  width: 100px;\
            \n\
            \n  %y {\
            \n    height: 100px;\
            \n  }\
            \n}\
            \n\
            \n.foo {\
            \n  @extend %x;\
            \n\
            \n  .bar { @extend %y }\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  width: 100px;\
        \n}\
        \n.foo .bar {\
        \n  height: 100px;\
        \n}\
        \n"
    );
}

mod precision;

// From "sass-spec/spec/libsass/properties-in-media.hrx"
#[test]
#[ignore] // wrong result
fn properties_in_media() {
    assert_eq!(
        rsass(
            "@media only screen {\r\
            \n    /* asd */\r\
            \n  color: red;\r\
            \n  color: gray;\r\
            \n    /* asd */\r\
            \n  color: green;\r\
            \n  foo {\r\
            \n    bar: baz;\r\
            \n    qwe: baz;\r\
            \n  }\r\
            \n  color: blue;\r\
            \n  color: yellow;\r\
            \n}"
        )
        .unwrap(),
        "@media only screen {\
        \n  /* asd */\
        \n  color: red;\
        \n  color: gray;\
        \n  /* asd */\
        \n  color: green;\
        \n  foo {\
        \n    bar: baz;\
        \n    qwe: baz;\
        \n  }\
        \n  color: blue;\
        \n  color: yellow;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/propsets.hrx"
#[test]
#[ignore] // wrong result
fn propsets() {
    assert_eq!(
        rsass(
            "$x: ground;\
            \n$y: e;\
            \n$z: it;\
            \n\
            \ndiv {\
            \n  back#{$x}: {\
            \n    imag#{$y}: url(foo.png);\
            \n    pos#{$z}ion: 50%;\
            \n  }\
            \n}\
            \n\
            \nspan {\
            \n  background: {\
            \n    image: url(bar.png);\
            \n    position: 100%;\
            \n  }\
            \n}\
            \n\
            \np {\
            \n  border: {\
            \n    upper: {\
            \n      left: 2px;\
            \n      right: 3px;\
            \n    }\
            \n  }\
            \n}\
            \n\
            \n@warn 2 + 3;\
            \n\
            \n/* 2 + 3 */\
            \n/* #{2 + 3} */\
            \n\
            \nfoo|div {\
            \n  color: red;\
            \n}\
            \n\
            \n$-hey : hey;\
            \n\
            \ndiv sp\\ ,#abcan {\
            \n  color: red;\
            \n  p, |q {\
            \n    background: blue;\
            \n    color: \\hey;\
            \n    width: \\10 + \\20 \\ ;\
            \n    a {\
            \n      height: 1;\
            \n    }\
            \n  }\
            \n}\
            \n\
            \nd\\ v, sp\\ n {\
            \n  a {\
            \n    color: blue;\
            \n  }\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  background-image: url(foo.png);\
        \n  background-position: 50%;\
        \n}\
        \nspan {\
        \n  background-image: url(bar.png);\
        \n  background-position: 100%;\
        \n}\
        \np {\
        \n  border-upper-left: 2px;\
        \n  border-upper-right: 3px;\
        \n}\
        \n/* 2 + 3 */\
        \n/* 5 */\
        \nfoo|div {\
        \n  color: red;\
        \n}\
        \ndiv sp\\ , #abcan {\
        \n  color: red;\
        \n}\
        \ndiv sp\\  p, div sp\\  |q, #abcan p, #abcan |q {\
        \n  background: blue;\
        \n  color: hey;\
        \n  width: \\10 \\ \\ ;\
        \n}\
        \ndiv sp\\  p a, div sp\\  |q a, #abcan p a, #abcan |q a {\
        \n  height: 1;\
        \n}\
        \nd\\ v a, sp\\ n a {\
        \n  color: blue;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/rel.hrx"
#[test]
#[ignore] // wrong result
fn rel() {
    assert_eq!(
        rsass(
            "div {\
            \n  less: 3px < 3pt;\
            \n  less: (1px / 1pt);\
            \n  less: 23fu < 120;\
            \n  eq: hello == hello;\
            \n  eq: \"hello\" == hello;\
            \n  eq: (1 2 3) == (1 2 3);\
            \n  eq: (1 2 3) == (1, 2, 3);\
            \n  eq: 23px == 23fu;\
            \n  eq: 3.1in == 2.54cm;\
            \n  eq: 2.54cm == 3.1in;\
            \n  eq: (1in) == (1cm*1in/1cm);\
            \n  x: 1in, (1cm*1in/1cm);\
            \n  y: 1cm*1in/1in;\
            \n  eq: (2cm*1in/2cm) == (1in*2cm/2cm);\
            \n  blah: (1cm/1in);\
            \n  in: (1in*2.54cm/1in);\
            \n  lt: 1in < 2.54cm;\
            \n  lt: 2.54cm < 1in;\
            \n  lt: 5 < 4;\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  less: true;\
        \n  less: 0.75;\
        \n  less: true;\
        \n  eq: true;\
        \n  eq: true;\
        \n  eq: true;\
        \n  eq: false;\
        \n  eq: false;\
        \n  eq: false;\
        \n  eq: false;\
        \n  eq: true;\
        \n  x: 1in, 1in;\
        \n  y: 0.3937007874in;\
        \n  eq: true;\
        \n  blah: 0.3937007874;\
        \n  in: 2.54cm;\
        \n  lt: false;\
        \n  lt: false;\
        \n  lt: false;\
        \n}\
        \n"
    );
}

mod selector_functions;

// From "sass-spec/spec/libsass/selector_interpolation_in_string.hrx"
#[test]
fn selector_interpolation_in_string() {
    assert_eq!(
        rsass(
            "foo[val=\"bar #{\"foo\" + \" bar\"} baz\"] {a: b}\
            \n"
        )
        .unwrap(),
        "foo[val=\"bar foo bar baz\"] {\
        \n  a: b;\
        \n}\
        \n"
    );
}

mod selectors;

// From "sass-spec/spec/libsass/test.hrx"
#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        rsass(
            "$x: 3;\
            \n\
            \ndiv {\
            \n\tnoo: not $x;\
            \n\tpoo: not 3;\
            \n\tdoo: not($x);\
            \n\tgoo: not(3);\
            \n\tzoo: not 1 + 2;\
            \n\
            \n\troo: not not not $x;\
            \n\thoo: not not not 3;\
            \n}\
            \n\
            \n@mixin foo($x-1, $x-2) {\
            \n  goo: $x-1;\
            \n  poo: $x-2;\
            \n}\
            \n$hux: \"blah.css\";\
            \nspan {\
            \n  a: rgba(100, 20, 0, 1);\
            \n  b: rgba(#abc, 1);\
            \n  c: compact(hello, my, false, name, is, false, aaron, false, false);\
            \n  d: join(1 2 3, 4 5 6, comma);\
            \n  e: join(a b c, d e f);\
            \n  f: change-color(#102030, $blue: 5);\
            \n  g: change-color(#102030, $red: 120, $blue: 5);\
            \n  h: hsl(25, 100%, 80%);\
            \n  h: change-color(#ffc499, $alpha: 0.8, $lightness: 40%);\
            \n  h: change-color(hsl(25, 100%, 80%), $alpha: 0.8, $lightness: 40%);\
            \n  i: hsla(25, 100%, 40%, 0.8);\
            \n  foo: url(\"http://blah/flah/grah\");\
            \n  foo: url(http://foo/bar/buzz.css);\
            \n  foo: url(hey#{1+3}ho.css);\
            \n  foo: url($hux);\
            \n  bug: compact(false 1 2 false 3 4 5 false);\
            \n  pug: compact(false, 1, 2, false, 3, 4, 5, false);\
            \n  mug: compact((flug, false, blug, false, krug, false));\
            \n}\
            \n\
            \n@mixin bg($file) {\
            \n  background: url($file) no-repeat;\
            \n}\
            \n\
            \ndiv {\
            \n  flug: url(bug.mug);\
            \n  krug: nth(1 2 3, 2px);\
            \n  blug: nth(a b c d, 3);\
            \n  flig: comparable(34, 22px) comparable(1%, 3) comparable(2, 1) comparable(4cm, 1in);\
            \n  flug: comparable(1px, 2.3in) comparable(1%, 2pt);\
            \n  flib: comparable(3ex, 2px) comparable(3em, 2cm);\
            \n  glib: not(fudge) not(false) not(0) not(red);\
            \n  trib: if(red, yellow, not taken);\
            \n  trub: if(not(fudge), not taken, here we are);\
            \n}\
            \n\
            \n$width: 10px;\
            \n$height: 10px;\
            \n@media (-webkit-min-device-pixel-ratio: 2), (-moz-min-device-pixel-ratio: 2) {\
            \n  div {\
            \n    background-image: url(fudge);\
            \n    -webkit-background-size: $width $height;\
            \n  }\
            \n}\
            \n\
            \nspan {\
            \n  @media foo {\
            \n    div {\
            \n      blah: blah;\
            \n    }\
            \n    @media bar {\
            \n      p {\
            \n        blah: bloo;\
            \n      }\
            \n    }\
            \n  }\
            \n}\
            \n\
            \ngudge {\
            \n  fudge: 20 + \"%\";\
            \n  mudge: 1 + blang;\
            \n}\
            \n\
            \nh1:lang(en) {\
            \n  foo: bar;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  noo: false;\
        \n  poo: false;\
        \n  doo: false;\
        \n  goo: false;\
        \n  zoo: false2;\
        \n  roo: false;\
        \n  hoo: false;\
        \n}\
        \nspan {\
        \n  a: #641400;\
        \n  b: #aabbcc;\
        \n  c: compact(hello, my, false, name, is, false, aaron, false, false);\
        \n  d: 1, 2, 3, 4, 5, 6;\
        \n  e: a b c d e f;\
        \n  f: #102005;\
        \n  g: #782005;\
        \n  h: #ffc499;\
        \n  h: rgba(204, 86, 0, 0.8);\
        \n  h: rgba(204, 85, 0, 0.8);\
        \n  i: rgba(204, 85, 0, 0.8);\
        \n  foo: url(\"http://blah/flah/grah\");\
        \n  foo: url(http://foo/bar/buzz.css);\
        \n  foo: url(hey4ho.css);\
        \n  foo: url(\"blah.css\");\
        \n  bug: compact(false 1 2 false 3 4 5 false);\
        \n  pug: compact(false, 1, 2, false, 3, 4, 5, false);\
        \n  mug: compact(flug, false, blug, false, krug, false);\
        \n}\
        \ndiv {\
        \n  flug: url(bug.mug);\
        \n  krug: 2;\
        \n  blug: c;\
        \n  flig: true true true true;\
        \n  flug: true false;\
        \n  flib: false false;\
        \n  glib: false true false false;\
        \n  trib: yellow;\
        \n  trub: here we are;\
        \n}\
        \n@media (-webkit-min-device-pixel-ratio: 2), (-moz-min-device-pixel-ratio: 2) {\
        \n  div {\
        \n    background-image: url(fudge);\
        \n    -webkit-background-size: 10px 10px;\
        \n  }\
        \n}\
        \n@media foo {\
        \n  span div {\
        \n    blah: blah;\
        \n  }\
        \n}\
        \ngudge {\
        \n  fudge: \"20%\";\
        \n  mudge: 1blang;\
        \n}\
        \nh1:lang(en) {\
        \n  foo: bar;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass/unary-ops.hrx"
#[test]
#[ignore] // wrong result
fn unary_ops() {
    assert_eq!(
        rsass(
            "$x: 20%;\
            \n\
            \ndiv {\
            \n  a: -10;\
            \n  b: -10px + 10px;\
            \n  c: +10;\
            \n  d: +10px + -10px;\
            \n  e: -$x;\
            \n  f: +$x;\
            \n  g: -hello;\
            \n  h: +hello;\
            \n  i: + hello;\
            \n  j: type-of(+ hello);\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  a: -10;\
        \n  b: 0px;\
        \n  c: 10;\
        \n  d: 0px;\
        \n  e: -20%;\
        \n  f: 20%;\
        \n  g: -hello;\
        \n  h: +hello;\
        \n  i: +hello;\
        \n  j: string;\
        \n}\
        \n"
    );
}

mod unicode_bom;

mod units;

// From "sass-spec/spec/libsass/url.hrx"
#[test]
#[ignore] // unexepected error
fn url() {
    assert_eq!(
        rsass(
            "$x: pop;\
            \n$y: 123;\
            \n\
            \n\
            \n\
            \ndiv {\
            \n  foo: url(bloo/blah.css);\
            \n  bar: url(http://foo/bar/hux.css);\
            \n  foo: url(fudge#{$x}.css);\
            \n  bar: url(\"http://fudge#{$x}/styles.css\");\
            \n  hux: url(http://box_#{$y}////fudge#{$x}.css);\
            \n  @each $i in (1 2 3 4 5) {\
            \n    hux: url(http://box_#{$y}////fudge#{$x}.css);\
            \n    foo: url(http://blah.com/bar-#{$i}.css);\
            \n  }\
            \n  gloo: url(\"hey#{1+2}.css\");\
            \n  floo: url(hadoop-#{$y+321}.css);\
            \n  flum: image-url(\"fudge.png\", hux);\
            \n  /*****/\
            \n  $bg: \"image.png\";\
            \n  background: url(\"#{$bg}\");\
            \n  //gudge: url(type-of(hello) + length(a b c));\
            \n  mudge: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAACXBIWXMAAAsTAAALEwEAmpwYAAAABGdBTUEAALGOfPtRkwAAACBjSFJNAAB6JQAAgIMAAPn/AACA6QAAdTAAAOpgAAA6mAAAF2+SX8VGAAACeElEQVR42nySy29McRTHP/fOnTvT6bQNrdHKMGGhFkTSELGxwoJY8Q9YWFhYEUJsRSKCsJWikjYSJBIbinpVPJLSRlEkKK2WTnXmvl+/Y4F4tPVJPqtvzjcnJ0cTEQxdY/miFH6gcAJpaWrQl86t05rR9axSKD8UZ6KqJscm5bMdyDDgAYgIBoCORm2G1u0b6w8unJ/bmDG1QtpUmIYiZ8Zk0zEpYmW76tujV9J3/Ep04v0XdR2IDYAdWxYt27Sa8/l8btWIlaYSupgqpNaMUYbC0DUa8qKXWpLGNSvZEETpZO/Z4B5gGQCRMio1xdVfioUIa3AQJ/ZARWhJgkQJKq3wfJ3RwETGhRtPgx7ABtBEhCVNBqViU2tn5+5bLfXmgurIYwJrGFEJmqZh2T4jo2X0YIreZ+7dfeejrcCEiKADfCon3O4fHzp25Nx+8nnqF65lXnEphQUtNBYKaKkMcRgxVY29093JUWCCn+gAORMaTLh0dbCjo/1KO3X1kC6BGIR+QLVioSc+F+9HnW/G1DX+QAcw0j8c/QaHj3UfeN0/MMicEmSL+J5P6DkMDUcfLvZGJ4FwWoHl/lAEXo344zv3dO3ynXJIpg7XdnBtj46bwSnblwH+QQdQ8lsNeNg32nOm/fIh3CGS0OXOQHCv90XYwUyICM2NNX85f26WUnOu5smFzX0vu9qktZjeNtusAbB+XdvfAWDZnjeurX2XST1Y8X6s7zmzYABUrHBaYNshYRC4k340FcZU/1vg2JVpgeP4uJXypHK8soD134In/W+mb+AJvffvvC022It/ve1MaCJCXU6f4UCQy1CbNVONH7/Gw7Md8fsAtddMUh5fveYAAAAASUVORK5CYII=);\
            \n  nudge: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAAAAAA6mKC9AAAAGElEQVQYV2N4DwX/oYBhgARgDJjEAAkAAEC99wFuu0VFAAAAAElFTkSuQmCC);\
            \n  pudge: url(http://wiki.jbussdieker.name/skins/common/images/Checker-16x16.png?2012-05-02T13:40:00Z);\
            \n}\
            \n\
            \np:after {\
            \n  content:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAACXBIWXMAAAsTAAALEwEAmpwYAAAABGdBTUEAALGOfPtRkwAAACBjSFJNAAB6JQAAgIMAAPn/AACA6QAAdTAAAOpgAAA6mAAAF2+SX8VGAAACeElEQVR42nySy29McRTHP/fOnTvT6bQNrdHKMGGhFkTSELGxwoJY8Q9YWFhYEUJsRSKCsJWikjYSJBIbinpVPJLSRlEkKK2WTnXmvl+/Y4F4tPVJPqtvzjcnJ0cTEQxdY/miFH6gcAJpaWrQl86t05rR9axSKD8UZ6KqJscm5bMdyDDgAYgIBoCORm2G1u0b6w8unJ/bmDG1QtpUmIYiZ8Zk0zEpYmW76tujV9J3/Ep04v0XdR2IDYAdWxYt27Sa8/l8btWIlaYSupgqpNaMUYbC0DUa8qKXWpLGNSvZEETpZO/Z4B5gGQCRMio1xdVfioUIa3AQJ/ZARWhJgkQJKq3wfJ3RwETGhRtPgx7ABtBEhCVNBqViU2tn5+5bLfXmgurIYwJrGFEJmqZh2T4jo2X0YIreZ+7dfeejrcCEiKADfCon3O4fHzp25Nx+8nnqF65lXnEphQUtNBYKaKkMcRgxVY29093JUWCCn+gAORMaTLh0dbCjo/1KO3X1kC6BGIR+QLVioSc+F+9HnW/G1DX+QAcw0j8c/QaHj3UfeN0/MMicEmSL+J5P6DkMDUcfLvZGJ4FwWoHl/lAEXo344zv3dO3ynXJIpg7XdnBtj46bwSnblwH+QQdQ8lsNeNg32nOm/fIh3CGS0OXOQHCv90XYwUyICM2NNX85f26WUnOu5smFzX0vu9qktZjeNtusAbB+XdvfAWDZnjeurX2XST1Y8X6s7zmzYABUrHBaYNshYRC4k340FcZU/1vg2JVpgeP4uJXypHK8soD134In/W+mb+AJvffvvC022It/ve1MaCJCXU6f4UCQy1CbNVONH7/Gw7Md8fsAtddMUh5fveYAAAAASUVORK5CYII= );\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  foo: url(bloo/blah.css);\
        \n  bar: url(http://foo/bar/hux.css);\
        \n  foo: url(fudgepop.css);\
        \n  bar: url(\"http://fudgepop/styles.css\");\
        \n  hux: url(http://box_123////fudgepop.css);\
        \n  hux: url(http://box_123////fudgepop.css);\
        \n  foo: url(http://blah.com/bar-1.css);\
        \n  hux: url(http://box_123////fudgepop.css);\
        \n  foo: url(http://blah.com/bar-2.css);\
        \n  hux: url(http://box_123////fudgepop.css);\
        \n  foo: url(http://blah.com/bar-3.css);\
        \n  hux: url(http://box_123////fudgepop.css);\
        \n  foo: url(http://blah.com/bar-4.css);\
        \n  hux: url(http://box_123////fudgepop.css);\
        \n  foo: url(http://blah.com/bar-5.css);\
        \n  gloo: url(\"hey3.css\");\
        \n  floo: url(hadoop-444.css);\
        \n  flum: image-url(\"fudge.png\", hux);\
        \n  /*****/\
        \n  background: url(\"image.png\");\
        \n  mudge: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAACXBIWXMAAAsTAAALEwEAmpwYAAAABGdBTUEAALGOfPtRkwAAACBjSFJNAAB6JQAAgIMAAPn/AACA6QAAdTAAAOpgAAA6mAAAF2+SX8VGAAACeElEQVR42nySy29McRTHP/fOnTvT6bQNrdHKMGGhFkTSELGxwoJY8Q9YWFhYEUJsRSKCsJWikjYSJBIbinpVPJLSRlEkKK2WTnXmvl+/Y4F4tPVJPqtvzjcnJ0cTEQxdY/miFH6gcAJpaWrQl86t05rR9axSKD8UZ6KqJscm5bMdyDDgAYgIBoCORm2G1u0b6w8unJ/bmDG1QtpUmIYiZ8Zk0zEpYmW76tujV9J3/Ep04v0XdR2IDYAdWxYt27Sa8/l8btWIlaYSupgqpNaMUYbC0DUa8qKXWpLGNSvZEETpZO/Z4B5gGQCRMio1xdVfioUIa3AQJ/ZARWhJgkQJKq3wfJ3RwETGhRtPgx7ABtBEhCVNBqViU2tn5+5bLfXmgurIYwJrGFEJmqZh2T4jo2X0YIreZ+7dfeejrcCEiKADfCon3O4fHzp25Nx+8nnqF65lXnEphQUtNBYKaKkMcRgxVY29093JUWCCn+gAORMaTLh0dbCjo/1KO3X1kC6BGIR+QLVioSc+F+9HnW/G1DX+QAcw0j8c/QaHj3UfeN0/MMicEmSL+J5P6DkMDUcfLvZGJ4FwWoHl/lAEXo344zv3dO3ynXJIpg7XdnBtj46bwSnblwH+QQdQ8lsNeNg32nOm/fIh3CGS0OXOQHCv90XYwUyICM2NNX85f26WUnOu5smFzX0vu9qktZjeNtusAbB+XdvfAWDZnjeurX2XST1Y8X6s7zmzYABUrHBaYNshYRC4k340FcZU/1vg2JVpgeP4uJXypHK8soD134In/W+mb+AJvffvvC022It/ve1MaCJCXU6f4UCQy1CbNVONH7/Gw7Md8fsAtddMUh5fveYAAAAASUVORK5CYII=);\
        \n  nudge: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAAAAAA6mKC9AAAAGElEQVQYV2N4DwX/oYBhgARgDJjEAAkAAEC99wFuu0VFAAAAAElFTkSuQmCC);\
        \n  pudge: url(http://wiki.jbussdieker.name/skins/common/images/Checker-16x16.png?2012-05-02T13:40:00Z);\
        \n}\
        \np:after {\
        \n  content: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAACXBIWXMAAAsTAAALEwEAmpwYAAAABGdBTUEAALGOfPtRkwAAACBjSFJNAAB6JQAAgIMAAPn/AACA6QAAdTAAAOpgAAA6mAAAF2+SX8VGAAACeElEQVR42nySy29McRTHP/fOnTvT6bQNrdHKMGGhFkTSELGxwoJY8Q9YWFhYEUJsRSKCsJWikjYSJBIbinpVPJLSRlEkKK2WTnXmvl+/Y4F4tPVJPqtvzjcnJ0cTEQxdY/miFH6gcAJpaWrQl86t05rR9axSKD8UZ6KqJscm5bMdyDDgAYgIBoCORm2G1u0b6w8unJ/bmDG1QtpUmIYiZ8Zk0zEpYmW76tujV9J3/Ep04v0XdR2IDYAdWxYt27Sa8/l8btWIlaYSupgqpNaMUYbC0DUa8qKXWpLGNSvZEETpZO/Z4B5gGQCRMio1xdVfioUIa3AQJ/ZARWhJgkQJKq3wfJ3RwETGhRtPgx7ABtBEhCVNBqViU2tn5+5bLfXmgurIYwJrGFEJmqZh2T4jo2X0YIreZ+7dfeejrcCEiKADfCon3O4fHzp25Nx+8nnqF65lXnEphQUtNBYKaKkMcRgxVY29093JUWCCn+gAORMaTLh0dbCjo/1KO3X1kC6BGIR+QLVioSc+F+9HnW/G1DX+QAcw0j8c/QaHj3UfeN0/MMicEmSL+J5P6DkMDUcfLvZGJ4FwWoHl/lAEXo344zv3dO3ynXJIpg7XdnBtj46bwSnblwH+QQdQ8lsNeNg32nOm/fIh3CGS0OXOQHCv90XYwUyICM2NNX85f26WUnOu5smFzX0vu9qktZjeNtusAbB+XdvfAWDZnjeurX2XST1Y8X6s7zmzYABUrHBaYNshYRC4k340FcZU/1vg2JVpgeP4uJXypHK8soD134In/W+mb+AJvffvvC022It/ve1MaCJCXU6f4UCQy1CbNVONH7/Gw7Md8fsAtddMUh5fveYAAAAASUVORK5CYII=);\
        \n}\
        \n"
    );
}

mod variable_scoping;

// From "sass-spec/spec/libsass/variables_in_media.hrx"
#[test]
fn variables_in_media() {
    assert_eq!(
        rsass(
            "$media1: screen;\
            \n$media2: print;\
            \n$var: -webkit-min-device-pixel-ratio;\
            \n$val: 20;\
            \n@media #{$media1} and ($var: $val), only #{$media2} {a {b: c}}\
            \n"
        )
        .unwrap(),
        "@media screen and (-webkit-min-device-pixel-ratio: 20), only print {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n"
    );
}

mod warn_directive_nested;

// From "sass-spec/spec/libsass/wrapped-selector-whitespace.hrx"
#[test]
#[ignore] // wrong result
fn wrapped_selector_whitespace() {
    assert_eq!(
        rsass(
            "div {\r\
            \n  :-moz-any(a , b) {\r\
            \n    foo: foo;\r\
            \n  }\r\
            \n  :foo(a , b) {\r\
            \n    bar: bar;\r\
            \n  }\r\
            \n}\r\
            \n"
        )
        .unwrap(),
        "div :-moz-any(a, b) {\
        \n  foo: foo;\
        \n}\
        \ndiv :foo(a , b) {\
        \n  bar: bar;\
        \n}\
        \n"
    );
}
