//! Tests from spec/scss
extern crate rsass;
use rsass::{compile_scss, OutputStyle};

#[test]
fn almost_ambiguous_nested_rules_and_declarations() {
    check(
        b"foo {\n  \
            bar:baz:bang:bop:biddle:woo:look:at:all:these:pseudoclasses \
            {a: b};\n  \
            bar:baz bang bop biddle woo look at all these elems {a: b};\n  \
            bar:baz bang bop biddle woo look at all these elems; }\n",
        "foo {\n  bar: baz bang bop biddle woo look at all these elems;\n}\n\
         foo bar:baz:bang:bop:biddle:woo:look:at:all:these:pseudoclasses \
         {\n  a: b;\n}\n\
         foo bar:baz bang bop biddle woo look at all these elems \
         {\n  a: b;\n}\n",
    )
}

#[test]
fn alpha() {
    check(
        b"$x: rgb(0, 255, 255);\n\n\
            div {\n  color: rgb(255, $blue: 0, $green: 255);\n  \
            background: rgb(123, 45, 6);\n//  \
            flah: rgba(0, 0, 0, 1) + #111;\n  \
            grah: rgba(#f0e, $alpha: .5);\n//  blah: rgba(1,2,3,.6);\n\n  \
            floo: $x;\n//  bloo: rgba($x, 0.7);\n  groo: $x;\n\n  \
            $x: rgb(123, 45, 6);\n\n  \
            hoo: red($x);\n\n  moo: green($x);\n  poo: blue($x);\n\n\
            //  goo: mix(rgba(255, 0, 0, 0.5), #00f);\n\n\
            boo: invert(#123456);\n}\n",
        "div {\n  color: yellow;\n  background: #7b2d06;\n  \
         grah: rgba(255, 0, 238, 0.5);\n  floo: cyan;\n  groo: cyan;\n  \
         hoo: 123;\n  moo: 45;\n  poo: 6;\n  boo: #edcba9;\n}\n",
    )
}

#[test]
fn append() {
    check(
        b"div {\n  \
            $l: append(a b, c d);\n  foo: nth($l, 3);\n  bar: type-of($l);\n}",
        "div {\n  foo: c d;\n  bar: list;\n}\n",
    )
}

#[test]
fn arglist() {
    check(
        b"@mixin foo($x, $y, $zs...) {\n  \
            foo-x: $x;\n  foo-y: $y;\n  foo-zs: $zs;\n}\n\n\
            div {\n  @include foo(a, b, c, d, e);\n}",
        "div {\n  foo-x: a;\n  foo-y: b;\n  foo-zs: c, d, e;\n}\n",
    )
}

#[test]
fn backrefs_in_selector_groups() {
    check(
        b"a {\n  &:c, & d {\n    hey: ho;\n  }\n}\n\n\
            a b {\n  &:c, & d {\n    hey: ho;\n  }\n}\n",
        "a:c, a d {\n  hey: ho;\n}\n\n\
         a b:c, a b d {\n  hey: ho;\n}\n",
    )
}

#[test]
fn backslash() {
    check(
        b"div, span {\n  color: red;\n  \\ foo {\n    color: blue;\n  }\n}",
        "div, span {\n  color: red;\n}\n\
         div \\ foo, span \\ foo {\n  color: blue;\n}\n",
    )
}

#[test]
fn basic_function() {
    check(
        b"@function foo() {\n  @return 1 + 2;\n}\n\n\
            bar {\n  a: foo();\n}\n",
        "bar {\n  a: 3;\n}\n",
    )
}

#[test]
fn block_comment_in_script() {
    check(b"foo {a: 1 + /* flang */ bar}\n", "foo {\n  a: 1bar;\n}\n")
}

#[test]
fn css_block_directive_with_semicolon() {
    check(
        b"@foo {\n  a: b; }\n\n@bar {\n  a: b; }\n",
        "@foo {\n  a: b;\n}\n@bar {\n  a: b;\n}\n",
    )
}

#[test]
fn css_selector_hacks() {
    check(b"> > E {\n  a: b; }\n", "> > E {\n  a: b;\n}\n")
}

#[test]
fn css_unary_ops() {
    check(
        b"foo {\n  a: -0.5em;\n  b: 0.5em;\n  \
            c: -foo(12px);\n  d: +foo(12px); }\n",
        "foo {\n  a: -0.5em;\n  b: 0.5em;\n  \
         c: -foo(12px);\n  d: +foo(12px);\n}\n",
    )
}

