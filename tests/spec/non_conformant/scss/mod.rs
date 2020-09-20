//! Tests auto-converted from "sass-spec/spec/non_conformant/scss"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/non_conformant/scss/almost_ambiguous_nested_rules_and_declarations.hrx"
#[test]
fn almost_ambiguous_nested_rules_and_declarations() {
    assert_eq!(
        rsass(
            "foo {\
            \n  bar:baz:bang:bop:biddle:woo:look:at:all:these:pseudoclasses {a: b};\
            \n  bar:baz bang bop biddle woo look at all these elems {a: b};\
            \n  bar:baz bang bop biddle woo look at all these elems; }\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: baz bang bop biddle woo look at all these elems;\
        \n}\
        \nfoo bar:baz:bang:bop:biddle:woo:look:at:all:these:pseudoclasses {\
        \n  a: b;\
        \n}\
        \nfoo bar:baz bang bop biddle woo look at all these elems {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/alpha.hrx"
#[test]
fn alpha() {
    assert_eq!(
        rsass(
            "$x: rgb(0, 255, 255);\
            \n\
            \ndiv {\
            \n  color: rgb(255, $blue: 0, $green: 255);\
            \n  background: rgb(123, 45, 6);\
            \n//  flah: rgba(0, 0, 0, 1) + #111;\
            \n  grah: rgba(#f0e, $alpha: .5);\
            \n//  blah: rgba(1,2,3,.6);\
            \n  \
            \n  floo: $x;\
            \n//  bloo: rgba($x, 0.7);\
            \n  groo: $x;\
            \n  \
            \n  $x: rgb(123, 45, 6);\
            \n  \
            \n  hoo: red($x);\
            \n  moo: green($x);\
            \n  poo: blue($x);\
            \n  \
            \n//  goo: mix(rgba(255, 0, 0, 0.5), #00f);\
            \n  \
            \n  boo: invert(#123456);\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  color: yellow;\
        \n  background: #7b2d06;\
        \n  grah: rgba(255, 0, 238, 0.5);\
        \n  floo: aqua;\
        \n  groo: aqua;\
        \n  hoo: 123;\
        \n  moo: 45;\
        \n  poo: 6;\
        \n  boo: #edcba9;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/arglist.hrx"
#[test]
fn arglist() {
    assert_eq!(
        rsass(
            "@mixin foo($x, $y, $zs...) {\
            \n  foo-x: $x;\
            \n  foo-y: $y;\
            \n  foo-zs: $zs;\
            \n}\
            \n\
            \ndiv {\
            \n  @include foo(a, b, c, d, e);\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  foo-x: a;\
        \n  foo-y: b;\
        \n  foo-zs: c, d, e;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/backrefs-in-selector-groups.hrx"
#[test]
fn backrefs_in_selector_groups() {
    assert_eq!(
        rsass(
            "a {\
            \n  &:c, & d {\
            \n    hey: ho;\
            \n  }\
            \n}\
            \n\
            \na b {\
            \n  &:c, & d {\
            \n    hey: ho;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "a:c, a d {\
        \n  hey: ho;\
        \n}\
        \na b:c, a b d {\
        \n  hey: ho;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/backslash.hrx"
#[test]
fn backslash() {
    assert_eq!(
        rsass(
            "div, span {\
            \n\tcolor: red;\
            \n\t\\ foo {\
            \n\t\tcolor: blue;\
            \n\t}\
            \n}"
        )
        .unwrap(),
        "div, span {\
        \n  color: red;\
        \n}\
        \ndiv \\ foo, span \\ foo {\
        \n  color: blue;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/basic_function.hrx"
#[test]
fn basic_function() {
    assert_eq!(
        rsass(
            "@function foo() {\
            \n  @return 1 + 2;\
            \n}\
            \n\
            \nbar {\
            \n  a: foo();\
            \n}\
            \n"
        )
        .unwrap(),
        "bar {\
        \n  a: 3;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/basic_mixins.hrx"
#[test]
fn basic_mixins() {
    assert_eq!(
        rsass(
            "@mixin foo {a: b}\
            \n\
            \nbar {\
            \n  @include foo;\
            \n  c: d; }\
            \n"
        )
        .unwrap(),
        "bar {\
        \n  a: b;\
        \n  c: d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/basic_prop_name_interpolation.hrx"
#[test]
fn basic_prop_name_interpolation() {
    assert_eq!(
        rsass(
            "foo {bar#{1 + 2}: blip}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar3: blip;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/basic_selector_interpolation.hrx"
#[test]
fn basic_selector_interpolation() {
    assert_eq!(
        rsass(
            "#{\"foo\"}.bar baz {a: b}\
            \n"
        )
        .unwrap(),
        "foo.bar baz {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/blimp.hrx"
#[test]
fn blimp() {
    assert_eq!(
        rsass(
            "blimp { color: green }\
            \n"
        )
        .unwrap(),
        "blimp {\
        \n  color: green;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/block_comment_in_script.hrx"
#[test]
fn block_comment_in_script() {
    assert_eq!(
        rsass(
            "foo {a: 1 + /* flang */ bar}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: 1bar;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/cfunc.hrx"
#[test]
fn cfunc() {
    assert_eq!(
        rsass(
            "div {\
            \n  blah: say-something();\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  blah: say-something();\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/classes-and-ids.hrx"
#[test]
fn classes_and_ids() {
    assert_eq!(
        rsass(
            "div.foo {\
            \n  color: red;\
            \n  #hux buz {\
            \n    width: auto;\
            \n  }\
            \n  > .mux {\
            \n    text-align: center;\
            \n  }\
            \n}"
        )
        .unwrap(),
        "div.foo {\
        \n  color: red;\
        \n}\
        \ndiv.foo #hux buz {\
        \n  width: auto;\
        \n}\
        \ndiv.foo > .mux {\
        \n  text-align: center;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/color_output.hrx"
#[test]
fn color_output() {
    assert_eq!(
        rsass(
            "$green: green;\
            \n$green-hex: #00FF00;\
            \n$green-hex-min: #0f0;\
            \n$green-rgb: rgb(0, 255, 0);\
            \n$green-rgba-t: rgba(0, 255, 0, 0.5);\
            \n$green-rgba-s: rgba(0, 255, 0, 1);\
            \n$offgreen: #00ff01;\
            \n$silver: silver;\
            \n$silver-hex: #ddd;\
            \n\
            \na {\
            \n\tq: silver;\
            \n\tr: #ddd;\
            \n\ts: green;\
            \n\tt: #00FF00;\
            \n\tv: #0f0;\
            \n\tw: rgb(0, 255, 0);\
            \n\tx: rgba(0, 255, 0, 0.5);\
            \n\ty: rgba(0, 255, 0, 1);\
            \n\tz: #00ff01; }\
            \n\
            \nb {\
            \n\tq: 1px solid silver;\
            \n\tr: 1px solid #ddd;\
            \n\ts: 1px solid green;\
            \n\tt: 1px solid #00FF00;\
            \n\tv: 1px solid #0f0;\
            \n\tw: 1px solid rgb(0, 255, 0);\
            \n\tx: 1px solid rgba(0, 255, 0, 0.5);\
            \n\ty: 1px solid rgba(0, 255, 0, 1);\
            \n\tz: 1px solid #00ff01; }\
            \n\
            \nc {\
            \n\tq: $silver;\
            \n\tr: $silver-hex;\
            \n\ts: $green;\
            \n\tt: $green-hex;\
            \n\tv: $green-hex-min;\
            \n\tw: $green-rgb;\
            \n\tx: $green-rgba-t;\
            \n\ty: $green-rgba-s;\
            \n\tz: $offgreen; }\
            \n\
            \nd {\
            \n\tq: 1px solid $silver;\
            \n\tr: 1px solid $silver-hex;\
            \n\ts: 1px solid $green;\
            \n\tt: 1px solid $green-hex;\
            \n\tv: 1px solid $green-hex-min;\
            \n\tw: 1px solid $green-rgb;\
            \n\tx: 1px solid $green-rgba-t;\
            \n\ty: 1px solid $green-rgba-s;\
            \n\tz: 1px solid $offgreen; }\
            \n\
            \n"
        )
        .unwrap(),
        "a {\
        \n  q: silver;\
        \n  r: #ddd;\
        \n  s: green;\
        \n  t: #00FF00;\
        \n  v: #0f0;\
        \n  w: lime;\
        \n  x: rgba(0, 255, 0, 0.5);\
        \n  y: lime;\
        \n  z: #00ff01;\
        \n}\
        \nb {\
        \n  q: 1px solid silver;\
        \n  r: 1px solid #ddd;\
        \n  s: 1px solid green;\
        \n  t: 1px solid #00FF00;\
        \n  v: 1px solid #0f0;\
        \n  w: 1px solid lime;\
        \n  x: 1px solid rgba(0, 255, 0, 0.5);\
        \n  y: 1px solid lime;\
        \n  z: 1px solid #00ff01;\
        \n}\
        \nc {\
        \n  q: silver;\
        \n  r: #ddd;\
        \n  s: green;\
        \n  t: #00FF00;\
        \n  v: #0f0;\
        \n  w: lime;\
        \n  x: rgba(0, 255, 0, 0.5);\
        \n  y: lime;\
        \n  z: #00ff01;\
        \n}\
        \nd {\
        \n  q: 1px solid silver;\
        \n  r: 1px solid #ddd;\
        \n  s: 1px solid green;\
        \n  t: 1px solid #00FF00;\
        \n  v: 1px solid #0f0;\
        \n  w: 1px solid lime;\
        \n  x: 1px solid rgba(0, 255, 0, 0.5);\
        \n  y: 1px solid lime;\
        \n  z: 1px solid #00ff01;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/comment_after_if_directive.hrx"
#[test]
fn comment_after_if_directive() {
    assert_eq!(
        rsass(
            "foo {\
            \n  @if true {a: b}\
            \n  @else {x: y}\
            \n  /* This is a comment */\
            \n  c: d }\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: b;\
        \n  /* This is a comment */\
        \n  c: d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/composed-args.hrx"
#[test]
#[ignore] // wrong result
fn composed_args() {
    assert_eq!(
        rsass(
            "@mixin A($width: 0, $height: 0, $opacity: 0) {\
            \n  width: $width;\
            \n  height: $height;\
            \n  opacity: $opacity;\
            \n}\
            \n\
            \n@mixin B($args...) {\
            \n  @include A($args...);\
            \n}\
            \n\
            \n@mixin C($args...) {\
            \n  @include B($args...);\
            \n}\
            \n\
            \n.testOneLevelPassthrough {\
            \n  @include B(1px, 2px, 0.3);\
            \n}\
            \n\
            \n.testOneLevelNoArgs {\
            \n  @include B();\
            \n}\
            \n\
            \n.testOneLevelSingleArg {\
            \n  @include B(1px);\
            \n}\
            \n\
            \n.testOneLevelNamedSingleArg {\
            \n  @include B($opacity: 0.1);\
            \n}\
            \n\
            \n.testOneLevelNamedArgs {\
            \n  @include B($opacity: 0.3, $width: 1px, $height: 2px);\
            \n}\
            \n\
            \n.testTwoLevelPassthrough {\
            \n  @include C(1px, 2px, 0.3);\
            \n}\
            \n\
            \n.testTwoLevelNoArgs {\
            \n  @include C();\
            \n}\
            \n\
            \n.testTwoLevelSingleArg {\
            \n  @include C(1px);\
            \n}\
            \n\
            \n.testTwoLevelNamedSingleArg {\
            \n  @include C($opacity: 0.1);\
            \n}\
            \n\
            \n.testTwoLevelNamedArgs {\
            \n  @include C($opacity: 0.3, $width: 1px, $height: 2px);\
            \n}\
            \n"
        )
        .unwrap(),
        ".testOneLevelPassthrough {\
        \n  width: 1px;\
        \n  height: 2px;\
        \n  opacity: 0.3;\
        \n}\
        \n.testOneLevelNoArgs {\
        \n  width: 0;\
        \n  height: 0;\
        \n  opacity: 0;\
        \n}\
        \n.testOneLevelSingleArg {\
        \n  width: 1px;\
        \n  height: 0;\
        \n  opacity: 0;\
        \n}\
        \n.testOneLevelNamedSingleArg {\
        \n  width: 0;\
        \n  height: 0;\
        \n  opacity: 0.1;\
        \n}\
        \n.testOneLevelNamedArgs {\
        \n  width: 1px;\
        \n  height: 2px;\
        \n  opacity: 0.3;\
        \n}\
        \n.testTwoLevelPassthrough {\
        \n  width: 1px;\
        \n  height: 2px;\
        \n  opacity: 0.3;\
        \n}\
        \n.testTwoLevelNoArgs {\
        \n  width: 0;\
        \n  height: 0;\
        \n  opacity: 0;\
        \n}\
        \n.testTwoLevelSingleArg {\
        \n  width: 1px;\
        \n  height: 0;\
        \n  opacity: 0;\
        \n}\
        \n.testTwoLevelNamedSingleArg {\
        \n  width: 0;\
        \n  height: 0;\
        \n  opacity: 0.1;\
        \n}\
        \n.testTwoLevelNamedArgs {\
        \n  width: 1px;\
        \n  height: 2px;\
        \n  opacity: 0.3;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/concat.hrx"
#[test]
fn concat() {
    assert_eq!(
        rsass(
            "div {\
            \n  a: hello + \"goodbye\";\
            \n  b: \"hello\" + goodbye;\
            \n  c: 3 + \"hello\";\
            \n  d: \"hello\" + 3;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  a: hellogoodbye;\
        \n  b: \"hellogoodbye\";\
        \n  c: \"3hello\";\
        \n  d: \"hello3\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/cons-up.hrx"
#[test]
fn cons_up() {
    assert_eq!(
        rsass(
            "$inputs-list: \'input[type=\"email\"]\',\
            \n              \'input[type=\"number\"]\',\
            \n              \'input[type=\"password\"]\',\
            \n              \'input[type=\"search\"]\',\
            \n              \'input[type=\"tel\"]\',\
            \n              \'input[type=\"text\"]\',\
            \n              \'input[type=\"url\"]\',\
            \n\
            \n              // Webkit & Gecko may change the display of these in the future\
            \n              \'input[type=\"color\"]\',\
            \n              \'input[type=\"date\"]\',\
            \n              \'input[type=\"datetime\"]\',\
            \n              \'input[type=\"datetime-local\"]\',\
            \n              \'input[type=\"month\"]\',\
            \n              \'input[type=\"time\"]\',\
            \n              \'input[type=\"week\"]\';\
            \n\
            \n$unquoted-inputs-list: ();\
            \n\
            \n@each $input-type in $inputs-list {\
            \n  $unquoted-inputs-list: append($unquoted-inputs-list, unquote($input-type), comma);\
            \n}\
            \n\
            \ndiv {\
            \n  content: $unquoted-inputs-list;\
            \n  content: append((), hello);\
            \n  content: length(());\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  content: input[type=\"email\"], input[type=\"number\"], input[type=\"password\"], input[type=\"search\"], input[type=\"tel\"], input[type=\"text\"], input[type=\"url\"], input[type=\"color\"], input[type=\"date\"], input[type=\"datetime\"], input[type=\"datetime-local\"], input[type=\"month\"], input[type=\"time\"], input[type=\"week\"];\
        \n  content: hello;\
        \n  content: 0;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/css_basic_scss.hrx"
#[test]
fn css_basic_scss() {
    assert_eq!(
        rsass(
            "sel {\
            \n  p: v; }\
            \n"
        )
        .unwrap(),
        "sel {\
        \n  p: v;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/css_block_directive_with_semicolon.hrx"
#[test]
fn css_block_directive_with_semicolon() {
    assert_eq!(
        rsass(
            "@foo {\
            \n  a: b; }\
            \n\
            \n@bar {\
            \n  a: b; }\
            \n"
        )
        .unwrap(),
        "@foo {\
        \n  a: b;\
        \n}\
        \n@bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/css_cdo_and_cdc_ignored_at_toplevel.hrx"
#[test]
fn css_cdo_and_cdc_ignored_at_toplevel() {
    assert_eq!(
        rsass(
            "foo {\
            \n  bar: baz; }\
            \n\
            \nbar {\
            \n  bar: baz; }\
            \n\
            \nbaz {\
            \n  bar: baz; }\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: baz;\
        \n}\
        \nbar {\
        \n  bar: baz;\
        \n}\
        \nbaz {\
        \n  bar: baz;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/css_crazy_comments.hrx"
#[test]
fn css_crazy_comments() {
    assert_eq!(
        rsass(
            "/* This is a CSS comment. */\
            \n.one {\
            \n  color: green; }\
            \n\
            \n/* Another comment */\
            \n/* The following should not be used:\
            \n.two {color: red;} */\
            \n.three {\
            \n  color: green;\
            \n  /* color: red; */ }\
            \n\
            \n/**\
            \n.four {color: red;} */\
            \n.five {\
            \n  color: green; }\
            \n\
            \n/**/\
            \n.six {\
            \n  color: green; }\
            \n\
            \n/*********/\
            \n.seven {\
            \n  color: green; }\
            \n\
            \n/* a comment **/\
            \n.eight {\
            \n  color: green; }\
            \n"
        )
        .unwrap(),
        "/* This is a CSS comment. */\
        \n.one {\
        \n  color: green;\
        \n}\
        \n/* Another comment */\
        \n/* The following should not be used:\
        \n.two {color: red;} */\
        \n.three {\
        \n  color: green;\
        \n  /* color: red; */\
        \n}\
        \n/**\
        \n.four {color: red;} */\
        \n.five {\
        \n  color: green;\
        \n}\
        \n/**/\
        \n.six {\
        \n  color: green;\
        \n}\
        \n/*********/\
        \n.seven {\
        \n  color: green;\
        \n}\
        \n/* a comment **/\
        \n.eight {\
        \n  color: green;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/css_empty_declarations.hrx"
#[test]
fn css_empty_declarations() {
    assert_eq!(
        rsass(
            "foo {\
            \n  bar: baz; }\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: baz;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/css_empty_rule.hrx"
#[test]
fn css_empty_rule() {
    assert_eq!(rsass("").unwrap(), "");
}

// From "sass-spec/spec/non_conformant/scss/css_import_directive.hrx"
#[test]
fn css_import_directive() {
    assert_eq!(
        rsass("@import url(foo.css);").unwrap(),
        "@import url(foo.css);\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/css_invisible_comments.hrx"
#[test]
fn css_invisible_comments() {
    assert_eq!(
        rsass(
            "foo {\
            \n  a: d; }\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/css_property_comments.hrx"
#[test]
fn css_property_comments() {
    assert_eq!(
        rsass(
            ".foo {\
            \n  /* Foo\
            \n   * Bar */\
            \n  a: b; }\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  /* Foo\
        \n   * Bar */\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/css_rule_comments.hrx"
#[test]
fn css_rule_comments() {
    assert_eq!(
        rsass(
            "/* Foo\
            \n * Bar */\
            \n.foo {\
            \n  a: b; }\
            \n"
        )
        .unwrap(),
        "/* Foo\
        \n * Bar */\
        \n.foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/css_selector_comments.hrx"
#[test]
fn css_selector_comments() {
    assert_eq!(
        rsass(
            ".foo #bar:baz(bip) {\
            \n  a: b; }\
            \n"
        )
        .unwrap(),
        ".foo #bar:baz(bip) {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/css_selector_hacks.hrx"
#[test]
fn css_selector_hacks() {
    assert_eq!(
        rsass(
            "> > E {\
            \n  a: b; }\
            \n"
        )
        .unwrap(),
        "> > E {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/css_spaceless_combo_selectors.hrx"
#[test]
fn css_spaceless_combo_selectors() {
    assert_eq!(
        rsass(
            "E + F {\
            \n  a: b; }\
            \n"
        )
        .unwrap(),
        "E + F {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/css_unary_ops.hrx"
#[test]
fn css_unary_ops() {
    assert_eq!(
        rsass(
            "foo {\
            \n  a: -0.5em;\
            \n  b: 0.5em;\
            \n  c: -foo(12px);\
            \n  d: +foo(12px); }\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: -0.5em;\
        \n  b: 0.5em;\
        \n  c: -foo(12px);\
        \n  d: +foo(12px);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/default-args.hrx"
#[test]
fn default_args() {
    assert_eq!(
        rsass(
            "@mixin foo($x: 1, $y: $x + 1) {\
            \n  value: $x, $y;\
            \n}\
            \n\
            \ndiv {\
            \n  @include foo();\
            \n  @include foo(2);\
            \n  @include foo($y: 3);\
            \n}\
            \n\
            \n$v: hey;\
            \n\
            \n@mixin bar($x: $v) {\
            \n  value: $x;\
            \n}\
            \n\
            \ndiv {\
            \n  $v: ho !global;\
            \n  @include bar();\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  value: 1, 2;\
        \n  value: 2, 3;\
        \n  value: 1, 3;\
        \n}\
        \ndiv {\
        \n  value: ho;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/default-parameters.hrx"
#[test]
fn default_parameters() {
    assert_eq!(
        rsass(
            "$a: red;\
            \n\
            \n@mixin f($a: $a) {\
            \n  color: $a;\
            \n}\
            \n\
            \nh1 {\
            \n  @include f;\
            \n}\
            \n\
            \nh2 {\
            \n  @include f(blue);\
            \n}"
        )
        .unwrap(),
        "h1 {\
        \n  color: red;\
        \n}\
        \nh2 {\
        \n  color: blue;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/default-vars-in-default-params.hrx"
#[test]
fn default_vars_in_default_params() {
    assert_eq!(
        rsass(
            "$y: why;\
            \n\
            \n@mixin foo($x, $y: $y) {\
            \n  stuff: $x $y;\
            \n}\
            \n\
            \ndiv {\
            \n  why: $y;\
            \n  @include foo(ecks);\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  why: why;\
        \n  stuff: ecks why;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/directives-in-propsets.hrx"
#[test]
fn directives_in_propsets() {
    assert_eq!(
        rsass(
            "$color: red;\
            \n$position: 50%;\
            \n$x: 0;\
            \n\
            \n@mixin foo() {\
            \n  image: url(foo.png);\
            \n}\
            \n\
            \ndiv {\
            \n  background: {\
            \n    something: {\
            \n      color: green;\
            \n    }\
            \n    @if (type-of($color) == \"color\") {\
            \n      color: $color;\
            \n    }\
            \n    @if (type-of($position) == \"number\") {\
            \n      position: $position;\
            \n      @include foo();\
            \n    }\
            \n    groo: foo;\
            \n  }\
            \n  width: $x;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  background-something-color: green;\
        \n  background-color: red;\
        \n  background-position: 50%;\
        \n  background-image: url(foo.png);\
        \n  background-groo: foo;\
        \n  width: 0;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/each.hrx"
#[test]
fn each() {
    assert_eq!(
        rsass(
            "div {\
            \n  $x: 3;\
            \n  @each $x in singleton {\
            \n    color: $x;\
            \n  }\
            \n  divider: $x;\
            \n  @each $x in foo, bar, hux {\
            \n    span {\
            \n      msg: $x;\
            \n    }\
            \n  }\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  color: singleton;\
        \n  divider: 3;\
        \n}\
        \ndiv span {\
        \n  msg: foo;\
        \n}\
        \ndiv span {\
        \n  msg: bar;\
        \n}\
        \ndiv span {\
        \n  msg: hux;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/each_directive.hrx"
#[test]
fn each_directive() {
    assert_eq!(
        rsass(
            "a {\
            \n  @each $number in 1px 2px 3px 4px {\
            \n    b: $number;\
            \n  }\
            \n}\
            \nc {\
            \n  @each $str in foo, bar, baz, bang {\
            \n    d: $str;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1px;\
        \n  b: 2px;\
        \n  b: 3px;\
        \n  b: 4px;\
        \n}\
        \nc {\
        \n  d: foo;\
        \n  d: bar;\
        \n  d: baz;\
        \n  d: bang;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/each_in_functions.hrx"
#[test]
fn each_in_functions() {
    assert_eq!(
        rsass(
            "$GLOBAL: global;\
            \n\
            \n@function foo($g1, $g2, $g3) {\
            \n  @each $value in $g1, $g2, $g3 {\
            \n    $GLOBAL: $GLOBAL each $value !global;\
            \n    $GLOBAL: $GLOBAL type1 type-of(nth($value, 1)) !global;\
            \n    $GLOBAL: $GLOBAL type2 type-of(nth($value, 2)) !global;\
            \n  }\
            \n  @each $value in (foo: foo, bar: bar) {\
            \n    $GLOBAL: $GLOBAL map $value !global;\
            \n  }\
            \n  @return 0;\
            \n}\
            \n\
            \ndiv {\
            \n  a: foo(50% 50%, cover circle, red blue);\
            \n  b: $GLOBAL;\
            \n  $colors: red green blue;\
            \n  c: a, b, type-of(nth($colors, 2)), d;\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  a: 0;\
        \n  b: global each 50% 50% type1 number type2 number each cover circle type1 string type2 string each red blue type1 color type2 color map foo foo map bar bar;\
        \n  c: a, b, color, d;\
        \n}\
        \n"
    );
}

mod feature_queries;

// From "sass-spec/spec/non_conformant/scss/for.hrx"
#[test]
fn test_for() {
    assert_eq!(
        rsass(
            "$limit: 10;\
            \n\
            \n@for $x from 1 through $limit {\
            \n  $limit: 4;\
            \n  div {\
            \n    content: $limit thing $x;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  content: 4 thing 1;\
        \n}\
        \ndiv {\
        \n  content: 4 thing 2;\
        \n}\
        \ndiv {\
        \n  content: 4 thing 3;\
        \n}\
        \ndiv {\
        \n  content: 4 thing 4;\
        \n}\
        \ndiv {\
        \n  content: 4 thing 5;\
        \n}\
        \ndiv {\
        \n  content: 4 thing 6;\
        \n}\
        \ndiv {\
        \n  content: 4 thing 7;\
        \n}\
        \ndiv {\
        \n  content: 4 thing 8;\
        \n}\
        \ndiv {\
        \n  content: 4 thing 9;\
        \n}\
        \ndiv {\
        \n  content: 4 thing 10;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/for_directive.hrx"
#[test]
fn for_directive() {
    assert_eq!(
        rsass(
            ".foo {\
            \n  @for $var from 1 through 5 {a: $var;}\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: 1;\
        \n  a: 2;\
        \n  a: 3;\
        \n  a: 4;\
        \n  a: 5;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/for_in_functions.hrx"
#[test]
fn for_in_functions() {
    assert_eq!(
        rsass(
            "@function foo() {\
            \n\t$limit: 10;\
            \n\t$y: 0;\
            \n\t@for $x from 1 through $limit {\
            \n\t  $limit: 4;\
            \n\t  $y: $y + $x;\
            \n\t}\
            \n\t@return $y;\
            \n}\
            \n\
            \ndiv {\
            \n\twidth: foo();\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  width: 55;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/function-names.hrx"
#[test]
fn function_names() {
    assert_eq!(
        rsass(
            "div {\
            \n  color: unquote(\"hello\");\
            \n  color: un#{quo}te(\"hello\");\
            \n  color: (\"hello\")un#{quo}te;\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  color: hello;\
        \n  color: unquote(\"hello\");\
        \n  color: \"hello\" unquote;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/function_args.hrx"
#[test]
fn function_args() {
    assert_eq!(
        rsass(
            "@function plus($var1, $var2) {\
            \n  @return $var1 + $var2;\
            \n}\
            \n\
            \nbar {\
            \n  a: plus(1, 2);\
            \n}\
            \n"
        )
        .unwrap(),
        "bar {\
        \n  a: 3;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/functions-and-mixins.hrx"
#[test]
fn functions_and_mixins() {
    assert_eq!(
        rsass(
            "@function foo() {\
            \n  @return \"hello\";\
            \n}\
            \n\
            \n@mixin foo() {\
            \n  content: \"hello\";\
            \n}\
            \n\
            \ndiv {\
            \n  span {\
            \n    @function length($a, $b, $c, $d) {\
            \n      @return $a + $b + $c + $d;\
            \n    }\
            \n\
            \n    div {\
            \n      content: foo();\
            \n      @include foo();\
            \n      width: length(1,2,2,3);\
            \n    }\
            \n  }\
            \n\
            \n  height: length(a b c d e);\
            \n\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  height: 5;\
        \n}\
        \ndiv span div {\
        \n  content: \"hello\";\
        \n  content: \"hello\";\
        \n  width: 8;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/functions.hrx"
#[test]
fn functions() {
    assert_eq!(
        rsass(
            "@function foo($x, $y, $z) {\
            \n  @while $x < $y {\
            \n    $z: transform($z);\
            \n    @return $z;\
            \n  }\
            \n}\
            \n\
            \n@function bar($x) {\
            \n  @if $x {\
            \n    @return YES;\
            \n  }\
            \n}\
            \n\
            \ndiv {\
            \n  answer: bar(true);\
            \n  flanswer: fudge(mux+flux) + mudge(a/b);\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  answer: YES;\
        \n  flanswer: fudge(muxflux)mudge(a/b);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/guard_assign.hrx"
#[test]
fn guard_assign() {
    assert_eq!(
        rsass(
            "$var: 2 !default;\
            \n\
            \nfoo {a: $var}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: 2;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/hey1.hrx"
#[test]
fn hey1() {
    assert_eq!(
        rsass(
            "div { width: 1px; }\
            \n"
        )
        .unwrap(),
        "div {\
        \n  width: 1px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/hey2.hrx"
#[test]
fn hey2() {
    assert_eq!(
        rsass(
            "div { color: red; }\
            \n"
        )
        .unwrap(),
        "div {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// Ignoring "huge.hrx", not expected to work yet.

// From "sass-spec/spec/non_conformant/scss/hyphen-interpolated.hrx"
#[test]
fn hyphen_interpolated() {
    assert_eq!(
        rsass(
            "div {\
            \n  foo: -hux-#{2+3};\
            \n  bar: hux-#{2+3};\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  foo: -hux-5;\
        \n  bar: hux-5;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/ie-backslash.hrx"
#[test]
fn ie_backslash() {
    assert_eq!(
        rsass(
            "div {\
            \n  background-color: darken(red, 10%) \\9;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  background-color: #cc0000 \\9 ;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/ie-functions.hrx"
#[test]
#[ignore] // wrong result
fn ie_functions() {
    assert_eq!(
        rsass(
            "@mixin ie-opacity($opacity) {\
            \n  opacity: $opacity / 100;\
            \n  filter: alpha(opacity=$opacity);\
            \n  bilter: alpha(opacity=$opacity);\
            \n  kilter: type-of(opacity=$opacity);\
            \n  left: expression(document.body.clientWidth/2-oDiv.offsetWidth/2);\
            \n  flop: expression(document.body.clientHeight/2-oDiv.offsetHeight/2);\
            \n}\
            \n\
            \n$startColor: red;\
            \n$endColor: green;\
            \n\
            \nfoo {\
            \n  filter: progid:Microsoft.foo.bar.Baz(flip=#{foo + bar}, bang=#00ff00cc);\
            \n  something: blah(hux = mumble);\
            \n  blah: progid:something.something(flip=foobar, bang=#abc);\
            \n  blah: progid:bar.hux();\
            \n  blah: type-of(hux = mumble);\
            \n  @include ie-opacity(.5);\
            \n  left: expression(document.body.clientWidth/4);\
            \n  filter: progid:DXImageTransform.Microsoft.gradient(startColorstr=\'#{ie-hex-str($startColor)}\', endColorstr=\'#{ie-hex-str($endColor)}\', GradientType=1);\
            \n}\
            \n\
            \n.parser {\
            \n    filter: progid:DXImageTransform.Microsoft.Alpha(opacity=20);\
            \n    filter: progid:DXImageTransform.Microsoft.MotionBlur(strength=50)\
            \n            progid:DXImageTransform.Microsoft.BasicImage(rotation=2, mirror=1);\
            \n    filter: progid:DXImageTransform.Microsoft.gradient(startColorstr=#550000FF, endColorstr=#55FFFF00);\
            \n    filter: progid:DXImageTransform.Microsoft.BasicImage(rotation=2, mirror=1)\
            \n            progid:DXImageTransform.Microsoft.Alpha(opacity=50)\
            \n            progid:DXImageTransform.Microsoft.Blur(strength=10);\
            \n    filter: progid:DXImageTransform.Microsoft.Wave(strength=100)\
            \n            progid:DXImageTransform.Microsoft.CheckerBoard(duration=4);\
            \n    filter: progid:DXImageTransform.Microsoft.Wave(strength=100)\
            \n            progid:DXImageTransform.Microsoft.BasicImage(rotation=2, mirror=1)\
            \n            progid:DXImageTransform.Microsoft.Iris(irisstyle=\'STAR\', duration=4);\
            \n    filter: progid:DXImageTransform.Microsoft.MotionBlur(strength=13, direction=310)\
            \n            progid:DXImageTransform.Microsoft.Blur(pixelradius=2)\
            \n            progid:DXImageTransform.Microsoft.Wheel(duration=3);\
            \n    filter: progid:DXImageTransform.Microsoft.gradient(enabled=\'false\',\
            \n            startColorstr=#550000FF, endColorstr=#55FFFF00);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  filter: progid:Microsoft.foo.bar.Baz(flip=foobar, bang=#00ff00cc);\
        \n  something: blah(hux=mumble);\
        \n  blah: progid:something.something(flip=foobar, bang=#abc);\
        \n  blah: progid:bar.hux();\
        \n  blah: string;\
        \n  opacity: 0.005;\
        \n  filter: alpha(opacity=0.5);\
        \n  bilter: alpha(opacity=0.5);\
        \n  kilter: string;\
        \n  left: expression(document.body.clientWidth/2-oDiv.offsetWidth/2);\
        \n  flop: expression(document.body.clientHeight/2-oDiv.offsetHeight/2);\
        \n  left: expression(document.body.clientWidth/4);\
        \n  filter: progid:DXImageTransform.Microsoft.gradient(startColorstr=\"#FFFF0000\", endColorstr=\"#FF008000\", GradientType=1);\
        \n}\
        \n.parser {\
        \n  filter: progid:DXImageTransform.Microsoft.Alpha(opacity=20);\
        \n  filter: progid:DXImageTransform.Microsoft.MotionBlur(strength=50) progid:DXImageTransform.Microsoft.BasicImage(rotation=2, mirror=1);\
        \n  filter: progid:DXImageTransform.Microsoft.gradient(startColorstr=#550000FF, endColorstr=#55FFFF00);\
        \n  filter: progid:DXImageTransform.Microsoft.BasicImage(rotation=2, mirror=1) progid:DXImageTransform.Microsoft.Alpha(opacity=50) progid:DXImageTransform.Microsoft.Blur(strength=10);\
        \n  filter: progid:DXImageTransform.Microsoft.Wave(strength=100) progid:DXImageTransform.Microsoft.CheckerBoard(duration=4);\
        \n  filter: progid:DXImageTransform.Microsoft.Wave(strength=100) progid:DXImageTransform.Microsoft.BasicImage(rotation=2, mirror=1) progid:DXImageTransform.Microsoft.Iris(irisstyle=\"STAR\", duration=4);\
        \n  filter: progid:DXImageTransform.Microsoft.MotionBlur(strength=13, direction=310) progid:DXImageTransform.Microsoft.Blur(pixelradius=2) progid:DXImageTransform.Microsoft.Wheel(duration=3);\
        \n  filter: progid:DXImageTransform.Microsoft.gradient(enabled=\"false\", startColorstr=#550000FF, endColorstr=#55FFFF00);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/if-in-mixin.hrx"
#[test]
fn if_in_mixin() {
    assert_eq!(
        rsass(
            "$x: true;\
            \n\
            \n@mixin foobar() {\
            \n  @if $x {\
            \n    $x: false !global;\
            \n    content: foo;\
            \n  }\
            \n  @else {\
            \n    $x: true !global;\
            \n    content: bar;\
            \n  }\
            \n}\
            \n\
            \ndiv {\
            \n  @include foobar();\
            \n  @include foobar();\
            \n  @include foobar();\
            \n  $x: true !global;\
            \n  @include foobar();\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  content: foo;\
        \n  content: bar;\
        \n  content: foo;\
        \n  content: foo;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/if-in-propset.hrx"
#[test]
fn if_in_propset() {
    assert_eq!(
        rsass(
            "div {\
            \n  prop: {\
            \n    a: \"hello\";\
            \n    b: \"goodbye\";\
            \n    @if true {\
            \n      c: \"badbye\";\
            \n    }\
            \n  }\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  prop-a: \"hello\";\
        \n  prop-b: \"goodbye\";\
        \n  prop-c: \"badbye\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/if.hrx"
#[test]
fn test_if() {
    assert_eq!(
        rsass(
            "@if false {\
            \n  div {\
            \n    color: red;\
            \n  }\
            \n}\
            \n@else if true {\
            \n  span {\
            \n    color: blue;\
            \n  }\
            \n}\
            \n\
            \ndiv {\
            \n  @if true {\
            \n    color: green;\
            \n  }\
            \n  @if false {\
            \n    height: 10px;\
            \n  }\
            \n  @else if false {\
            \n    height: 20px;\
            \n  }\
            \n  @else if false {\
            \n    height: 30px;\
            \n  }\
            \n  @else {\
            \n    height: 40px;\
            \n  }\
            \n}"
        )
        .unwrap(),
        "span {\
        \n  color: blue;\
        \n}\
        \ndiv {\
        \n  color: green;\
        \n  height: 40px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/if_directive.hrx"
#[test]
fn if_directive() {
    assert_eq!(
        rsass(
            "@if \"foo\" != \"foo\" {foo {a: b}}\
            \n@else {bar {a: b}}\
            \n"
        )
        .unwrap(),
        "bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/imp.hrx"
#[test]
fn imp() {
    assert_eq!(
        rsass(
            "div { color: red; }\
            \n"
        )
        .unwrap(),
        "div {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/important-in-arglist.hrx"
#[test]
fn important_in_arglist() {
    assert_eq!(
        rsass(
            "@mixin foo($x) {\
            \n  style: $x;\
            \n}\
            \n\
            \ndiv {\
            \n  @include foo(0px 0px 0px 0px #ef8086 inset !important);\
            \n  fludge: foo bar ! important hux;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  style: 0px 0px 0px 0px #ef8086 inset !important;\
        \n  fludge: foo bar !important hux;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/important.hrx"
#[test]
fn important() {
    assert_eq!(
        rsass(
            "div {\
            \n  color: red ! important;\
            \n  width: 5px ! important;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  color: red !important;\
        \n  width: 5px !important;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/interpolated-selectors.hrx"
#[test]
fn interpolated_selectors() {
    assert_eq!(
        rsass(
            "foo#{bar} hux {\
            \n  color: red;\
            \n}"
        )
        .unwrap(),
        "foobar hux {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/interpolated-strings.hrx"
#[test]
fn interpolated_strings() {
    assert_eq!(
        rsass(
            "$x: ecks;\
            \n$y: why;\
            \n\
            \ndiv {\
            \n  blah: \"hey #{$x} ho\";\
            \n  blee: hey#{$y}ho;\
            \n  bluh: \"foo #{$x}\";\
            \n  bleg: foo#{\"hey\"}bar;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  blah: \"hey ecks ho\";\
        \n  blee: heywhyho;\
        \n  bluh: \"foo ecks\";\
        \n  bleg: fooheybar;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/interpolation-operators-precedence.hrx"

// Ignoring "interpolation_operators_precedence", error tests are not supported yet.

// From "sass-spec/spec/non_conformant/scss/interpolation.hrx"
#[test]
fn interpolation() {
    assert_eq!(
        rsass(
            "$bar : \"#foo\";\
            \n\
            \n\
            \n\
            \nul li#{$bar} a span.label { foo: bar; }\
            \n"
        )
        .unwrap(),
        "ul li#foo a span.label {\
        \n  foo: bar;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/interpolation_with_bracket_on_next_line.hrx"
#[test]
fn interpolation_with_bracket_on_next_line() {
    assert_eq!(
        rsass(
            "a.#{\"foo\"} b\
            \n{color: red}\
            \n"
        )
        .unwrap(),
        "a.foo b {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/keyword_args_in_functions.hrx"
#[test]
fn keyword_args_in_functions() {
    assert_eq!(
        rsass(
            ".keyed { color: rgba($color: #a7c, $alpha: 0.4) }\
            \n"
        )
        .unwrap(),
        ".keyed {\
        \n  color: rgba(170, 119, 204, 0.4);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/lang.hrx"
#[test]
fn lang() {
    assert_eq!(
        rsass(
            "h1:lang(as),h1:lang(bn),h1:lang(gu),h1:lang(hi),h1:lang(kn),h1:lang(ml),h1:lang(mr),h1:lang(or),h1:lang(pa),h1:lang(sa),h1:lang(ta),h1:lang(te) {\
            \n  line-height:1.5em !important\
            \n}\
            \nh2:lang(as),h3:lang(as),h4:lang(as),h5:lang(as),h6:lang(as),h2:lang(bn),h3:lang(bn),h4:lang(bn),h5:lang(bn),h6:lang(bn),h2:lang(gu),h3:lang(gu),h4:lang(gu),h5:lang(gu),h6:lang(gu),h2:lang(hi),h3:lang(hi),h4:lang(hi),h5:lang(hi),h6:lang(hi),h2:lang(kn),h3:lang(kn),h4:lang(kn),h5:lang(kn),h6:lang(kn),h2:lang(ml),h3:lang(ml),h4:lang(ml),h5:lang(ml),h6:lang(ml),h2:lang(mr),h3:lang(mr),h4:lang(mr),h5:lang(mr),h6:lang(mr),h2:lang(or),h3:lang(or),h4:lang(or),h5:lang(or),h6:lang(or),h2:lang(pa),h3:lang(pa),h4:lang(pa),h5:lang(pa),h6:lang(pa),h2:lang(sa),h3:lang(sa),h4:lang(sa),h5:lang(sa),h6:lang(sa),h2:lang(ta),h3:lang(ta),h4:lang(ta),h5:lang(ta),h6:lang(ta),h2:lang(te),h3:lang(te),h4:lang(te),h5:lang(te),h6:lang(te)\
            \n{\
            \n  line-height:1.2em\
            \n}\
            \nol:lang(bcc) li,ol:lang(bqi) li,ol:lang(fa) li,ol:lang(glk) li,ol:lang(kk-arab) li,ol:lang(mzn) li {\
            \n  list-style-type:-moz-persian;list-style-type:persian\
            \n}\
            \nol:lang(ckb) li {\
            \n  list-style-type:-moz-arabic-indic;list-style-type:arabic-indic\
            \n}\
            \nol:lang(as) li,ol:lang(bn) li{\
            \n  list-style-type:-moz-bengali;list-style-type:bengali\
            \n}\
            \nol:lang(or) li {\
            \n  list-style-type:-moz-oriya;list-style-type:oriya\
            \n}"
        )
        .unwrap(),
        "h1:lang(as), h1:lang(bn), h1:lang(gu), h1:lang(hi), h1:lang(kn), h1:lang(ml), h1:lang(mr), h1:lang(or), h1:lang(pa), h1:lang(sa), h1:lang(ta), h1:lang(te) {\
        \n  line-height: 1.5em !important;\
        \n}\
        \nh2:lang(as), h3:lang(as), h4:lang(as), h5:lang(as), h6:lang(as), h2:lang(bn), h3:lang(bn), h4:lang(bn), h5:lang(bn), h6:lang(bn), h2:lang(gu), h3:lang(gu), h4:lang(gu), h5:lang(gu), h6:lang(gu), h2:lang(hi), h3:lang(hi), h4:lang(hi), h5:lang(hi), h6:lang(hi), h2:lang(kn), h3:lang(kn), h4:lang(kn), h5:lang(kn), h6:lang(kn), h2:lang(ml), h3:lang(ml), h4:lang(ml), h5:lang(ml), h6:lang(ml), h2:lang(mr), h3:lang(mr), h4:lang(mr), h5:lang(mr), h6:lang(mr), h2:lang(or), h3:lang(or), h4:lang(or), h5:lang(or), h6:lang(or), h2:lang(pa), h3:lang(pa), h4:lang(pa), h5:lang(pa), h6:lang(pa), h2:lang(sa), h3:lang(sa), h4:lang(sa), h5:lang(sa), h6:lang(sa), h2:lang(ta), h3:lang(ta), h4:lang(ta), h5:lang(ta), h6:lang(ta), h2:lang(te), h3:lang(te), h4:lang(te), h5:lang(te), h6:lang(te) {\
        \n  line-height: 1.2em;\
        \n}\
        \nol:lang(bcc) li, ol:lang(bqi) li, ol:lang(fa) li, ol:lang(glk) li, ol:lang(kk-arab) li, ol:lang(mzn) li {\
        \n  list-style-type: -moz-persian;\
        \n  list-style-type: persian;\
        \n}\
        \nol:lang(ckb) li {\
        \n  list-style-type: -moz-arabic-indic;\
        \n  list-style-type: arabic-indic;\
        \n}\
        \nol:lang(as) li, ol:lang(bn) li {\
        \n  list-style-type: -moz-bengali;\
        \n  list-style-type: bengali;\
        \n}\
        \nol:lang(or) li {\
        \n  list-style-type: -moz-oriya;\
        \n  list-style-type: oriya;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/line_comment_in_script.hrx"
#[test]
fn line_comment_in_script() {
    assert_eq!(
        rsass(
            "foo {a: 1 + // flang }\
            \n  blang }\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: 1blang;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/long-selector.hrx"
#[test]
fn long_selector() {
    assert_eq!(
        rsass(
            "html, body, div, span, iframe, h1, h2, h3, h4, h5, h6, p, blockquote, pre, a, ab, br, address, cite, code, del, dfn, em, img, ins, kbd, q, samp, small, strong, su, b, sup, var, b, u, i, dl, dt, dd, ol, ul, li, fieldset, form, label, legend, tab, le, caption, tbody, tfoot, thead, tr, th, td {\
            \n  border: 0;\
            \n  font-size: 100%;\
            \n  font: inherit;\
            \n  margin: 0;\
            \n  padding: 0;\
            \n  vertical-align: baseline;\
            \n  hey, ho, hoo {\
            \n    blah: bloo;\
            \n    blee: bleh;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "html, body, div, span, iframe, h1, h2, h3, h4, h5, h6, p, blockquote, pre, a, ab, br, address, cite, code, del, dfn, em, img, ins, kbd, q, samp, small, strong, su, b, sup, var, b, u, i, dl, dt, dd, ol, ul, li, fieldset, form, label, legend, tab, le, caption, tbody, tfoot, thead, tr, th, td {\
        \n  border: 0;\
        \n  font-size: 100%;\
        \n  font: inherit;\
        \n  margin: 0;\
        \n  padding: 0;\
        \n  vertical-align: baseline;\
        \n}\
        \nhtml hey, html ho, html hoo, body hey, body ho, body hoo, div hey, div ho, div hoo, span hey, span ho, span hoo, iframe hey, iframe ho, iframe hoo, h1 hey, h1 ho, h1 hoo, h2 hey, h2 ho, h2 hoo, h3 hey, h3 ho, h3 hoo, h4 hey, h4 ho, h4 hoo, h5 hey, h5 ho, h5 hoo, h6 hey, h6 ho, h6 hoo, p hey, p ho, p hoo, blockquote hey, blockquote ho, blockquote hoo, pre hey, pre ho, pre hoo, a hey, a ho, a hoo, ab hey, ab ho, ab hoo, br hey, br ho, br hoo, address hey, address ho, address hoo, cite hey, cite ho, cite hoo, code hey, code ho, code hoo, del hey, del ho, del hoo, dfn hey, dfn ho, dfn hoo, em hey, em ho, em hoo, img hey, img ho, img hoo, ins hey, ins ho, ins hoo, kbd hey, kbd ho, kbd hoo, q hey, q ho, q hoo, samp hey, samp ho, samp hoo, small hey, small ho, small hoo, strong hey, strong ho, strong hoo, su hey, su ho, su hoo, b hey, b ho, b hoo, sup hey, sup ho, sup hoo, var hey, var ho, var hoo, b hey, b ho, b hoo, u hey, u ho, u hoo, i hey, i ho, i hoo, dl hey, dl ho, dl hoo, dt hey, dt ho, dt hoo, dd hey, dd ho, dd hoo, ol hey, ol ho, ol hoo, ul hey, ul ho, ul hoo, li hey, li ho, li hoo, fieldset hey, fieldset ho, fieldset hoo, form hey, form ho, form hoo, label hey, label ho, label hoo, legend hey, legend ho, legend hoo, tab hey, tab ho, tab hoo, le hey, le ho, le hoo, caption hey, caption ho, caption hoo, tbody hey, tbody ho, tbody hoo, tfoot hey, tfoot ho, tfoot hoo, thead hey, thead ho, thead hoo, tr hey, tr ho, tr hoo, th hey, th ho, th hoo, td hey, td ho, td hoo {\
        \n  blah: bloo;\
        \n  blee: bleh;\
        \n}\
        \n"
    );
}

mod media;

// From "sass-spec/spec/non_conformant/scss/mixin-content-selectors.hrx"
#[test]
#[ignore] // wrong result
fn mixin_content_selectors() {
    assert_eq!(
        rsass(
            "@mixin foo($x: 1) {\
            \n  foo-sel {\
            \n    @content;\
            \n  }\
            \n}\
            \n\
            \ndiv {\
            \n  $x: hey;\
            \n  @include foo() {\
            \n    bar {\
            \n      color: red;\
            \n      hux {\
            \n        msg: $x;\
            \n      }\
            \n    }\
            \n  }\
            \n}"
        )
        .unwrap(),
        "div foo-sel bar {\
        \n  color: red;\
        \n}\
        \ndiv foo-sel bar hux {\
        \n  msg: hey;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/mixin-content-with-no-block.hrx"
#[test]
fn mixin_content_with_no_block() {
    assert_eq!(
        rsass(
            "@mixin foo {\
            \n  .foo {\
            \n    color: red;\
            \n    @content;\
            \n  }\
            \n}\
            \n\
            \ndiv.a {\
            \n  @include foo() {\
            \n    hey: now;\
            \n  }\
            \n}"
        )
        .unwrap(),
        "div.a .foo {\
        \n  color: red;\
        \n  hey: now;\
        \n}\
        \n"
    );
}

// Ignoring "mixin-content.hrx", not expected to work yet.

// From "sass-spec/spec/non_conformant/scss/mixin_with_keyword_args.hrx"
#[test]
fn mixin_with_keyword_args() {
    assert_eq!(
        rsass(
            "@mixin a-mixin($required, $arg1: default-val1, $arg2: default-val2) {\
            \n  required: $required;\
            \n  arg1: $arg1;\
            \n  arg2: $arg2;\
            \n}\
            \n.mixed { @include a-mixin(foo, $arg2: non-default-val2); }\
            \n"
        )
        .unwrap(),
        ".mixed {\
        \n  required: foo;\
        \n  arg1: default-val1;\
        \n  arg2: non-default-val2;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/mixins_with_args.hrx"
#[test]
fn mixins_with_args() {
    assert_eq!(
        rsass(
            "@mixin foo($a, $b) {\
            \n  a: $a;\
            \n  b: $b; }\
            \n\
            \n.foo {@include foo(bar, 12px)}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: bar;\
        \n  b: 12px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/mixins_with_empty_args.hrx"
#[test]
fn mixins_with_empty_args() {
    assert_eq!(
        rsass(
            "@mixin foo {a: b}\
            \n\
            \n.foo {@include foo();}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/multiline-var.hrx"
#[test]
fn multiline_var() {
    assert_eq!(
        rsass(
            "foo {\
            \n  $var1: 1 +\
            \n    2;\
            \n  $var2: true and\
            \n    false;\
            \n  $var3: a b\
            \n    c;\
            \n  a: $var1;\
            \n//  b: $var2;\
            \n  c: $var3; }\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: 3;\
        \n  c: a b c;\
        \n}\
        \n"
    );
}

// Ignoring "multiline_var.hrx", not expected to work yet.

// From "sass-spec/spec/non_conformant/scss/multiple-operators.hrx"
#[test]
fn multiple_operators() {
    assert_eq!(
        rsass(
            "$x: 2;\
            \n$y: 1;\
            \n\
            \n@function getResult() { @return true; }\
            \n\
            \n.test {\
            \n    a: $x > $y == getResult();\
            \n}\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  a: true;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/namespace_properties.hrx"
#[test]
fn namespace_properties() {
    assert_eq!(
        rsass(
            "foo {\
            \n  bar: baz;\
            \n  bang: {\
            \n    bip: 1px;\
            \n    bop: bar;}}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: baz;\
        \n  bang-bip: 1px;\
        \n  bang-bop: bar;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/namespace_properties_with_value.hrx"
#[test]
fn namespace_properties_with_value() {
    assert_eq!(
        rsass(
            "foo {\
            \n  bar: baz {\
            \n    bip: bop;\
            \n    bing: bop; }}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: baz;\
        \n  bar-bip: bop;\
        \n  bar-bing: bop;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/negation.hrx"
#[test]
#[ignore] // wrong result
fn negation() {
    assert_eq!(
        rsass(
            ".asdf {\
            \n  $bwidth: 52px;\
            \n  left: -$bwidth/3;\
            \n  right: (1/3);\
            \n  center: (10000/3);\
            \n  blah: (20/8);\
            \n}"
        )
        .unwrap(),
        ".asdf {\
        \n  left: -17.3333333333px;\
        \n  right: 0.3333333333;\
        \n  center: 3333.3333333333;\
        \n  blah: 2.5;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/nested-extend.hrx"
#[test]
#[ignore] // wrong result
fn nested_extend() {
    assert_eq!(
        rsass(
            ".sprites-nav {\
            \n  color: red;\
            \n}\
            \n\
            \n.sprites-nav_up {\
            \n  color: green;\
            \n}\
            \n\
            \n.mw_nav_button {\
            \n  float: right;\
            \n  width: 30px;\
            \n  height: 30px;\
            \n  margin: 10px 10px 10px 0;\
            \n  overflow: hidden;\
            \n  &[data-ur-state=\"disabled\"] {\
            \n    @extend .sprites-nav;\
            \n  }\
            \n  &[data-ur-state=\"enabled\"] {\
            \n    @extend .sprites-nav_up;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".sprites-nav, .mw_nav_button[data-ur-state=disabled] {\
        \n  color: red;\
        \n}\
        \n.sprites-nav_up, .mw_nav_button[data-ur-state=enabled] {\
        \n  color: green;\
        \n}\
        \n.mw_nav_button {\
        \n  float: right;\
        \n  width: 30px;\
        \n  height: 30px;\
        \n  margin: 10px 10px 10px 0;\
        \n  overflow: hidden;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/nested_namespace_properties.hrx"
#[test]
fn nested_namespace_properties() {
    assert_eq!(
        rsass(
            "foo {\
            \n  bar: baz;\
            \n  bang: {\
            \n    bip: 1px;\
            \n    bop: bar;\
            \n    blat:{baf:bort}}}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: baz;\
        \n  bang-bip: 1px;\
        \n  bang-bop: bar;\
        \n  bang-blat-baf: bort;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/nested_rules.hrx"
#[test]
fn nested_rules() {
    assert_eq!(
        rsass(
            "foo {\
            \n  bar {baz {a: b}}\
            \n  bang {bip {a: b}}}\
            \n"
        )
        .unwrap(),
        "foo bar baz {\
        \n  a: b;\
        \n}\
        \nfoo bang bip {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/nested_rules_with_declarations.hrx"
#[test]
fn nested_rules_with_declarations() {
    assert_eq!(
        rsass(
            "foo {\
            \n  ump: nump;\
            \n  grump: clump;\
            \n  bar {\
            \n    blat: bang;\
            \n    habit: rabbit;\
            \n    baz {a: b}\
            \n    bip {c: d}}\
            \n  bibble {\
            \n    bap {e: f}}}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  ump: nump;\
        \n  grump: clump;\
        \n}\
        \nfoo bar {\
        \n  blat: bang;\
        \n  habit: rabbit;\
        \n}\
        \nfoo bar baz {\
        \n  a: b;\
        \n}\
        \nfoo bar bip {\
        \n  c: d;\
        \n}\
        \nfoo bibble bap {\
        \n  e: f;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/nested_rules_with_fancy_selectors.hrx"
#[test]
fn nested_rules_with_fancy_selectors() {
    assert_eq!(
        rsass(
            "foo {\
            \n  .bar {a: b}\
            \n  :baz {c: d}\
            \n  bang:bop {e: f}}\
            \n"
        )
        .unwrap(),
        "foo .bar {\
        \n  a: b;\
        \n}\
        \nfoo :baz {\
        \n  c: d;\
        \n}\
        \nfoo bang:bop {\
        \n  e: f;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/newlines_in_selectors.hrx"
#[test]
#[ignore] // wrong result
fn newlines_in_selectors() {
    assert_eq!(
        rsass(
            "foo, bar\
            \nbaz {\
            \n  bang, bip\
            \n  bop {a: b}}\
            \n"
        )
        .unwrap(),
        "foo bang, foo bip\
        \nbop, bar\
        \nbaz bang, bar\
        \nbaz bip\
        \nbop {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/no_namespace_properties_without_space.hrx"
#[test]
fn no_namespace_properties_without_space() {
    assert_eq!(
        rsass(
            "foo {\
            \n  bar:baz {\
            \n    bip: bop }}\
            \n"
        )
        .unwrap(),
        "foo bar:baz {\
        \n  bip: bop;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/null.hrx"
#[test]
fn null() {
    assert_eq!(
        rsass(
            "$x: 2;\
            \n\
            \ndiv {\
            \n  $x: null;\
            \n  a: length(null null null);\
            \n  b: #{null};\
            \n  d: type-of($x);\
            \n  e: null == null;\
            \n  f: -null;\
            \n  g: -fudge;\
            \n  h: (null null null);\
            \n  i: froo(null, 4);\
            \n  j: (null), (null), 3, 4;\
            \n  k: length(((null), (null), 3, 4));\
            \n  \
            \n  a2: length($x $x $x);\
            \n  b2: #{$x};\
            \n  e2: $x == null;\
            \n  f2: -$x;\
            \n  h2: ($x $x $x);\
            \n  i2: froo($x, 4);\
            \n  j2: ($x), ($x), 3, 4;\
            \n  k2: length((($x), ($x), 3, 4));\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  a: 3;\
        \n  d: null;\
        \n  e: true;\
        \n  f: -null;\
        \n  g: -fudge;\
        \n  i: froo(, 4);\
        \n  j: 3, 4;\
        \n  k: 4;\
        \n  a2: 3;\
        \n  e2: true;\
        \n  f2: -;\
        \n  i2: froo(, 4);\
        \n  j2: 3, 4;\
        \n  k2: 4;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/numbers.hrx"
#[test]
fn numbers() {
    assert_eq!(
        rsass(
            "div {\
            \n  width: 10px;\
            \n  height: 20%;\
            \n  blah: 12;\
            \n  color: #abc;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  width: 10px;\
        \n  height: 20%;\
        \n  blah: 12;\
        \n  color: #abc;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/one_line_comments.hrx"
#[test]
fn one_line_comments() {
    assert_eq!(
        rsass(
            ".foo bar[val=\"//\"] {\
            \n  baz: bang; //}\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo bar[val=\"//\"] {\
        \n  baz: bang;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/parent_selectors.hrx"
#[test]
fn parent_selectors() {
    assert_eq!(
        rsass(
            "foo {\
            \n  &:hover {a: b}\
            \n  bar &.baz {c: d}}\
            \n"
        )
        .unwrap(),
        "foo:hover {\
        \n  a: b;\
        \n}\
        \nbar foo.baz {\
        \n  c: d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/passing_all_as_keyword_args_in_opposite_order.hrx"
#[test]
fn passing_all_as_keyword_args_in_opposite_order() {
    assert_eq!(
        rsass(
            "@mixin a-mixin($required, $arg1: default-val1, $arg2: default-val2) {\
            \n  required: $required;\
            \n  arg1: $arg1;\
            \n  arg2: $arg2; }\
            \n.mixed { @include a-mixin($arg2: non-default-val2, $arg1: non-default-val1, $required: foo); }\
            \n"
        )
        .unwrap(),
        ".mixed {\
        \n  required: foo;\
        \n  arg1: non-default-val1;\
        \n  arg2: non-default-val2;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/passing_required_args_as_a_keyword_arg.hrx"
#[test]
fn passing_required_args_as_a_keyword_arg() {
    assert_eq!(
        rsass(
            "@mixin a-mixin($required, $arg1: default-val1, $arg2: default-val2) {\
            \n  required: $required;\
            \n  arg1: $arg1;\
            \n  arg2: $arg2; }\
            \n.mixed { @include a-mixin($required: foo); }\
            \n"
        )
        .unwrap(),
        ".mixed {\
        \n  required: foo;\
        \n  arg1: default-val1;\
        \n  arg2: default-val2;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/percentages.hrx"
#[test]
fn percentages() {
    assert_eq!(
        rsass(
            "div {\
            \n  width: 10% + 20%;\
            \n  height: 10% - 20%;\
            \n  width: 10% + 10;\
            \n  width: 10 + 10%;\
            \n  height: 10% - 10;\
            \n  height: 10 - 10%;\
            \n  blah: (20% / 4%);\
            \n  flah: 12 * 75%;\
            \n  grah: 75% * 12;\
            \n  // hwah: (24 / 8%);\
            \n  nyah: (35% / 7);\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  width: 30%;\
        \n  height: -10%;\
        \n  width: 20%;\
        \n  width: 20%;\
        \n  height: 0%;\
        \n  height: 0%;\
        \n  blah: 5;\
        \n  flah: 900%;\
        \n  grah: 900%;\
        \n  nyah: 5%;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/placeholder-with-media.hrx"
#[test]
#[ignore] // wrong result
fn placeholder_with_media() {
    assert_eq!(
        rsass(
            "%a {\
            \n  @media only screen and (max-width: 100px) {\
            \n    color: red;\
            \n  }\
            \n}\
            \n\
            \nb {\
            \n  @extend %a;\
            \n}\
            \n"
        )
        .unwrap(),
        "@media only screen and (max-width: 100px) {\
        \n  b {\
        \n    color: red;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/placeholder.hrx"
#[test]
#[ignore] // wrong result
fn placeholder() {
    assert_eq!(
        rsass(
            "%x {\
            \n  color: red;\
            \n}\
            \n\
            \nfoo {\
            \n  width: 10px;\
            \n  @extend %x;\
            \n}\
            \n\
            \nhux {\
            \n  height: 12px;\
            \n  @extend %x;\
            \n}"
        )
        .unwrap(),
        "hux, foo {\
        \n  color: red;\
        \n}\
        \nfoo {\
        \n  width: 10px;\
        \n}\
        \nhux {\
        \n  height: 12px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/precision.hrx"
#[test]
#[ignore] // wrong result
fn precision() {
    assert_eq!(
        rsass(
            "div {\
            \n  a: (20/3);\
            \n  b: (5/2);\
            \n  c: (9/3);\
            \n  d: (20/-3);\
            \n  e: (-5/2);\
            \n  f: -(9/3);\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  a: 6.6666666667;\
        \n  b: 2.5;\
        \n  c: 3;\
        \n  d: -6.6666666667;\
        \n  e: -2.5;\
        \n  f: -3;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/prop_name_interpolation_after_hyphen.hrx"
#[test]
fn prop_name_interpolation_after_hyphen() {
    assert_eq!(
        rsass(
            "a { -#{\"foo\"}-bar: b; }\
            \n"
        )
        .unwrap(),
        "a {\
        \n  -foo-bar: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/prop_name_only_interpolation.hrx"
#[test]
fn prop_name_only_interpolation() {
    assert_eq!(
        rsass(
            "foo {#{\"baz\" + \"bang\"}: blip}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bazbang: blip;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/property_interpolation_at_dashes.hrx"
#[test]
fn property_interpolation_at_dashes() {
    assert_eq!(
        rsass(
            "$a : a;\
            \n$b : b;\
            \ndiv { -foo-#{$a}-#{$b}-foo: foo }\
            \n"
        )
        .unwrap(),
        "div {\
        \n  -foo-a-b-foo: foo;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/quotes-in-interpolated-strings.hrx"
#[test]
fn quotes_in_interpolated_strings() {
    assert_eq!(
        rsass(
            "$bar: \"bar\";\
            \n$foobar: \"foo#{$bar}\";\
            \n#{$bar} {\
            \n  #{$bar}: #{$bar};\
            \n  #{$bar}: $bar;\
            \n}\
            \nfoobar {\
            \n  #{$foobar}: #{$foobar};\
            \n  #{$foobar}: $foobar;\
            \n}"
        )
        .unwrap(),
        "bar {\
        \n  bar: bar;\
        \n  bar: \"bar\";\
        \n}\
        \nfoobar {\
        \n  foobar: foobar;\
        \n  foobar: \"foobar\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/retina-image.hrx"
#[test]
fn retina_image() {
    assert_eq!(
        rsass(
            "@mixin retina-image($filename, $background-size, $extension: png, $retina-filename: null, $asset-pipeline: false) {\
            \n  @if $asset-pipeline {\
            \n    background-image: image_url($filename + \".\" + $extension);\
            \n  }\
            \n  @else {\
            \n    background-image: url($filename + \".\" + $extension);\
            \n  }\
            \n  @include hidpi {\
            \n    @if $asset-pipeline {\
            \n      @if $retina-filename {\
            \n        background-image: image_url($retina-filename + \".\" + $extension);\
            \n      }\
            \n      @else {\
            \n        background-image: image_url($filename + \"@2x\" + \".\" + $extension);\
            \n      }\
            \n    }\
            \n    @else {\
            \n      @if $retina-filename {\
            \n        background-image: url($retina-filename + \".\" + $extension);\
            \n      }\
            \n      @else {\
            \n        background-image: url($filename + \"@2x\" + \".\" + $extension);\
            \n      }\
            \n    }\
            \n    background-size: $background-size;\
            \n  }\
            \n}"
        )
        .unwrap(),
        ""
    );
}

// From "sass-spec/spec/non_conformant/scss/sass_script.hrx"
#[test]
fn sass_script() {
    assert_eq!(
        rsass(
            "foo {\
            \n  a: 1 + 2;\
            \n  b: 1 - 2;\
            \n  c: foo + bar;\
            \n  d: floor(12.3px); }\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: 3;\
        \n  b: -1;\
        \n  c: foobar;\
        \n  d: 12px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/selector_interpolation_at_attr_beginning.hrx"
#[test]
fn selector_interpolation_at_attr_beginning() {
    assert_eq!(
        rsass(
            "$zzz: zzz;\
            \n[#{$zzz}=foo] { a: b; }\
            \n"
        )
        .unwrap(),
        "[zzz=foo] {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/selector_interpolation_at_class_begininng.hrx"
#[test]
fn selector_interpolation_at_class_begininng() {
    assert_eq!(
        rsass(
            "$zzz: zzz;\
            \n.#{$zzz} { a: b; }\
            \n"
        )
        .unwrap(),
        ".zzz {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/selector_interpolation_at_id_begininng.hrx"
#[test]
fn selector_interpolation_at_id_begininng() {
    assert_eq!(
        rsass(
            "$zzz: zzz;\
            \n##{$zzz} { a: b; }\
            \n"
        )
        .unwrap(),
        "#zzz {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/selector_interpolation_at_pseudo_begininng.hrx"
#[test]
fn selector_interpolation_at_pseudo_begininng() {
    assert_eq!(
        rsass(
            "$zzz: zzz;\
            \n:#{$zzz}::#{$zzz} { a: b; }\
            \n"
        )
        .unwrap(),
        ":zzz::zzz {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/selector_interpolation_in_pseudoclass.hrx"
#[test]
fn selector_interpolation_in_pseudoclass() {
    assert_eq!(
        rsass(
            "foo:nth-child(#{5 + \"n\"}) {a: b}\
            \n"
        )
        .unwrap(),
        "foo:nth-child(5n) {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/several_namespace_properties.hrx"
#[test]
fn several_namespace_properties() {
    assert_eq!(
        rsass(
            "foo {\
            \n  bar: baz;\
            \n  bang: {\
            \n    bip: 1px;\
            \n    bop: bar;}\
            \n  buzz: {\
            \n    fram: \"foo\";\
            \n    frum: moo;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: baz;\
        \n  bang-bip: 1px;\
        \n  bang-bop: bar;\
        \n  buzz-fram: \"foo\";\
        \n  buzz-frum: moo;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/simple-inheritance.hrx"
#[test]
#[ignore] // wrong result
fn simple_inheritance() {
    assert_eq!(
        rsass(
            "earth {\
            \n  mammal, bird {\
            \n    blood: warm;\
            \n  }\
            \n}\
            \n\
            \nearth {\
            \n  mammal {\
            \n    produces-milk: true;\
            \n  }\
            \n}\
            \n\
            \n@mixin mammal-says($message) {\
            \n  @extend mammal;\
            \n  says: $message;\
            \n}\
            \n\
            \ndog {\
            \n  @include mammal-says(\"Woof!\");\
            \n}\
            \n\
            \ncat {\
            \n  @include mammal-says(\"Meow.\");\
            \n}\
            \n\
            \nhorse, naysayer {\
            \n  @include mammal-says(\"Nay.\");\
            \n}\
            \n\
            \n[hey] {\
            \n  a: b;\
            \n}\
            \n\
            \nho {\
            \n  @extend [hey];\
            \n  c: d;\
            \n}\
            \n\
            \nfancy outer space vehicle {\
            \n  insides: advanced;\
            \n}\
            \n\
            \nnew american mars rover {\
            \n  wheels: big;\
            \n  @extend vehicle;\
            \n}\
            \n\
            \nfoo {\
            \n  something: whatever;\
            \n}\
            \n\
            \na b c {\
            \n  blah: blah;\
            \n  @extend foo;\
            \n}\
            \n\
            \nd e f {\
            \n  blah: blah;\
            \n}\
            \n\
            \ng {\
            \n  @extend f;\
            \n  bloo: bloo;\
            \n}"
        )
        .unwrap(),
        "earth mammal, earth horse, earth naysayer, earth cat, earth dog, earth bird {\
        \n  blood: warm;\
        \n}\
        \nearth mammal, earth horse, earth naysayer, earth cat, earth dog {\
        \n  produces-milk: true;\
        \n}\
        \ndog {\
        \n  says: \"Woof!\";\
        \n}\
        \ncat {\
        \n  says: \"Meow.\";\
        \n}\
        \nhorse, naysayer {\
        \n  says: \"Nay.\";\
        \n}\
        \n[hey], ho {\
        \n  a: b;\
        \n}\
        \nho {\
        \n  c: d;\
        \n}\
        \nfancy outer space vehicle, fancy outer space new american mars rover, new american mars fancy outer space rover {\
        \n  insides: advanced;\
        \n}\
        \nnew american mars rover {\
        \n  wheels: big;\
        \n}\
        \nfoo, a b c {\
        \n  something: whatever;\
        \n}\
        \na b c {\
        \n  blah: blah;\
        \n}\
        \nd e f, d e g {\
        \n  blah: blah;\
        \n}\
        \ng {\
        \n  bloo: bloo;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/simple-lists.hrx"
#[test]
fn simple_lists() {
    assert_eq!(
        rsass(
            "div {\
            \n  hey: a, b, c, d;\
            \n  ho: a b c d;\
            \n  ha: unquote(\"a, b, c, d\");\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  hey: a, b, c, d;\
        \n  ho: a b c d;\
        \n  ha: a, b, c, d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/star_plus_and_parent.hrx"
#[test]
fn star_plus_and_parent() {
    assert_eq!(
        rsass(
            "foo {*+html & {a: b}}\
            \n"
        )
        .unwrap(),
        "* + html foo {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/strings.hrx"
#[test]
fn strings() {
    assert_eq!(
        rsass(
            "div {\
            \n  content: blang + 1;\
            \n  content: 1 + blang;\
            \n  content: \"blang\" + 1;\
            \n  content: 1 + \"blang\";\
            \n  content: bar + \"foo\";\
            \n  content: \"quoted\" + unquoted;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  content: blang1;\
        \n  content: 1blang;\
        \n  content: \"blang1\";\
        \n  content: \"1blang\";\
        \n  content: barfoo;\
        \n  content: \"quotedunquoted\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/url_import.hrx"
#[test]
fn url_import() {
    assert_eq!(
        rsass("@import url(fonts.sass);").unwrap(),
        "@import url(fonts.sass);\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/variables.hrx"
#[test]
fn variables() {
    assert_eq!(
        rsass(
            "foo {\
            \n  $var: 2;\
            \n  $another-var: 4;\
            \n  a: $var;\
            \n  b: $var + $another-var;}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: 2;\
        \n  b: 6;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/vars.hrx"
#[test]
fn vars() {
    assert_eq!(
        rsass(
            "$x: hello;\
            \n$y: 1/2 3/4 (2+3);\
            \n\
            \ndiv {\
            \n  content: 1 2 $x;\
            \n  content: $y;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  content: 1 2 hello;\
        \n  content: 1/2 3/4 5;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/weird-selectors.hrx"
#[test]
fn weird_selectors() {
    assert_eq!(
        rsass(
            "> > E {\
            \n  color: red;\
            \n}\
            \n\
            \nE > > {\
            \n  color: red;\
            \n}\
            \n\
            \n> > E > > {\
            \n  > > F > > {\
            \n    color: red;\
            \n  }\
            \n}"
        )
        .unwrap(),
        "> > E {\
        \n  color: red;\
        \n}\
        \nE > > {\
        \n  color: red;\
        \n}\
        \n> > E > > > > F > > {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/weird_added_space.hrx"
#[test]
fn weird_added_space() {
    assert_eq!(
        rsass(
            "$value : bip;\
            \n\
            \nfoo {\
            \n  bar: -moz-#{$value};\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: -moz-bip;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/while.hrx"
#[test]
fn test_while() {
    assert_eq!(
        rsass(
            "div {\
            \n  $x : true;\
            \n  @while $x {\
            \n    stuff: 1;\
            \n    more-stuff: 2;\
            \n    even-more-stuff: 3;\
            \n    lets-stop-now: 4;\
            \n    $x: false;\
            \n  }\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  stuff: 1;\
        \n  more-stuff: 2;\
        \n  even-more-stuff: 3;\
        \n  lets-stop-now: 4;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/while_directive.hrx"
#[test]
fn while_directive() {
    assert_eq!(
        rsass(
            "$i: 1;\
            \n\
            \n.foo {\
            \n  @while $i != 5 {\
            \n    a: $i;\
            \n    $i: $i + 1;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: 1;\
        \n  a: 2;\
        \n  a: 3;\
        \n  a: 4;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/while_in_functions.hrx"
#[test]
fn while_in_functions() {
    assert_eq!(
        rsass(
            "@function test-while() {\
            \n  $x : true;\
            \n  @while $x {\
            \n    @return $x\
            \n  }\
            \n}\
            \n\
            \ndiv {\
            \n  y: test-while();\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  y: true;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/scss/while_without_condition.hrx"

// Ignoring "while_without_condition", error tests are not supported yet.

// From "sass-spec/spec/non_conformant/scss/zero-compression.hrx"
#[test]
fn zero_compression() {
    assert_eq!(
        rsass(
            "$orig: 0.12em;\r\
            \n$value: (0.12em);\r\
            \n$score: (item-height: 0.12em);\r\
            \nfoo {\r\
            \n    tst-1: 0 -#{0.12em};\r\
            \n    tst-2: 0 -#{$orig};\r\
            \n    tst-3: 0 -#{$value};\r\
            \n    tst-4: 0 -#{map-get($score, item-height)};\r\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  tst-1: 0 -0.12em;\
        \n  tst-2: 0 -0.12em;\
        \n  tst-3: 0 -0.12em;\
        \n  tst-4: 0 -0.12em;\
        \n}\
        \n"
    );
}