#[test]
fn concat() {
    check(
        b"div {\n  a: hello + \"goodbye\";\n  b: \"hello\" + goodbye;\n  \
            c: 3 + \"hello\";\n  d: \"hello\" + 3;\n}",
        "div {\n  a: hellogoodbye;\n  b: \"hellogoodbye\";\n  \
         c: \"3hello\";\n  d: \"hello3\";\n}\n",
    )
}

#[test]
fn cons_up() {
    check(
        b"$inputs-list: 'input[type=\"email\"]',\n\t'\
            input[type=\"number\"]',\n\t'input[type=\"password\"]',\n\t\
            'input[type=\"search\"]',\n\t'input[type=\"tel\"]',\n\t\
            'input[type=\"text\"]',\n\t'input[type=\"url\"]',\n\n\
            // Webkit & Gecko may change the display of these in the future\
            \n\t'input[type=\"color\"]',\n\t'input[type=\"date\"]',\n\t\
            'input[type=\"datetime\"]',\n\t\
            'input[type=\"datetime-local\"]',\n\t'input[type=\"month\"]',\n\t\
            'input[type=\"time\"]',\n\t'input[type=\"week\"]';\n\n\
            $unquoted-inputs-list: ();\n\n\
            @each $input-type in $inputs-list {\n  \
            $unquoted-inputs-list: append($unquoted-inputs-list, \
            unquote($input-type), comma);\n}\n\n\
            div {\n  content: $unquoted-inputs-list;\n  \
            content: append((), hello);\n  content: length(());\n}",
        "div {\n  \
         content: input[type=\"email\"], input[type=\"number\"], \
         input[type=\"password\"], input[type=\"search\"], \
         input[type=\"tel\"], input[type=\"text\"], input[type=\"url\"], \
         input[type=\"color\"], input[type=\"date\"], \
         input[type=\"datetime\"], input[type=\"datetime-local\"], \
         input[type=\"month\"], input[type=\"time\"], \
         input[type=\"week\"];\n  \
         content: hello;\n  content: 0;\n}\n",
    )
}

#[test]
fn directives_in_propsets() {
    check(
        b"$color: red;\n$position: 50%;\n$x: 0;\n\n\
            @mixin foo() {\n  image: url(foo.png);\n}\n\n\
            div {\n  background: {\n    \
            something: {\n      color: green;\n    }\n    \
            @if (type-of($color) == \"color\") {\n      color: $color;\n    \
            }\n    \
            @if (type-of($position) == \"number\") {\n      \
            position: $position;\n      @include foo();\n    }\n    \
            groo: foo;\n  }\n  width: $x;\n}",
        "div {\n  background-something-color: green;\n  \
         background-color: red;\n  background-position: 50%;\n  \
         background-image: url(foo.png);\n  background-groo: foo;\n  \
         width: 0;\n}\n",
    )
}

#[test]
fn each_in_functions() {
    check(
        b"$GLOBAL: global;\n\n\
            @function foo($g1, $g2, $g3) {\n  \
            @each $value in $g1, $g2, $g3 {\n    \
            $GLOBAL: $GLOBAL each $value !global;\n    \
            $GLOBAL: $GLOBAL type1 type-of(nth($value, 1)) !global;\n    \
            $GLOBAL: $GLOBAL type2 type-of(nth($value, 2)) !global;\n  }\n  \
            @each $value in (foo: foo, bar: bar) {\n    \
            $GLOBAL: $GLOBAL map $value !global;\n  }\n  @return 0;\n}\n\n\
            div {\n  a: foo(50% 50%, cover circle, red blue);\n  \
            b: $GLOBAL;\n  $colors: red green blue;\n  \
            c: a, b, type-of(nth($colors, 2)), d;\n}\n",
        "div {\n  a: 0;\n  \
         b: global each 50% 50% type1 number type2 number each cover \
         circle type1 string type2 string each red blue type1 color \
         type2 color map foo foo map bar bar;\n  \
         c: a, b, color, d;\n}\n",
    )
}

mod feature_queries {
    use super::check;

    #[test]
    fn nested() {
        check(
            b".foo {\n     display: block;\n     \
                @supports (display: flex) {\n         display: flex;\n     \
                }\n }\n",
            ".foo {\n  display: block;\n}\n@supports (display: flex) {\n  \
             .foo {\n    display: flex;\n  }\n}\n",
        )
    }
}

#[test]
fn for_in_functions() {
    check(
        b"@function foo() {\n  $limit: 10;\n  $y: 0;\n  \
            @for $x from 1 through $limit {\n    \
            $limit: 4;\n    $y: $y + $x;\n  }\n  \
            @return $y;\n}\n\n\
            div {\n	width: foo();\n}",
        "div {\n  width: 55;\n}\n",
    )
}

#[test]
fn function_names_4_0() {
    check(
        b"div {\n  color: unquote(\"hello\");\n  \
            color: un#{quo}te(\"hello\");\n  \
            color: (\"hello\")un#{quo}te;\n}\n",
        "div {\n  color: hello;\n  color: unquote(\"hello\");\n  \
         color: \"hello\" unquote;\n}\n",
    )
}

#[test]
fn functions() {
    check(
        b"@function foo($x, $y, $z) {\n  @while $x < $y {\n    \
            $z: transform($z);\n    @return $z;\n  }\n}\n\n\
            @function bar($x) {\n  @if $x {\n    @return YES;\n  }\n}\n\n\
            div {\n  answer: bar(true);\n  \
            flanswer: fudge(mux+flux) + mudge(a/b);\n}",
        "div {\n  answer: YES;\n  flanswer: fudge(muxflux)mudge(a/b);\n}\n",
    )
}

#[test]
fn functions_and_mixins() {
    check(
        b"@function foo() {\n  @return \"hello\";\n}\n\n\
            @mixin foo() {\n  content: \"hello\";\n}\n\n\
            div {\n  span {\n    @function length($a, $b, $c, $d) {\n      \
            @return $a + $b + $c + $d;\n    }\n\n    \
            div {\n      content: foo();\n      @include foo();\n      \
            width: length(1,2,2,3);\n    }\n  }\n\n  \
            height: length(a b c d e);\n\n}",
        "div {\n  height: 5;\n}\n\
         div span div {\n  content: \"hello\";\n  content: \"hello\";\n  \
         width: 8;\n}\n",
    )
}

#[test]
fn important() {
    check(
        b"div {\n  color: red ! important;\n  width: 5px ! important;\n}",
        "div {\n  color: red !important;\n  width: 5px !important;\n}\n",
    )
}

/// My own addition
#[test]
fn important_compact_input() {
    check(
        b"div {z-index: 40!important}",
        "div {\n  z-index: 40 !important;\n}\n",
    )
}

#[test]
fn interpolation() {
    check(
        b"$bar : \"#foo\";\n\n\n\
            ul li#{$bar} a span.label { foo: bar; }\n",
        "ul li#foo a span.label {\n  foo: bar;\n}\n",
    )
}

/// My own addition
#[test]
fn interpolation_sq() {
    check(
        b"$bar : '#foo';\n\n\
            ul li#{$bar} a span.label { foo: bar; }\n",
        "ul li#foo a span.label {\n  foo: bar;\n}\n",
    )
}

#[test]
fn comparable() {
    // TODO One line, involving a fictional unit, removed here
    check(
        b".color-functions {
  $color: red;
  hue: hue($color);
  hue-type: type-of(hue($color));
  hue-unit: unit(hue($color));
  hue-comparable: comparable(hue($color), hue($color));
  x: comparable(10px, 10px);
  x: comparable(10, 10);
  y: type-of(10px);
  y: type-of(10deg);
  z: lightness(red);
  z: type-of(lightness(red));
  z: type-of(50%);
  z: comparable(lightness(red), 50%);
}",
        ".color-functions {
  hue: 0deg;
  hue-type: number;
  hue-unit: \"deg\";
  hue-comparable: true;
  x: true;
  x: true;
  y: number;
  y: number;
  z: 50%;
  z: number;
  z: number;
  z: true;
}
",
    )
}

#[test]
fn index() {
    check(
        b"div {\n  foo: index(hello goodbye futz, goodbye);\n  \
            bar: index(hello goodbye futz, badbye);\n  \
            baz: index((hello world) (my name) (is aaron), is aaron);\n}",
        "div {\n  foo: 2;\n  baz: 3;\n}\n",
    )
}

#[test]
fn keyword_args_in_functions() {
    check(
        b".keyed { color: rgba($color: #a7c, $alpha: 0.4) }\n",
        ".keyed {\n  color: rgba(170, 119, 204, 0.4);\n}\n",
    )
}

#[test]
fn media2() {
    check(
        b"$foo: 3;\n$bar: 4;\n\n\
            @media only screen and (max-width: $foo) and (min-width: $bar) \
            {\n  \
            /* hey */\n  \
            a {color: red}\n}",
        "@media only screen and (max-width: 3) and (min-width: 4) {\n  \
         /* hey */\n  \
         a {\n    color: red;\n  }\n}\n",
    )
}

#[test]
fn mixin_content_with_no_block() {
    check(
        b"@mixin foo {\n  .foo {\n    color: red;\n    @content;\n  }\n}\n\n\
            div.a {\n  @include foo() {\n    hey: now;\n  }\n}",
        "div.a .foo {\n  color: red;\n  hey: now;\n}\n",
    );
}

#[test]
fn mixins_with_args() {
    check(
        b"@mixin foo($a, $b) {\n  a: $a;\n  b: $b; }\n\n\
            .foo {@include foo(bar, 12px)}\n",
        ".foo {\n  a: bar;\n  b: 12px;\n}\n",
    )
}

#[test]
fn nested_namespace_properties() {
    check(
        b"foo {\n  bar: baz;\n  bang: {\n    \
            bip: 1px;\n    bop: bar;\n    blat:{baf:bort}}}\n",
        "foo {\n  bar: baz;\n  bang-bip: 1px;\n  bang-bop: bar;\n  \
         bang-blat-baf: bort;\n}\n",
    )
}

#[test]
fn null() {
    check(
        b"$x: 2;\n\n\
            div {\n  $x: null;\n  a: length(null null null);\n  b: #{null};\n  \
            d: type-of($x);\n  e: null == null;\n  f: -null;\n  g: -fudge;\n  \
            h: (null null null);\n  i: froo(null, 4);\n  \
            j: (null), (null), 3, 4;\n  k: length(((null), (null), 3, 4));\n\n\
            a2: length($x $x $x);\n  b2: #{$x};\n  e2: $x == null;\n  \
            f2: -$x;\n  h2: ($x $x $x);\n  i2: froo($x, 4);\n  \
            j2: ($x), ($x), 3, 4;\n  k2: length((($x), ($x), 3, 4));\n}",
        "div {\n  a: 3;\n  d: null;\n  e: true;\n  f: -null;\n  \
         g: -fudge;\n  i: froo(, 4);\n  j: 3, 4;\n  k: 4;\n  \
         a2: 3;\n  e2: true;\n  f2: -;\n  i2: froo(, 4);\n  j2: 3, 4;\n  \
         k2: 4;\n}\n",
    )
}

#[test]
fn precision() {
    check(
        b"div {\n  a: (20/3);\n  b: (5/2);\n  c: (9/3);\n  d: (20/-3);\n  \
            e: (-5/2);\n  f: -(9/3);\n}",
        "div {\n  a: 6.66667;\n  b: 2.5;\n  c: 3;\n  d: -6.66667;\n  \
         e: -2.5;\n  f: -3;\n}\n",
    )
}

#[test]
fn prop_name_only_interpolation() {
    check(
        b"foo {#{\"baz\" + \"bang\"}: blip}\n",
        "foo {\n  bazbang: blip;\n}\n",
    )
}

#[test]
fn prop_name_interpolation_after_hyphen() {
    check(b"a { -#{\"foo\"}-bar: b; }\n", "a {\n  -foo-bar: b;\n}\n")
}

#[test]
fn star_plus_and_parent() {
    check(b"foo {*+html & {a: b}}\n", "* + html foo {\n  a: b;\n}\n")
}

#[test]
fn selector_interpolation_at_attr_beginning() {
    check(
        b"$zzz: zzz;\n[#{$zzz}=foo] { a: b; }\n",
        "[zzz=foo] {\n  a: b;\n}\n",
    )
}

#[test]
fn selector_interpolation_at_id_begininng() {
    check(b"$zzz: zzz;\n##{$zzz} { a: b; }\n", "#zzz {\n  a: b;\n}\n")
}

#[test]
fn selector_interpolation_at_pseudo_begininng() {
    check(
        b"$zzz: zzz;\n:#{$zzz}::#{$zzz} { a: b; }\n",
        ":zzz::zzz {\n  a: b;\n}\n",
    )
}

#[test]
fn selector_interpolation_in_pseudoclass() {
    check(
        b"foo:nth-child(#{5 + \"n\"}) {a: b}\n",
        "foo:nth-child(5n) {\n  a: b;\n}\n",
    )
}

#[test]
fn while_directive() {
    check(
        b"$i: 1;\n\n\
            .foo {\n  @while $i != 5 {\n    a: $i;\n    $i: $i + 1;\n  }\n}\n",
        ".foo {\n  a: 1;\n  a: 2;\n  a: 3;\n  a: 4;\n}\n",
    )
}

#[test]
fn while_in_functions() {
    check(
        b"@function test-while() {\n  \
            $x : true;\n  @while $x {\n    @return $x\n  }\n}\n\n\
            div {\n  y: test-while();\n}",
        "div {\n  y: true;\n}\n",
    )
}

fn check(input: &[u8], expected: &str) {
    assert_eq!(
        compile_scss(input, OutputStyle::Expanded)
            .and_then(|s| Ok(String::from_utf8(s)?))
            .unwrap(),
        expected
    );
}
