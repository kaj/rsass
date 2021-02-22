//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues"
#[allow(unused)]
use super::rsass;

mod t47_str_slice;

// From "sass-spec/spec/libsass-closed-issues/issue-2640.hrx"
#[test]
fn issue_2640() {
    assert_eq!(
        rsass(
            ".theme1, .theme2 {\
            \n  .something {\
            \n    /* nothing */\
            \n  }\
            \n}\
            \n\
            \n$sel: selector-nest(\'.theme1, .theme2\', \'.something\');\
            \n  \
            \n#{$sel} {\
            \n  /* nothing */\
            \n}\
            \n"
        )
        .unwrap(),
        ".theme1 .something, .theme2 .something {\
        \n  /* nothing */\
        \n}\
        \n.theme1 .something, .theme2 .something {\
        \n  /* nothing */\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue-2681.hrx"
#[test]
#[ignore] // wrong result
fn issue_2681() {
    assert_eq!(
        rsass(
            "%button-styles {\
            \n  color: red;\
            \n\
            \n  &:focus {\
            \n    color: blue;\
            \n  }\
            \n}\
            \n\
            \n[type=\"button\"] {\
            \n  @extend %button-styles;\
            \n}\
            \n\
            \n"
        )
        .unwrap(),
        "[type=button] {\
        \n  color: red;\
        \n}\
        \n[type=button]:focus {\
        \n  color: blue;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_100.hrx"
#[test]
fn issue_100() {
    assert_eq!(
        rsass(
            "$endColor: red;\r\
            \ntest {\r\
            \n  background-color: darken($endColor, 10%) \\9;\r\
            \n}"
        )
        .unwrap(),
        "test {\
        \n  background-color: #cc0000 \\9 ;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1007.hrx"
#[test]
fn issue_1007() {
    assert_eq!(
        rsass(
            "/* start */ foo /* foo */ baz /* bar */ {\
            \n    /* before */ margin /* X */: /* Y */ 0 /* */; /* after */\
            \n} /* end */"
        )
        .unwrap(),
        "/* start */\
        \nfoo baz {\
        \n  /* before */\
        \n  margin: 0;\
        \n  /* after */\
        \n}\
        \n/* end */\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1016.hrx"
#[test]
fn issue_1016() {
    assert_eq!(
        rsass(
            ".foo {\
            \n  [baz=\"#{&}\"] {\
            \n    foo: bar;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo [baz=\".foo\"] {\
        \n  foo: bar;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1021.hrx"
#[test]
fn issue_1021() {
    assert_eq!(
        rsass(
            "div {\r\
            \n    top: 10px - 2 * 5px /* arrow size */;\r\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  top: 0px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1025.hrx"
#[test]
fn issue_1025() {
    assert_eq!(
        rsass(
            "@mixin m() {\
            \n  .a & {\
            \n    @content;\
            \n  }\
            \n}\
            \n\
            \n:not(:last-of-type) {\
            \n  top: 10px;\
            \n  @include m {\
            \n    top: 10px;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ":not(:last-of-type) {\
        \n  top: 10px;\
        \n}\
        \n.a :not(:last-of-type) {\
        \n  top: 10px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1030.hrx"
#[test]
fn issue_1030() {
    assert_eq!(
        rsass(
            "@mixin will-change() {\
            \n  @supports (will-change: transform) {\
            \n    will-change: transform;\
            \n  }\
            \n}\
            \ndiv {\
            \n  a {\
            \n    top: 10px;\
            \n    @include will-change();\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "div a {\
        \n  top: 10px;\
        \n}\
        \n@supports (will-change: transform) {\
        \n  div a {\
        \n    will-change: transform;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1036.hrx"
#[test]
fn issue_1036() {
    assert_eq!(
        rsass(
            "@mixin all-vip() {\
            \n  test: vip;\
            \n}\
            \n@mixin gold() {\
            \n  test: gold;\
            \n}\
            \n@mixin platinum() {\
            \n  test: platinum;\
            \n}\
            \n\
            \n@mixin icons-sprite($icon-name){\
            \n    @if $icon-name == \'all-vip\' {\
            \n        @include all-vip();\
            \n    }\
            \n    @else if $icon-name == \'gold\' {\
            \n        @include gold();\
            \n    }\
            \n    @else if $icon-name == \'platinum\' {\
            \n        @include platinum();\
            \n    }\
            \n}\
            \n\
            \ndiv {\
            \n  @include icons-sprite(\"platinum\");\
            \n  @include icons-sprite(\"all-vip\");\
            \n  @include icons-sprite(\"gold\");\
            \n}\
            \ndiv {\
            \n  @include icons-sprite(platinum);\
            \n  @include icons-sprite(all-vip);\
            \n  @include icons-sprite(gold);\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  test: platinum;\
        \n  test: vip;\
        \n  test: gold;\
        \n}\
        \ndiv {\
        \n  test: platinum;\
        \n  test: vip;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1043.hrx"
#[test]
#[ignore] // wrong result
fn issue_1043() {
    assert_eq!(
        rsass(
            ".component{\
            \n    color: red;\
            \n    @at-root{\
            \n        #{&}--foo#{&}--bar {\
            \n            color: blue;\
            \n        }\
            \n    }\
            \n}\
            \n\
            \n.test{\
            \n        .selector#{&} {\
            \n            color: blue;\
            \n        }\
            \n}"
        )
        .unwrap(),
        ".component {\
        \n  color: red;\
        \n}\
        \n.component--foo.component--bar {\
        \n  color: blue;\
        \n}\
        \n.test .selector.test {\
        \n  color: blue;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1060.hrx"
#[test]
fn issue_1060() {
    assert_eq!(
        rsass(
            "foo {\
            \n  @if true {\
            \n    foo: true;\
            \n  } @elseif true {\
            \n    foo: false;\
            \n  } @else {\
            \n    foo: false;\
            \n  }\
            \n\
            \n  @if true {\
            \n    bar: true;\
            \n  } @else if true {\
            \n    bar: false;\
            \n  } @else {\
            \n    bar: false;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  foo: true;\
        \n  bar: true;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1061.hrx"
#[test]
fn issue_1061() {
    assert_eq!(
        rsass(
            "a {\
            \n  &.div,\
            \n  &.span {\
            \n    display: block;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "a.div, a.span {\
        \n  display: block;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1063.hrx"
#[test]
#[ignore] // wrong result
fn issue_1063() {
    assert_eq!(
        rsass(
            "%foo {\
            \n  & > x { display: block; }\
            \n}\
            \n\
            \na {\
            \n  > b { @extend %foo; }\
            \n  > b > c { @extend %foo; }\
            \n}\
            \n"
        )
        .unwrap(),
        "a > b > c > x, a > b > x {\
        \n  display: block;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1074.hrx"
#[test]
fn issue_1074() {
    assert_eq!(
        rsass(
            "$i: 1;\
            \n.foo#{-$i} { a:b }\
            \n.foo-#{$i} { a:b }\
            \n.foo#{-1} { a:b }\
            \n.foo-#{1} { a:b }\
            \n"
        )
        .unwrap(),
        ".foo-1 {\
        \n  a: b;\
        \n}\
        \n.foo-1 {\
        \n  a: b;\
        \n}\
        \n.foo-1 {\
        \n  a: b;\
        \n}\
        \n.foo-1 {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1075.hrx"
#[test]
fn issue_1075() {
    assert_eq!(
        rsass(
            "$name: \"lighten\";\
            \n$args: (\"color\": #ff0000, \"amount\": 10%);\
            \nfoo {\
            \n  bar: call($name, $args...);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: #ff3333;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1079.hrx"

// Ignoring "issue_1079", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_108.hrx"
#[test]
fn issue_108() {
    assert_eq!(
        rsass(
            "$a: red;\r\
            \n\r\
            \n@mixin f($a: $a) {\r\
            \n  color: $a;\r\
            \n}\r\
            \n\r\
            \nh1 {\r\
            \n  @include f;\r\
            \n}\r\
            \n\r\
            \nh2 {\r\
            \n  @include f(blue);\r\
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

// From "sass-spec/spec/libsass-closed-issues/issue_1080.hrx"
#[test]
#[ignore] // wrong result
fn issue_1080() {
    assert_eq!(
        rsass(
            "/** comment 1 */\
            \n@import url(\"import-1\");\
            \n/** comment 2 */\
            \n@import url(\"import-2\");\
            \n/** comment 3 */\
            \nfoo { bar: baz; }\
            \n"
        )
        .unwrap(),
        "/** comment 1 */\
        \n@import url(\"import-1\");\
        \n/** comment 2 */\
        \n@import url(\"import-2\");\
        \n/** comment 3 */\
        \nfoo {\
        \n  bar: baz;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1081.hrx"
#[test]
#[ignore] // wrong result
fn issue_1081() {
    assert_eq!(
        rsass(
            "$foo: foo !global !default;\
            \n\
            \ndefault {\
            \n  foo: $foo;\
            \n}\
            \n\
            \n$foo: bar;\
            \n\
            \nafter {\
            \n  @import \"import\";\
            \n  foo: $foo;\
            \n}\
            \n"
        )
        .unwrap(),
        "default {\
        \n  foo: foo;\
        \n}\
        \nafter {\
        \n  foo: bar;\
        \n}\
        \nafter import-before {\
        \n  foo: bar;\
        \n}\
        \nafter import-after {\
        \n  foo: bar;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1082.hrx"
#[test]
fn issue_1082() {
    assert_eq!(
        rsass(
            "@font-face {\
            \n  font-family: \'My Font\';\
            \n  font-style: normal;\
            \n  font-weight: 300;\
            \n  src: local(\'My Font\'), local(\'My-Font\'),\
            \n    /* from http://.... original source of .eot */\
            \n    url(\'my-font.eot?#iefix\') format(\'embedded-opentype\'),\
            \n    /* from http://.... original source of .woff */\
            \n    url(\'my-font.woff\') format(\'woff\'),\
            \n    /* from http://.... original source of .ttf */\
            \n    url(\'my-font.ttf\') format(\'truetype\'),\
            \n    /* from http://.... original source of .svg */\
            \n    url(\'my-font.svg#MyFont\') format(\'svg\');\
            \n}\
            \n"
        )
        .unwrap(),
        "@font-face {\
        \n  font-family: \"My Font\";\
        \n  font-style: normal;\
        \n  font-weight: 300;\
        \n  src: local(\"My Font\"), local(\"My-Font\"), url(\"my-font.eot?#iefix\") format(\"embedded-opentype\"), url(\"my-font.woff\") format(\"woff\"), url(\"my-font.ttf\") format(\"truetype\"), url(\"my-font.svg#MyFont\") format(\"svg\");\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1086.hrx"
#[test]
fn issue_1086() {
    assert_eq!(rsass("$map: (-1px: 12);").unwrap(), "");
}

// From "sass-spec/spec/libsass-closed-issues/issue_1087.hrx"
#[test]
fn issue_1087() {
    assert_eq!(
        rsass(
            "$foo: bar;\
            \na {\
            \n  foo: url($foo);\
            \n  foo: url(#{$foo});\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  foo: url(bar);\
        \n  foo: url(bar);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1091.hrx"
#[test]
#[ignore] // wrong result
fn issue_1091() {
    assert_eq!(
        rsass(
            ".a {\
            \n  top: 0;\
            \n}\
            \n\
            \n.b .c {\
            \n  @extend .a;\
            \n}\
            \n\
            \n.d > .e {\
            \n  @extend .a;\
            \n  @extend .c;\
            \n}\
            \n"
        )
        .unwrap(),
        ".a, .d > .e, .b .c, .b .d > .e {\
        \n  top: 0;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1092.hrx"
#[test]
fn issue_1092() {
    assert_eq!(
        rsass(
            "$bar: \"\";\
            \n$baz: \" \";\
            \na { a: foo #{\"\"}; }\
            \nb { b: foo #{\" \"}; }\
            \nc { c: foo #{$bar}; }\
            \nd { d: foo #{$baz}; }\
            \n"
        )
        .unwrap(),
        "a {\
        \n  a: foo;\
        \n}\
        \nb {\
        \n  b: foo  ;\
        \n}\
        \nc {\
        \n  c: foo;\
        \n}\
        \nd {\
        \n  d: foo  ;\
        \n}\
        \n"
    );
}

mod issue_1093;

// From "sass-spec/spec/libsass-closed-issues/issue_1098.hrx"
#[test]
#[ignore] // wrong result
fn issue_1098() {
    assert_eq!(
        rsass(
            "div {\
            \n opacity: 1\\9;\
            \n width: 500px\\9;\
            \n color: #f00000\\9\\0;\
            \n color: #f00000\\9\\0\\;\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  opacity: 1\\9 ;\
        \n  width: 500px\\9 ;\
        \n  color: #f00000\\9 \\0 ;\
        \n  color: #f00000\\9 \\0 \\;;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1101.hrx"
#[test]
fn issue_1101() {
    assert_eq!(
        rsass(
            "$foo: white;\r\
            \nfoo {\r\
            \n  bar: adjust-color($foo, $hue: -6deg, $lightness: -16%, $saturation: -7%);\r\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  bar: #d6d6d6;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1102.hrx"
#[test]
fn issue_1102() {
    assert_eq!(
        rsass(
            "foo {\
            \n  display:expression(\"inline\",\
            \n    (this.innerHTML += (this.innerHTML.indexOf(\",\") == -1 ? \", \" : \"\")),\
            \n    this.runtimeStyle.display = \"inline\");\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  display: expression(\"inline\", (this.innerHTML += (this.innerHTML.indexOf(\",\") == -1 ? \", \" : \"\")), this.runtimeStyle.display = \"inline\");\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1103.hrx"
#[test]
#[ignore] // wrong result
fn issue_1103() {
    assert_eq!(
        rsass(
            "@import \"import\";\
            \n\
            \n@media screen and (min-width: 1) {\
            \n    foo { bar: baz }\
            \n    baz { bar: foo }\
            \n}\
            \n\
            \n@media screen and (min-width: 1) {\
            \n    @import \"import\";\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: baz;\
        \n}\
        \nbaz {\
        \n  bar: foo;\
        \n}\
        \n@media screen and (max-width: 2) {\
        \n  foo {\
        \n    bar: baz;\
        \n  }\
        \n  baz {\
        \n    bar: foo;\
        \n  }\
        \n}\
        \n@media screen and (min-width: 1) {\
        \n  foo {\
        \n    bar: baz;\
        \n  }\
        \n  baz {\
        \n    bar: foo;\
        \n  }\
        \n}\
        \n@media screen and (min-width: 1) {\
        \n  foo {\
        \n    bar: baz;\
        \n  }\
        \n  baz {\
        \n    bar: foo;\
        \n  }\
        \n}\
        \n@media screen and (min-width: 1) and (max-width: 2) {\
        \n  foo {\
        \n    bar: baz;\
        \n  }\
        \n  baz {\
        \n    bar: foo;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1106.hrx"
#[test]
fn issue_1106() {
    assert_eq!(
        rsass(
            "@function foo() { @return null; }\
            \n$foo: null;\
            \na {\
            \n    foo: bar;\
            \n    variable: $foo;\
            \n    function: foo();\
            \n    unquote: unquote($foo);\
            \n}\
            \n\
            \nb {\
            \n    variable: $foo;\
            \n    function: foo();\
            \n    unquote: unquote($foo);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  foo: bar;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1107.hrx"
#[test]
#[ignore] // wrong result
fn issue_1107() {
    assert_eq!(
        rsass(
            ".foo {\
            \n    filter: progid:DXImageTransform.Microsoft.AlphaImageLoader(\
            \n        src=\"#{foo}\",\
            \n        sizingMethod=\'scale\');\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  filter: progid:DXImageTransform.Microsoft.AlphaImageLoader( src=\"foo\", sizingMethod=\"scale\");\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1115.hrx"
#[test]
fn issue_1115() {
    assert_eq!(
        rsass(
            "foo {\
            \n    bar: \"x\\79\";\
            \n    baz: \"#{x}\\79\";\
            \n    bar: \"x\\a\";\
            \n    baz: \"#{x}\\a\";\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: \"xy\";\
        \n  baz: \"xy\";\
        \n  bar: \"x\\a\";\
        \n  baz: \"x\\a\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_112.hrx"
#[test]
fn issue_112() {
    assert_eq!(
        rsass(
            "@mixin media($var1, $var2) {\r\
            \n  @media screen and ($var1: $var2) {\r\
            \n    @content;\r\
            \n  }\r\
            \n}\r\
            \n\r\
            \n@include media(max-device-width, 500px) {\r\
            \n  foo {\r\
            \n    bar: \"works\";\r\
            \n  }\r\
            \n}"
        )
        .unwrap(),
        "@media screen and (max-device-width: 500px) {\
        \n  foo {\
        \n    bar: \"works\";\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1121.hrx"
#[test]
fn issue_1121() {
    assert_eq!(
        rsass(
            "$foo: \"foo\";\
            \n$bar: \"bar\";\
            \n$baz: \"baz\";\
            \n/*\
            \n * <div class=\"foo #{$foo}\" bar=\"#{$bar}\" baz=\"#{$baz}\">\
            \n */\
            \n"
        )
        .unwrap(),
        "/*\
        \n * <div class=\"foo foo\" bar=\"bar\" baz=\"baz\">\
        \n */\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1127.hrx"
#[test]
fn issue_1127() {
    assert_eq!(
        rsass(
            "$a: to-upper-case(\'abcd\');\
            \n$b: to-upper-case(\"abcd\");\
            \n$c: to-upper-case(abcd);\
            \n\
            \nfoo {\
            \n    content: #{$a};\
            \n    content: #{$b};\
            \n    content: #{$c};\
            \n    content: \'#{$a}\';\
            \n    content: \'#{$b}\';\
            \n    content: \'#{$c}\';\
            \n    content: \"#{$a}\";\
            \n    content: \"#{$b}\";\
            \n    content: \"#{$c}\";\
            \n\
            \n    content: #{unquote($a)};\
            \n    content: #{unquote($b)};\
            \n    content: #{unquote($c)};\
            \n    content: \'#{unquote($a)}\';\
            \n    content: \'#{unquote($b)}\';\
            \n    content: \'#{unquote($c)}\';\
            \n    content: \"#{unquote($a)}\";\
            \n    content: \"#{unquote($b)}\";\
            \n    content: \"#{unquote($c)}\";\
            \n\
            \n    content: #{$a + unquote(\"efg\")};\
            \n    content: #{$b + unquote(\"efg\")};\
            \n    content: #{$c + unquote(\"efg\")};\
            \n    content: \'#{$a + unquote(\"efg\")}\';\
            \n    content: \'#{$b + unquote(\"efg\")}\';\
            \n    content: \'#{$c + unquote(\"efg\")}\';\
            \n    content: \"#{$a + unquote(\"efg\")}\";\
            \n    content: \"#{$b + unquote(\"efg\")}\";\
            \n    content: \"#{$c + unquote(\"efg\")}\";\
            \n\
            \n    content: #{$a + unquote(\"\")};\
            \n    content: #{$b + unquote(\"\")};\
            \n    content: #{$c + unquote(\"\")};\
            \n    content: \'#{$a + unquote(\"\")}\';\
            \n    content: \'#{$b + unquote(\"\")}\';\
            \n    content: \'#{$c + unquote(\"\")}\';\
            \n    content: \"#{$a + unquote(\"\")}\";\
            \n    content: \"#{$b + unquote(\"\")}\";\
            \n    content: \"#{$c + unquote(\"\")}\";\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  content: ABCD;\
        \n  content: ABCD;\
        \n  content: ABCD;\
        \n  content: \"ABCD\";\
        \n  content: \"ABCD\";\
        \n  content: \"ABCD\";\
        \n  content: \"ABCD\";\
        \n  content: \"ABCD\";\
        \n  content: \"ABCD\";\
        \n  content: ABCD;\
        \n  content: ABCD;\
        \n  content: ABCD;\
        \n  content: \"ABCD\";\
        \n  content: \"ABCD\";\
        \n  content: \"ABCD\";\
        \n  content: \"ABCD\";\
        \n  content: \"ABCD\";\
        \n  content: \"ABCD\";\
        \n  content: ABCDefg;\
        \n  content: ABCDefg;\
        \n  content: ABCDefg;\
        \n  content: \"ABCDefg\";\
        \n  content: \"ABCDefg\";\
        \n  content: \"ABCDefg\";\
        \n  content: \"ABCDefg\";\
        \n  content: \"ABCDefg\";\
        \n  content: \"ABCDefg\";\
        \n  content: ABCD;\
        \n  content: ABCD;\
        \n  content: ABCD;\
        \n  content: \"ABCD\";\
        \n  content: \"ABCD\";\
        \n  content: \"ABCD\";\
        \n  content: \"ABCD\";\
        \n  content: \"ABCD\";\
        \n  content: \"ABCD\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_113.hrx"
#[test]
fn issue_113() {
    assert_eq!(
        rsass(
            "// Input\
            \nsection {\
            \n    $w: null, 10px;\
            \n    width: $w;\
            \n}"
        )
        .unwrap(),
        "section {\
        \n  width: 10px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1130.hrx"
#[test]
fn issue_1130() {
    assert_eq!(
        rsass(
            "@function foo($args...) {\
            \n  @return bar($args...);\
            \n}\
            \n\
            \n@function bar() {\
            \n @return \"hi\";\
            \n}\
            \n\
            \n.foo {\
            \n  result: foo();\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  result: \"hi\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1132.hrx"
#[test]
fn issue_1132() {
    assert_eq!(
        rsass(
            "foo {\
            \n  @for $i from 0 through 360 {\
            \n    i#{$i}: hue(hsl($i, 10%, 20%));\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  i0: 0deg;\
        \n  i1: 1deg;\
        \n  i2: 2deg;\
        \n  i3: 3deg;\
        \n  i4: 4deg;\
        \n  i5: 5deg;\
        \n  i6: 6deg;\
        \n  i7: 7deg;\
        \n  i8: 8deg;\
        \n  i9: 9deg;\
        \n  i10: 10deg;\
        \n  i11: 11deg;\
        \n  i12: 12deg;\
        \n  i13: 13deg;\
        \n  i14: 14deg;\
        \n  i15: 15deg;\
        \n  i16: 16deg;\
        \n  i17: 17deg;\
        \n  i18: 18deg;\
        \n  i19: 19deg;\
        \n  i20: 20deg;\
        \n  i21: 21deg;\
        \n  i22: 22deg;\
        \n  i23: 23deg;\
        \n  i24: 24deg;\
        \n  i25: 25deg;\
        \n  i26: 26deg;\
        \n  i27: 27deg;\
        \n  i28: 28deg;\
        \n  i29: 29deg;\
        \n  i30: 30deg;\
        \n  i31: 31deg;\
        \n  i32: 32deg;\
        \n  i33: 33deg;\
        \n  i34: 34deg;\
        \n  i35: 35deg;\
        \n  i36: 36deg;\
        \n  i37: 37deg;\
        \n  i38: 38deg;\
        \n  i39: 39deg;\
        \n  i40: 40deg;\
        \n  i41: 41deg;\
        \n  i42: 42deg;\
        \n  i43: 43deg;\
        \n  i44: 44deg;\
        \n  i45: 45deg;\
        \n  i46: 46deg;\
        \n  i47: 47deg;\
        \n  i48: 48deg;\
        \n  i49: 49deg;\
        \n  i50: 50deg;\
        \n  i51: 51deg;\
        \n  i52: 52deg;\
        \n  i53: 53deg;\
        \n  i54: 54deg;\
        \n  i55: 55deg;\
        \n  i56: 56deg;\
        \n  i57: 57deg;\
        \n  i58: 58deg;\
        \n  i59: 59deg;\
        \n  i60: 60deg;\
        \n  i61: 61deg;\
        \n  i62: 62deg;\
        \n  i63: 63deg;\
        \n  i64: 64deg;\
        \n  i65: 65deg;\
        \n  i66: 66deg;\
        \n  i67: 67deg;\
        \n  i68: 68deg;\
        \n  i69: 69deg;\
        \n  i70: 70deg;\
        \n  i71: 71deg;\
        \n  i72: 72deg;\
        \n  i73: 73deg;\
        \n  i74: 74deg;\
        \n  i75: 75deg;\
        \n  i76: 76deg;\
        \n  i77: 77deg;\
        \n  i78: 78deg;\
        \n  i79: 79deg;\
        \n  i80: 80deg;\
        \n  i81: 81deg;\
        \n  i82: 82deg;\
        \n  i83: 83deg;\
        \n  i84: 84deg;\
        \n  i85: 85deg;\
        \n  i86: 86deg;\
        \n  i87: 87deg;\
        \n  i88: 88deg;\
        \n  i89: 89deg;\
        \n  i90: 90deg;\
        \n  i91: 91deg;\
        \n  i92: 92deg;\
        \n  i93: 93deg;\
        \n  i94: 94deg;\
        \n  i95: 95deg;\
        \n  i96: 96deg;\
        \n  i97: 97deg;\
        \n  i98: 98deg;\
        \n  i99: 99deg;\
        \n  i100: 100deg;\
        \n  i101: 101deg;\
        \n  i102: 102deg;\
        \n  i103: 103deg;\
        \n  i104: 104deg;\
        \n  i105: 105deg;\
        \n  i106: 106deg;\
        \n  i107: 107deg;\
        \n  i108: 108deg;\
        \n  i109: 109deg;\
        \n  i110: 110deg;\
        \n  i111: 111deg;\
        \n  i112: 112deg;\
        \n  i113: 113deg;\
        \n  i114: 114deg;\
        \n  i115: 115deg;\
        \n  i116: 116deg;\
        \n  i117: 117deg;\
        \n  i118: 118deg;\
        \n  i119: 119deg;\
        \n  i120: 120deg;\
        \n  i121: 121deg;\
        \n  i122: 122deg;\
        \n  i123: 123deg;\
        \n  i124: 124deg;\
        \n  i125: 125deg;\
        \n  i126: 126deg;\
        \n  i127: 127deg;\
        \n  i128: 128deg;\
        \n  i129: 129deg;\
        \n  i130: 130deg;\
        \n  i131: 131deg;\
        \n  i132: 132deg;\
        \n  i133: 133deg;\
        \n  i134: 134deg;\
        \n  i135: 135deg;\
        \n  i136: 136deg;\
        \n  i137: 137deg;\
        \n  i138: 138deg;\
        \n  i139: 139deg;\
        \n  i140: 140deg;\
        \n  i141: 141deg;\
        \n  i142: 142deg;\
        \n  i143: 143deg;\
        \n  i144: 144deg;\
        \n  i145: 145deg;\
        \n  i146: 146deg;\
        \n  i147: 147deg;\
        \n  i148: 148deg;\
        \n  i149: 149deg;\
        \n  i150: 150deg;\
        \n  i151: 151deg;\
        \n  i152: 152deg;\
        \n  i153: 153deg;\
        \n  i154: 154deg;\
        \n  i155: 155deg;\
        \n  i156: 156deg;\
        \n  i157: 157deg;\
        \n  i158: 158deg;\
        \n  i159: 159deg;\
        \n  i160: 160deg;\
        \n  i161: 161deg;\
        \n  i162: 162deg;\
        \n  i163: 163deg;\
        \n  i164: 164deg;\
        \n  i165: 165deg;\
        \n  i166: 166deg;\
        \n  i167: 167deg;\
        \n  i168: 168deg;\
        \n  i169: 169deg;\
        \n  i170: 170deg;\
        \n  i171: 171deg;\
        \n  i172: 172deg;\
        \n  i173: 173deg;\
        \n  i174: 174deg;\
        \n  i175: 175deg;\
        \n  i176: 176deg;\
        \n  i177: 177deg;\
        \n  i178: 178deg;\
        \n  i179: 179deg;\
        \n  i180: 180deg;\
        \n  i181: 181deg;\
        \n  i182: 182deg;\
        \n  i183: 183deg;\
        \n  i184: 184deg;\
        \n  i185: 185deg;\
        \n  i186: 186deg;\
        \n  i187: 187deg;\
        \n  i188: 188deg;\
        \n  i189: 189deg;\
        \n  i190: 190deg;\
        \n  i191: 191deg;\
        \n  i192: 192deg;\
        \n  i193: 193deg;\
        \n  i194: 194deg;\
        \n  i195: 195deg;\
        \n  i196: 196deg;\
        \n  i197: 197deg;\
        \n  i198: 198deg;\
        \n  i199: 199deg;\
        \n  i200: 200deg;\
        \n  i201: 201deg;\
        \n  i202: 202deg;\
        \n  i203: 203deg;\
        \n  i204: 204deg;\
        \n  i205: 205deg;\
        \n  i206: 206deg;\
        \n  i207: 207deg;\
        \n  i208: 208deg;\
        \n  i209: 209deg;\
        \n  i210: 210deg;\
        \n  i211: 211deg;\
        \n  i212: 212deg;\
        \n  i213: 213deg;\
        \n  i214: 214deg;\
        \n  i215: 215deg;\
        \n  i216: 216deg;\
        \n  i217: 217deg;\
        \n  i218: 218deg;\
        \n  i219: 219deg;\
        \n  i220: 220deg;\
        \n  i221: 221deg;\
        \n  i222: 222deg;\
        \n  i223: 223deg;\
        \n  i224: 224deg;\
        \n  i225: 225deg;\
        \n  i226: 226deg;\
        \n  i227: 227deg;\
        \n  i228: 228deg;\
        \n  i229: 229deg;\
        \n  i230: 230deg;\
        \n  i231: 231deg;\
        \n  i232: 232deg;\
        \n  i233: 233deg;\
        \n  i234: 234deg;\
        \n  i235: 235deg;\
        \n  i236: 236deg;\
        \n  i237: 237deg;\
        \n  i238: 238deg;\
        \n  i239: 239deg;\
        \n  i240: 240deg;\
        \n  i241: 241deg;\
        \n  i242: 242deg;\
        \n  i243: 243deg;\
        \n  i244: 244deg;\
        \n  i245: 245deg;\
        \n  i246: 246deg;\
        \n  i247: 247deg;\
        \n  i248: 248deg;\
        \n  i249: 249deg;\
        \n  i250: 250deg;\
        \n  i251: 251deg;\
        \n  i252: 252deg;\
        \n  i253: 253deg;\
        \n  i254: 254deg;\
        \n  i255: 255deg;\
        \n  i256: 256deg;\
        \n  i257: 257deg;\
        \n  i258: 258deg;\
        \n  i259: 259deg;\
        \n  i260: 260deg;\
        \n  i261: 261deg;\
        \n  i262: 262deg;\
        \n  i263: 263deg;\
        \n  i264: 264deg;\
        \n  i265: 265deg;\
        \n  i266: 266deg;\
        \n  i267: 267deg;\
        \n  i268: 268deg;\
        \n  i269: 269deg;\
        \n  i270: 270deg;\
        \n  i271: 271deg;\
        \n  i272: 272deg;\
        \n  i273: 273deg;\
        \n  i274: 274deg;\
        \n  i275: 275deg;\
        \n  i276: 276deg;\
        \n  i277: 277deg;\
        \n  i278: 278deg;\
        \n  i279: 279deg;\
        \n  i280: 280deg;\
        \n  i281: 281deg;\
        \n  i282: 282deg;\
        \n  i283: 283deg;\
        \n  i284: 284deg;\
        \n  i285: 285deg;\
        \n  i286: 286deg;\
        \n  i287: 287deg;\
        \n  i288: 288deg;\
        \n  i289: 289deg;\
        \n  i290: 290deg;\
        \n  i291: 291deg;\
        \n  i292: 292deg;\
        \n  i293: 293deg;\
        \n  i294: 294deg;\
        \n  i295: 295deg;\
        \n  i296: 296deg;\
        \n  i297: 297deg;\
        \n  i298: 298deg;\
        \n  i299: 299deg;\
        \n  i300: 300deg;\
        \n  i301: 301deg;\
        \n  i302: 302deg;\
        \n  i303: 303deg;\
        \n  i304: 304deg;\
        \n  i305: 305deg;\
        \n  i306: 306deg;\
        \n  i307: 307deg;\
        \n  i308: 308deg;\
        \n  i309: 309deg;\
        \n  i310: 310deg;\
        \n  i311: 311deg;\
        \n  i312: 312deg;\
        \n  i313: 313deg;\
        \n  i314: 314deg;\
        \n  i315: 315deg;\
        \n  i316: 316deg;\
        \n  i317: 317deg;\
        \n  i318: 318deg;\
        \n  i319: 319deg;\
        \n  i320: 320deg;\
        \n  i321: 321deg;\
        \n  i322: 322deg;\
        \n  i323: 323deg;\
        \n  i324: 324deg;\
        \n  i325: 325deg;\
        \n  i326: 326deg;\
        \n  i327: 327deg;\
        \n  i328: 328deg;\
        \n  i329: 329deg;\
        \n  i330: 330deg;\
        \n  i331: 331deg;\
        \n  i332: 332deg;\
        \n  i333: 333deg;\
        \n  i334: 334deg;\
        \n  i335: 335deg;\
        \n  i336: 336deg;\
        \n  i337: 337deg;\
        \n  i338: 338deg;\
        \n  i339: 339deg;\
        \n  i340: 340deg;\
        \n  i341: 341deg;\
        \n  i342: 342deg;\
        \n  i343: 343deg;\
        \n  i344: 344deg;\
        \n  i345: 345deg;\
        \n  i346: 346deg;\
        \n  i347: 347deg;\
        \n  i348: 348deg;\
        \n  i349: 349deg;\
        \n  i350: 350deg;\
        \n  i351: 351deg;\
        \n  i352: 352deg;\
        \n  i353: 353deg;\
        \n  i354: 354deg;\
        \n  i355: 355deg;\
        \n  i356: 356deg;\
        \n  i357: 357deg;\
        \n  i358: 358deg;\
        \n  i359: 359deg;\
        \n  i360: 0deg;\
        \n}\
        \n"
    );
}

mod issue_1133;

// From "sass-spec/spec/libsass-closed-issues/issue_1153.hrx"
#[test]
fn issue_1153() {
    assert_eq!(
        rsass(
            "/* precision: 0 */\
            \n$foo: 123px;\
            \nfoo {\
            \n  bar: $foo;\
            \n}"
        )
        .unwrap(),
        "/* precision: 0 */\
        \nfoo {\
        \n  bar: 123px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1162.hrx"
#[test]
fn issue_1162() {
    assert_eq!(
        rsass(
            "div {\
            \n  content: #{0/0} a;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  content: 0/0 a;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1163.hrx"
#[test]
fn issue_1163() {
    assert_eq!(
        rsass(
            "div {\
            \n  content: (((92px * 12) / 13px) * 1em) + 22em;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  content: 106.9230769231em;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1167.hrx"
#[test]
fn issue_1167() {
    assert_eq!(
        rsass(
            "a {\
            \n  b: 3s + 101ms;\
            \n}"
        )
        .unwrap(),
        "a {\
        \n  b: 3.101s;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1168.hrx"
#[test]
fn issue_1168() {
    assert_eq!(
        rsass(
            "$namespace: \'test-\';\
            \n$column: 1;\
            \n\
            \n.#{$namespace}#{$column}\\/#{$column} {\
            \n  width: 100% !important;\
            \n}"
        )
        .unwrap(),
        ".test-1\\/1 {\
        \n  width: 100% !important;\
        \n}\
        \n"
    );
}

mod issue_1169;

mod issue_1170;

// From "sass-spec/spec/libsass-closed-issues/issue_1171.hrx"
#[test]
#[ignore] // wrong result
fn issue_1171() {
    assert_eq!(
        rsass(
            "@function foo($initial, $args...) {\
            \n  $args: append($args, 3);\
            \n\
            \n  @return bar($initial, $args...);\
            \n}\
            \n\
            \n@function bar($args...) {\
            \n  @return length($args);\
            \n}\
            \n\
            \n@function baz($initial, $args...) {\
            \n  $args: append($args, 3);\
            \n\
            \n  @return nth($args, 1);\
            \n}\
            \n\
            \n.test {\
            \n  foo: foo(1, 2);\
            \n  baz: baz(1, 2);\
            \n}"
        )
        .unwrap(),
        ".test {\
        \n  foo: 3;\
        \n  baz: 2;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1178.hrx"
#[test]
fn issue_1178() {
    assert_eq!(
        rsass(
            "$foo: ((4, 5), 6, (7 8) 9);\
            \n\
            \nbar {\
            \n  a: $foo;\
            \n  f: 1 2 3 + $foo;\
            \n  b: 1, 2, 3 + (2 ($foo));\
            \n  x: inspect($foo);\
            \n}\
            \n"
        )
        .unwrap(),
        "bar {\
        \n  a: 4, 5, 6, 7 8 9;\
        \n  f: 1 2 34, 5, 6, 7 8 9;\
        \n  b: 1, 2, 32 4, 5, 6, 7 8 9;\
        \n  x: (4, 5), 6, (7 8) 9;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1187.hrx"

// Ignoring "issue_1187", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_1188.hrx"
#[test]
#[ignore] // wrong result
fn issue_1188() {
    assert_eq!(
        rsass(
            "$columns: 4;\
            \n$context: 120px;\
            \n$name-multiplicator: 2;\
            \nfoo {\
            \n  *width: expression((this.parentNode.clientWidth/#{$context}*#{($columns / $name-multiplicator)} - parseInt(this.currentStyle[\'paddingLeft\']) - parseInt(this.currentStyle[\'paddingRight\'])) + \'px\');\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  *width: expression((this.parentNode.clientWidth/120px*2 - parseInt(this.currentStyle[\"paddingLeft\"]) - parseInt(this.currentStyle[\"paddingRight\"])) + \"px\");\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1192"
#[test]
#[ignore] // wrong result
fn issue_1192() {
    assert_eq!(
        rsass(
            "$keyword: foobar;\
            \n\
            \n@mixin test($arglist...){\
            \n  $map: keywords($arglist);\
            \n  /*#{inspect($map)}*/\
            \n  /*#{inspect($arglist)}*/\
            \n}\
            \n\
            \n// Works\
            \n@include test(foo, bar, baz);\
            \n// Ruby Sass:  /*foo, bar, baz*/\
            \n// LibSass  :  /*foo, bar, baz*/\
            \n\
            \n// LibSass does not inspect as ()\
            \n@include test;\
            \n// Ruby Sass:  /*()*/\
            \n// LibSass  :  /**/\
            \n\
            \n// Ruby Sass throws error – LibSass shows keywords in arglist\
            \n// (keywords should not show in arglist – see below)\
            \n@include test(foo, bar, baz, $keyword: keyword);\
            \n// Ruby Sass:  \"Mixin test1 doesn\'t have an argument named $keyword.\"\
            \n// LibSass  :  /*foo, bar, baz, $keyword: keyword*/"
        )
        .unwrap(),
        "/*()*/\
        \n/*foo, bar, baz*/\
        \n/*()*/\
        \n/*()*/\
        \n/*(keyword: keyword)*/\
        \n/*foo, bar, baz*/\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1206.hrx"
#[test]
fn issue_1206() {
    assert_eq!(
        rsass(
            "foo {\
            \n  bar: #{0/0};\
            \n  bar: #{0/1};\
            \n  bar: #{1/2};\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: 0/0;\
        \n  bar: 0/1;\
        \n  bar: 1/2;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1207.hrx"
#[test]
fn issue_1207() {
    assert_eq!(
        rsass(
            "@function test($pos) {\
            \n  @return test-#{$pos};\
            \n}\
            \n\
            \n.foo {\
            \n  content: test(str-slice(\'scale-0\', 7));   // Nope\
            \n  content: test-#{str-slice(\'scale-0\', 7)}; // Yep\
            \n}"
        )
        .unwrap(),
        ".foo {\
        \n  content: test-0;\
        \n  content: test-0;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1208.hrx"
#[test]
fn issue_1208() {
    assert_eq!(
        rsass(
            "foo {\
            \n  &.bar, /* */\
            \n  &.baz {\
            \n    color: red;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "foo.bar, foo.baz {\
        \n  color: red;\
        \n}\
        \n"
    );
}

mod issue_1210;

// From "sass-spec/spec/libsass-closed-issues/issue_1214.hrx"
#[test]
fn issue_1214() {
    assert_eq!(
        rsass(
            "@mixin keyframes($animation-name) {\
            \n  @keyframes $animation-name {\
            \n    @content;\
            \n  }\
            \n}\
            \n\
            \n@include keyframes(bounce) {\
            \n  0%, 20%, 50%, 80%, 100% {transform: translateY(0);}\
            \n  40% {transform: translateY(-30px);}\
            \n  60% {transform: translateY(-15px);}\
            \n}"
        )
        .unwrap(),
        "@keyframes $animation-name {\
        \n  0%, 20%, 50%, 80%, 100% {\
        \n    transform: translateY(0);\
        \n  }\
        \n  40% {\
        \n    transform: translateY(-30px);\
        \n  }\
        \n  60% {\
        \n    transform: translateY(-15px);\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1215.hrx"
#[test]
fn issue_1215() {
    assert_eq!(
        rsass(
            "foo {\
            \n  -quotes: \'this-string\' == \'this-string\';\
            \n  -quotes: this-string == \'this-string\';\
            \n  -quotes: \'this-string\' == \"this-string\";\
            \n  -quotes: \'this-string\' == \'\"this-string\"\';\
            \n  -quotes: \'\"this-string\"\' == \"\'this-string\'\";\
            \n  foo: this-string;\
            \n  foo: \'this-string\';\
            \n  foo: \"this-string\";\
            \n  foo: \'\"this-string\"\';\
            \n  foo: \"\'this-string\'\";\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  -quotes: true;\
        \n  -quotes: true;\
        \n  -quotes: true;\
        \n  -quotes: false;\
        \n  -quotes: false;\
        \n  foo: this-string;\
        \n  foo: \"this-string\";\
        \n  foo: \"this-string\";\
        \n  foo: \'\"this-string\"\';\
        \n  foo: \"\'this-string\'\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1216.hrx"
#[test]
fn issue_1216() {
    assert_eq!(
        rsass(
            "a {\
            \n  width: 4.0px;\
            \n  height: 3.00px;\
            \n  opacity: 1.0;\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  width: 4px;\
        \n  height: 3px;\
        \n  opacity: 1;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1218.hrx"
#[test]
#[ignore] // wrong result
fn issue_1218() {
    assert_eq!(
        rsass(
            "$foo: 20px;\
            \n@media screen and (\"min-width:#{$foo}\") {\
            \n    .bar {\
            \n        width: 12px;\
            \n    }\
            \n}\
            \n@media screen and (\"min-width:0\") {\
            \n    .bar {\
            \n        width: 12px;\
            \n    }\
            \n}\
            \n"
        )
        .unwrap(),
        "@media screen and (min-width:20px) {\
        \n  .bar {\
        \n    width: 12px;\
        \n  }\
        \n}\
        \n@media screen and (min-width:0) {\
        \n  .bar {\
        \n    width: 12px;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1224.hrx"
#[test]
#[ignore] // wrong result
fn issue_1224() {
    assert_eq!(
        rsass(
            "@media all and (max-width: 768px) {\
            \n  @media only screen {\
            \n    a { b: c; }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@media only screen and (max-width: 768px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1230.hrx"
#[test]
fn issue_1230() {
    assert_eq!(
        rsass(
            "div {\
            \n  transition-property:\
            \n    border-color,\
            \n    box-shadow,\
            \n    color;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  transition-property: border-color, box-shadow, color;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1231"
#[test]
fn issue_1231() {
    assert_eq!(
        rsass(
            "div::before {\
            \n  content: #{\"\\\"\"+\\e600+\"\\\"\"};\
            \n}"
        )
        .unwrap(),
        "@charset \"UTF-8\";\
        \ndiv::before {\
        \n  content: \"\u{e600}\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1233.hrx"
#[test]
fn issue_1233() {
    assert_eq!(
        rsass(
            "@-moz-keyframes animatetoptop /* Firefox */ line 429\
            \n{\
            \nfrom {width:0%}\
            \nto {width:100%}\
            \n}"
        )
        .unwrap(),
        "@-moz-keyframes animatetoptop /* Firefox */ line 429 {\
        \n  from {\
        \n    width: 0%;\
        \n  }\
        \n  to {\
        \n    width: 100%;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1240.hrx"
#[test]
fn issue_1240() {
    assert_eq!(
        rsass(
            "$var: 1;\
            \n$list: 2, 3;\
            \n$new-list: append($var, $list);\
            \n$nested-list: $var $list;\
            \n@debug($var);\
            \n@debug($list);\
            \n@debug($new-list);\
            \n@debug($nested-list);\
            \ndiv {\
            \n a: $var;\
            \n a: $list;\
            \n a: $new-list;\
            \n a: $nested-list;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  a: 1;\
        \n  a: 2, 3;\
        \n  a: 1 2, 3;\
        \n  a: 1 2, 3;\
        \n}\
        \n"
    );
}

mod issue_1243;

// From "sass-spec/spec/libsass-closed-issues/issue_1248.hrx"
#[test]
#[ignore] // wrong result
fn issue_1248() {
    assert_eq!(
        rsass(
            ".a.b .c {\
            \n  top: 0;\
            \n}\
            \n.a {\
            \n  @extend .b;\
            \n}\
            \n.a .d {\
            \n  @extend .c;\
            \n}\
            \n"
        )
        .unwrap(),
        ".a.b .c, .a .c, .a .d {\
        \n  top: 0;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1251.hrx"
#[test]
fn issue_1251() {
    assert_eq!(
        rsass(
            ".foo {\
            \n  yellow: yellow;\
            \n  red: red;\
            \n  blue: blue;\
            \n  white: white;\
            \n  black: black;\
            \n  eval: if(red, yellow, null);\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  yellow: yellow;\
        \n  red: red;\
        \n  blue: blue;\
        \n  white: white;\
        \n  black: black;\
        \n  eval: yellow;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1253.hrx"
#[test]
fn issue_1253() {
    assert_eq!(
        rsass(
            "$foo: bar;\
            \n@keyframes $foo {\
            \n  from { a: b }\
            \n  to { a: c }\
            \n}"
        )
        .unwrap(),
        "@keyframes $foo {\
        \n  from {\
        \n    a: b;\
        \n  }\
        \n  to {\
        \n    a: c;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1255.hrx"
#[test]
#[ignore] // wrong result
fn issue_1255() {
    assert_eq!(
        rsass(
            "@function double($value) {\
            \n  @return $value * 2;\
            \n}\
            \n\
            \n@mixin dummy-bug($args...) {\
            \n  @for $i from 1 through length($args) {\
            \n    $args: set-nth($args, $i, double(nth($args, $i)));\
            \n  }\
            \n\
            \n  content: $args;\
            \n}\
            \n\
            \n.foo {\
            \n  @include dummy-bug(1, 2, 3, 4);\
            \n}"
        )
        .unwrap(),
        ".foo {\
        \n  content: 2, 4, 6, 8;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1257.hrx"
#[test]
#[ignore] // wrong result
fn issue_1257() {
    assert_eq!(
        rsass(
            ".foo {\
            \n  color: invert(red...);\
            \n}"
        )
        .unwrap(),
        ".foo {\
        \n  color: aqua;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1258.hrx"
#[test]
fn issue_1258() {
    assert_eq!(
        rsass(
            "$list:   \'(-webkit-min-device-pixel-ratio: 2)\', \'(min-resolution: 192dpi)\';\
            \n$string: \'(-webkit-min-device-pixel-ratio: 2),   (min-resolution: 192dpi)\';\
            \n\
            \n.foo {\
            \n  // I should not unquote a list, I know. But still.\
            \n  content: unquote($list);\
            \n  content: unquote($string);\
            \n}"
        )
        .unwrap(),
        ".foo {\
        \n  content: \"(-webkit-min-device-pixel-ratio: 2)\", \"(min-resolution: 192dpi)\";\
        \n  content: (-webkit-min-device-pixel-ratio: 2),   (min-resolution: 192dpi);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1259.hrx"
#[test]
#[ignore] // wrong result
fn issue_1259() {
    assert_eq!(
        rsass(
            "@mixin dummy($a, $b, $c, $d, $e: true) {\
            \n  content: $a $b $c $d $e;\
            \n}\
            \n\
            \n.foo {\
            \n  @include dummy( (\'a\', \'b\', \'c\', \'e\')..., $e: false );\
            \n}"
        )
        .unwrap(),
        ".foo {\
        \n  content: \"a\" \"b\" \"c\" \"e\" false;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1260.hrx"
#[test]
fn issue_1260() {
    assert_eq!(
        rsass(
            "$EQ-Selectors: ();\
            \n\
            \n.el {\
            \n    $EQ-Selectors: append($EQ-Selectors, &, \'comma\') !global;\
            \n}\
            \n\
            \nhtml:before {\
            \n  content: \"#{$EQ-Selectors}\";\
            \n}"
        )
        .unwrap(),
        "html:before {\
        \n  content: \".el\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1263.hrx"
#[test]
#[ignore] // unexepected error
fn issue_1263() {
    assert_eq!(
        rsass(
            "foo {\
            \n  @ap#{pl}y;\
            \n  @apply(--bar);\
            \n  @apply  (  --bar  );\
            \n  @ap#{pl}y   (   --bar , --foo  )  ;\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  @apply;\
        \n  @apply (--bar);\
        \n  @apply (  --bar  );\
        \n  @apply (   --bar , --foo  );\
        \n}\
        \n"
    );
}

mod issue_1266;

// From "sass-spec/spec/libsass-closed-issues/issue_1269.hrx"
#[test]
fn issue_1269() {
    assert_eq!(
        rsass(
            "@function push($list, $items...) {\
            \n  @return join($list, $items, $separator: auto);\
            \n}\
            \n\
            \n.test {\
            \n  $list: push(1 2 3, 4, 5);\
            \n  list: inspect($list);\
            \n  value: nth($list, 4);\
            \n}"
        )
        .unwrap(),
        ".test {\
        \n  list: 1 2 3 4 5;\
        \n  value: 4;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1271.hrx"
#[test]
fn issue_1271() {
    assert_eq!(
        rsass(
            "$character-code: f102;\
            \n\
            \ntest {\
            \n\
            \n  /* Expected: \"\\f102\" */\
            \n\
            \n  /* Sass 3.4 */\
            \n  content: unquote(\"\\\"\\\\#{$character-code}\\\"\");\
            \n\
            \n  /* Sass 3.3 */\
            \n  content: str-slice(\"\\x\", 1, 1) + $character-code;\
            \n\
            \n}"
        )
        .unwrap(),
        "test {\
        \n  /* Expected: \"\\f102\" */\
        \n  /* Sass 3.4 */\
        \n  content: \"\\f102\";\
        \n  /* Sass 3.3 */\
        \n  content: \"xf102\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1273.hrx"
#[test]
fn issue_1273() {
    assert_eq!(
        rsass(
            "test {\
            \n  src: url(test.eot#{if(true, \'?#{42}\', \'\')});\
            \n}"
        )
        .unwrap(),
        "test {\
        \n  src: url(test.eot?42);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1277.hrx"
#[test]
fn issue_1277() {
    assert_eq!(
        rsass(
            "$foo: foo;\
            \n$bar: bar;\
            \n\
            \n.foo {\
            \n  foo: foo #{$foo}, bar #{$bar};\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  foo: foo foo, bar bar;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1279.hrx"
#[test]
fn issue_1279() {
    assert_eq!(
        rsass(
            "@function noop($string) {\
            \n  @return $string;\
            \n}\
            \n\
            \n.foo {\
            \n  upper: to-upper-case(\'f\') + str-slice(\'foo\', 2);\
            \n  lower: to-lower-case(\'f\') + str-slice(\'foo\', 2);\
            \n  user-upper: to-upper-case(\'f\') + noop(\'oo\');\
            \n  user-lower: to-lower-case(\'f\') + noop(\'oo\');\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  upper: \"Foo\";\
        \n  lower: \"foo\";\
        \n  user-upper: \"Foo\";\
        \n  user-lower: \"foo\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1281.hrx"
#[test]
fn issue_1281() {
    assert_eq!(
        rsass(
            "$quoted: \"green\";\
            \n$unquoted: green;\
            \n\
            \n.test {\
            \n  string: type-of($quoted);\
            \n  color: type-of($unquoted);\
            \n  string: type-of(\"green\");\
            \n  color: type-of(green);\
            \n}\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  string: string;\
        \n  color: color;\
        \n  string: string;\
        \n  color: color;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1283.hrx"
#[test]
#[ignore] // unexepected error
fn issue_1283() {
    assert_eq!(
        rsass(
            "$map: map-merge((1 2: 3), (2 1: 3));\
            \n\
            \n.test {\
            \n  test: inspect($map);\
            \n}\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  test: (1 2: 3, 2 1: 3);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1285.hrx"
#[test]
fn issue_1285() {
    assert_eq!(
        rsass(
            ".container {\
            \n  @for $i from 1 through 3 {\
            \n    @at-root .box-#{$i} {\
            \n      color: darken(red,($i * 5));\
            \n    }\
            \n  }\
            \n\
            \n // Control\
            \n @at-root .outside-child {\
            \n   background-color: blue;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".box-1 {\
        \n  color: #e60000;\
        \n}\
        \n.box-2 {\
        \n  color: #cc0000;\
        \n}\
        \n.box-3 {\
        \n  color: #b30000;\
        \n}\
        \n.outside-child {\
        \n  background-color: blue;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1291.hrx"
#[test]
fn issue_1291() {
    assert_eq!(
        rsass(
            "@mixin spec1($decimal) {\
            \n  $decimal: unquote($decimal) * -1;\
            \n  value: $decimal;\
            \n}\
            \n\
            \n@mixin spec2($decimal) {\
            \n  $decimal: -1 * unquote($decimal);\
            \n  value: $decimal;\
            \n}\
            \n\
            \n@mixin spec3($decimal) {\
            \n  value: #{$decimal * -1};\
            \n}\
            \n\
            \n.my-element {\
            \n  @include spec1(3);\
            \n  @include spec1(-3);\
            \n  @include spec2(5);\
            \n  @include spec2(-5);\
            \n  @include spec3(7);\
            \n  @include spec3(-7);\
            \n}"
        )
        .unwrap(),
        ".my-element {\
        \n  value: -3;\
        \n  value: 3;\
        \n  value: -5;\
        \n  value: 5;\
        \n  value: -7;\
        \n  value: 7;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1294.hrx"
#[test]
fn issue_1294() {
    assert_eq!(
        rsass(
            "/*------------------------------------*\\\
            \n  #BUTTONS\
            \n\\*------------------------------------*/\
            \n\
            \nfoo {\
            \n  display: inline-block; /* [1] */\
            \n}\
            \n"
        )
        .unwrap(),
        "/*------------------------------------*\\\
        \n  #BUTTONS\
        \n\\*------------------------------------*/\
        \nfoo {\
        \n  display: inline-block;\
        \n  /* [1] */\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1295.hrx"
#[test]
fn issue_1295() {
    assert_eq!(
        rsass(
            "foo {\
            \n  $nothing: null;\
            \n  foo: \"#{$nothing}\' %\' \'#{$nothing}\";\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  foo: \"\' %\' \'\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1297.hrx"
#[test]
#[ignore] // wrong result
fn issue_1297() {
    assert_eq!(
        rsass(
            ".test .testa {\
            \n  @at-root #{\"%foo\"} {\
            \n    color: red;\
            \n  }\
            \n  @extend %foo;\
            \n}\
            \n"
        )
        .unwrap(),
        ".test .testa {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1298.hrx"
#[test]
fn issue_1298() {
    assert_eq!(
        rsass(
            "@import url(//fonts.googleapis.com/css?family=Roboto:400,500,700,400italic);\
            \nhtml {\
            \n  font-family: roboto, arial, helvetica, sans-serif;\
            \n}\
            \n"
        )
        .unwrap(),
        "@import url(//fonts.googleapis.com/css?family=Roboto:400,500,700,400italic);\
        \nhtml {\
        \n  font-family: roboto, arial, helvetica, sans-serif;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1301.hrx"
#[test]
fn issue_1301() {
    assert_eq!(
        rsass(
            "$name: \"my-class\";\
            \n\
            \n.-#{$name} {\
            \n  content: \"test\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".-my-class {\
        \n  content: \"test\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1303.hrx"
#[test]
#[ignore] // wrong result
fn issue_1303() {
    assert_eq!(
        rsass(
            ".simple {\
            \n  a: selector-replace(\'foo.bar\', \'foo\', \'foo[baz]\');\
            \n}\
            \n"
        )
        .unwrap(),
        ".simple {\
        \n  a: foo.bar[baz];\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1304.hrx"
#[test]
fn issue_1304() {
    assert_eq!(
        rsass(
            "foo {\
            \n    a:&;\
            \n    > bar {\
            \n        b:&;\
            \n        > baz {\
            \n            c:&;\
            \n        }\
            \n    }\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: foo;\
        \n}\
        \nfoo > bar {\
        \n  b: foo > bar;\
        \n}\
        \nfoo > bar > baz {\
        \n  c: foo > bar > baz;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1305.hrx"
#[test]
fn issue_1305() {
    assert_eq!(
        rsass(
            ".foo {\
            \n    content: call(\'unquote\', \'foo\', ()...);\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  content: foo;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_131.hrx"
#[test]
fn issue_131() {
    assert_eq!(
        rsass(
            "$foo: bar;\r\
            \n\r\
            \ndiv {\r\
            \n    content: \"foo #{$foo}\"\r\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  content: \"foo bar\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1322.hrx"
#[test]
#[ignore] // wrong result
fn issue_1322() {
    assert_eq!(
        rsass(
            "$foo: 400px;\
            \n$bar: \"min-width:400px\";\
            \n@import url(foo.css) (min-width:400px);\
            \n@import url(foo.css) (min-width:$foo);\
            \n@import url(foo.css) (min-width:#{$foo});\
            \n@import url(foo.css) ($bar);\
            \n@import url(foo.css) (#{$bar});\
            \n"
        )
        .unwrap(),
        "@import url(foo.css) (min-width: 400px);\
        \n@import url(foo.css) (min-width: 400px);\
        \n@import url(foo.css) (min-width: 400px);\
        \n@import url(foo.css) (min-width:400px);\
        \n@import url(foo.css) (min-width:400px);\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1323.hrx"
#[test]
fn issue_1323() {
    assert_eq!(
        rsass(
            "@import url(foo.css) only screen;\
            \n@import url(foo.css) (min-width:400px);\
            \n@import url(foo.css) (min-width:400px) and (max-width:599px);\
            \n"
        )
        .unwrap(),
        "@import url(foo.css) only screen;\
        \n@import url(foo.css) (min-width: 400px);\
        \n@import url(foo.css) (min-width: 400px) and (max-width: 599px);\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1328.hrx"
#[test]
#[ignore] // wrong result
fn issue_1328() {
    assert_eq!(
        rsass(
            "#{bar},\
            \n[foo=\"#{bar}\"],\
            \n[foo=\"#{bar}\"] {\
            \n    content: \"foo\";\
            \n}\
            \n"
        )
        .unwrap(),
        "bar,\
        \n[foo=bar],\
        \n[foo=bar] {\
        \n  content: \"foo\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1331.hrx"
#[test]
fn issue_1331() {
    assert_eq!(
        rsass(
            "$m: (foo: 1px, null: 2px, false: 3px, true: 4px);\
            \n\
            \n@debug $m;\
            \n@debug map-get($m, foo);\
            \n@debug map-get($m, null);\
            \n@debug map-get($m, false);\
            \n@debug map-get($m, true);\
            \n"
        )
        .unwrap(),
        ""
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1332.hrx"
#[test]
fn issue_1332() {
    assert_eq!(
        rsass(
            ".box1 {\
            \n    color: rgb(20%, 20%, 20%);\
            \n}\
            \n.box2 {\
            \n    color: rgb(32, 32, 32);\
            \n}\
            \n.box3 {\
            \n    color: rgba(20%, 20%, 20%, 0.7);\
            \n}\
            \n"
        )
        .unwrap(),
        ".box1 {\
        \n  color: #333333;\
        \n}\
        \n.box2 {\
        \n  color: #202020;\
        \n}\
        \n.box3 {\
        \n  color: rgba(51, 51, 51, 0.7);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1333.hrx"
#[test]
fn issue_1333() {
    assert_eq!(
        rsass(
            "@function baz() {\
            \n    @return \'baz\';\
            \n}\
            \n\
            \nfoo {\
            \n    bar: baz()#{\' !important\'};\
            \n    bar: baz() #{\' !important\'};\
            \n}\
            \n\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: \"baz\"  !important;\
        \n  bar: \"baz\"  !important;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1336.hrx"
#[test]
fn issue_1336() {
    assert_eq!(
        rsass(
            "@debug null;\
            \n"
        )
        .unwrap(),
        ""
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1355.hrx"

// Ignoring "issue_1355", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_137.hrx"
#[test]
#[ignore] // wrong result
fn issue_137() {
    assert_eq!(
        rsass(
            ".foo {\
            \n  background-color: lime;\
            \n  a {\
            \n    color: white;\
            \n  }\
            \n}\
            \n\
            \n.baz {\
            \n  @extend .foo;\
            \n}"
        )
        .unwrap(),
        ".foo, .baz {\
        \n  background-color: lime;\
        \n}\
        \n.foo a, .baz a {\
        \n  color: white;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1370.hrx"
#[test]
fn issue_1370() {
    assert_eq!(
        rsass(
            "@mixin ico-common($imgUrl){\r\
            \n    display: inline-block;\r\
            \n    background: url(i/$imgUrl);\r\
            \n    background-repeat: no-repeat;\r\
            \n}\r\
            \n\r\
            \n@mixin ico-size($width,$height){\r\
            \n    width: $width;\r\
            \n    height: $height;\r\
            \n}\r\
            \n\r\
            \n.test{\r\
            \n    @include ico-common(\"icon.png\");\r\
            \n\r\
            \n    @include ico-size(100px, 100px);\r\
            \n}"
        )
        .unwrap(),
        ".test {\
        \n  display: inline-block;\
        \n  background: url(i/\"icon.png\");\
        \n  background-repeat: no-repeat;\
        \n  width: 100px;\
        \n  height: 100px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1376.hrx"
#[test]
fn issue_1376() {
    assert_eq!(
        rsass(
            ".div{\
            \n  $foo: 1, null, 2, null, 3;\
            \n\
            \n  content: \"#{$foo}\";\
            \n}"
        )
        .unwrap(),
        ".div {\
        \n  content: \"1, 2, 3\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1393.hrx"
#[test]
fn issue_1393() {
    assert_eq!(
        rsass(
            "div {\
            \n  back#{ground}: {\
            \n    imag#{e}: url(foo.png);\
            \n    pos#{it}ion: 50%;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  background-image: url(foo.png);\
        \n  background-position: 50%;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1394.hrx"
#[test]
fn issue_1394() {
    assert_eq!(
        rsass(
            "foo {\
            \n  width: \\10 + \\20 \\ ;\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  width: \\10 \\ \\ ;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1396.hrx"
#[test]
fn issue_1396() {
    assert_eq!(
        rsass(
            "foo {\
            \n  foo: foo\"bar\"#{baz};\
            \n  foo: foo\"bar\"baz;\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  foo: foo \"bar\" baz;\
        \n  foo: foo \"bar\" baz;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1398.hrx"
#[test]
fn issue_1398() {
    assert_eq!(
        rsass(
            "@media screen and (hux: 3/4) {\
            \n  foo {\
            \n    bar: baz;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@media screen and (hux: 3/4) {\
        \n  foo {\
        \n    bar: baz;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1399.hrx"
#[test]
fn issue_1399() {
    assert_eq!(
        rsass(
            "foo {\
            \n  foo: 3 - \"bar\";\
            \n  foo: (3 - \"bar\");\
            \n  foo: 3 / \"bar\";\
            \n  foo: (3 / \"bar\");\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  foo: 3-\"bar\";\
        \n  foo: 3-\"bar\";\
        \n  foo: 3/\"bar\";\
        \n  foo: 3/\"bar\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1404.hrx"
#[test]
#[ignore] // wrong result
fn issue_1404() {
    assert_eq!(
        rsass(
            ".test {\r\
            \n    color: #aaabbb--1-2-a;\r\
            \n    color: type-of(#aaabbb--1-2-a);\r\
            \n    color: type-of(#aaabbb--1-2);\r\
            \n}"
        )
        .unwrap(),
        ".test {\
        \n  color: #aaabbb--1-2-a;\
        \n  color: string;\
        \n  color: string;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1405.hrx"
#[test]
#[ignore] // wrong result
fn issue_1405() {
    assert_eq!(
        rsass(
            "div {\r\
            \n  foo: (1a2b3c);\r\
            \n\r\
            \n  length-1: length(1a2b3c);\r\
            \n\r\
            \n  unit-1: unit(1a2b3c);\r\
            \n\r\
            \n  result-1: 1em-.75em;\r\
            \n  result-2: 2em-1em;\r\
            \n  result-3: 2em-0.75em;\r\
            \n  result-4: 1.5em-1em;\r\
            \n  result-5: 2em-1.5em;\r\
            \n\r\
            \n  type-1: type-of(1em-.75em);\r\
            \n  type-2: type-of(2em-1em);\r\
            \n  type-3: type-of(2em-0.75em);\r\
            \n  type-4: type-of(1.5em-1em);\r\
            \n  type-5: type-of(2em-1.5em);\r\
            \n  type-6: type-of(1a2b3c);\r\
            \n\r\
            \n  test-1: (1-em-2-em);\r\
            \n  test-1: (1-em - 2-em);\r\
            \n\r\
            \n  test-2: (1-0-em-2-0-em);\r\
            \n  test-2: (1-0-em - 2-0-em);\r\
            \n\r\
            \n  test-3: (1-A-em-2-A-em);\r\
            \n  test-3: (1-A-em - 2-A-em);\r\
            \n\r\
            \n  test-4: (1_em--_--e-2_em--_--e);\r\
            \n  test-4: (1_em--_--e - 2_em--_--e);\r\
            \n\r\
            \n  test-5: (1_em--_--e0-2_em--_--e0);\r\
            \n  test-5: (1_em--_--e0 - 2_em--_--e0);\r\
            \n\r\
            \n  test-6: (1_em--_--e0__-2_em--_--e0__);\r\
            \n  test-6: (1_em--_--e0__ - 2_em--_--e0__);\r\
            \n\r\
            \n  test-7: (1\\65 _em--_--e0-2\\65 _em--_--e0);\r\
            \n  test-7: (1\\65 _em--_--e0 - 2\\65 _em--_--e0);\r\
            \n}\r\
            \n"
        )
        .unwrap(),
        "div {\
        \n  foo: 1a2b3c;\
        \n  length-1: 1;\
        \n  unit-1: \"a2b3c\";\
        \n  result-1: 0.25em;\
        \n  result-2: 1em;\
        \n  result-3: 1.25em;\
        \n  result-4: 0.5em;\
        \n  result-5: 0.5em;\
        \n  type-1: number;\
        \n  type-2: number;\
        \n  type-3: number;\
        \n  type-4: number;\
        \n  type-5: number;\
        \n  type-6: number;\
        \n  test-1: -1-em;\
        \n  test-1: -1-em;\
        \n  test-2: -1-em;\
        \n  test-2: -1-em;\
        \n  test-3: -1-A-em;\
        \n  test-3: -1-A-em;\
        \n  test-4: -1_em--_--e;\
        \n  test-4: -1_em--_--e;\
        \n  test-5: -1_em--_--e0;\
        \n  test-5: -1_em--_--e0;\
        \n  test-6: -1_em--_--e0__;\
        \n  test-6: -1_em--_--e0__;\
        \n  test-7: -1e_em--_--e0;\
        \n  test-7: -1e_em--_--e0;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1413.hrx"
#[test]
fn issue_1413() {
    assert_eq!(
        rsass(
            "div {\r\
            \n    foo: \'A\'#{B};\r\
            \n    foo: #{A}\'B\';\r\
            \n    foo: \'A\'#{B}\'C\';\r\
            \n    foo: #{A}\'B\'#{C};\r\
            \n    foo: A#{B}\'C\';\r\
            \n    foo: \'A\'#{B}C;\r\
            \n    foo: #{A}B\'C\';\r\
            \n    foo: \'A\'#{B}C\'D\';\r\
            \n    foo: \'A\'B#{C}D\'E\';\r\
            \n    foo: A\'B\'#{C}D\'E\';\r\
            \n    foo: #{A}\'B\'C\'D\'\'E\';\r\
            \n}\r\
            \n\r\
            \ndiv {\r\
            \n    foo: type-of(\'A\'#{B});\r\
            \n    foo: type-of(#{A}\'B\');\r\
            \n    foo: type-of(\'A\'#{B}\'C\');\r\
            \n    foo: type-of(#{A}\'B\'#{C});\r\
            \n    foo: type-of(A#{B}\'C\');\r\
            \n    foo: type-of(\'A\'#{B}C);\r\
            \n    foo: type-of(#{A}B\'C\');\r\
            \n    foo: type-of(\'A\'#{B}C\'D\');\r\
            \n    foo: type-of(\'A\'B#{C}D\'E\');\r\
            \n    foo: type-of(A\'B\'#{C}D\'E\');\r\
            \n    foo: type-of(#{A}\'B\'C\'D\'\'E\');\r\
            \n}\r\
            \n\r\
            \ndiv {\r\
            \n    foo: length(\'A\'#{B});\r\
            \n    foo: length(#{A}\'B\');\r\
            \n    foo: length(\'A\'#{B}\'C\');\r\
            \n    foo: length(#{A}\'B\'#{C});\r\
            \n    foo: length(A#{B}\'C\');\r\
            \n    foo: length(\'A\'#{B}C);\r\
            \n    foo: length(#{A}B\'C\');\r\
            \n    foo: length(\'A\'#{B}C\'D\');\r\
            \n    foo: length(\'A\'B#{C}D\'E\');\r\
            \n    foo: length(A\'B\'#{C}D\'E\');\r\
            \n    foo: length(#{A}\'B\'C\'D\'\'E\');\r\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  foo: \"A\" B;\
        \n  foo: A \"B\";\
        \n  foo: \"A\" B \"C\";\
        \n  foo: A \"B\" C;\
        \n  foo: AB \"C\";\
        \n  foo: \"A\" BC;\
        \n  foo: AB \"C\";\
        \n  foo: \"A\" BC \"D\";\
        \n  foo: \"A\" BCD \"E\";\
        \n  foo: A \"B\" CD \"E\";\
        \n  foo: A \"B\" C \"D\" \"E\";\
        \n}\
        \ndiv {\
        \n  foo: list;\
        \n  foo: list;\
        \n  foo: list;\
        \n  foo: list;\
        \n  foo: list;\
        \n  foo: list;\
        \n  foo: list;\
        \n  foo: list;\
        \n  foo: list;\
        \n  foo: list;\
        \n  foo: list;\
        \n}\
        \ndiv {\
        \n  foo: 2;\
        \n  foo: 2;\
        \n  foo: 3;\
        \n  foo: 3;\
        \n  foo: 2;\
        \n  foo: 2;\
        \n  foo: 2;\
        \n  foo: 3;\
        \n  foo: 3;\
        \n  foo: 4;\
        \n  foo: 5;\
        \n}\
        \n"
    );
}

mod issue_1415;

// From "sass-spec/spec/libsass-closed-issues/issue_1417.hrx"
#[test]
#[ignore] // wrong result
fn issue_1417() {
    assert_eq!(
        rsass(
            "@function foo($a, $b) {\
            \n  @return ($a $b);\
            \n}\
            \n\
            \nfoo {\
            \n  foo: 1px / 2px;\
            \n  foo: 1px / round(1.5);\
            \n  foo: foo(1px / 2px, 1px / round(1.5));\
            \n  foo: missing(1px / 2px, 1px / round(1.5));\
            \n  foo: call(missing, 1px / 2px, 1px / round(1.5));\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  foo: 1px/2px;\
        \n  foo: 0.5px;\
        \n  foo: 0.5 0.5px;\
        \n  foo: missing(1px/2px, 0.5px);\
        \n  foo: missing(1px/2px, 0.5px);\
        \n}\
        \n"
    );
}

mod issue_1418;

mod issue_1419;

// From "sass-spec/spec/libsass-closed-issues/issue_1422.hrx"
#[test]
#[ignore] // wrong result
fn issue_1422() {
    assert_eq!(
        rsass(
            ".foo {\
            \n  /*foo*/foo/*foo*/: /*foo*/bar/*foo*/;\
            \n  /*foo*/ foo /*foo*/ : /*foo*/ bar /*foo*/;\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  /*foo*/\
        \n  foo/*foo*/: bar;\
        \n  /*foo*/\
        \n  foo: bar;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_143.hrx"
#[test]
fn issue_143() {
    assert_eq!(
        rsass(
            "$path: \"images\";\
            \n$file: \"kittens.jpg\";\
            \n$image: \"\";\
            \n$other: file_join(\"images\", \"kittens.jpg\");\
            \n\
            \n@if $image != none {\
            \n\t$image: url(file_join($path, $file));\
            \n}\
            \nbody {\
            \n\tbackground: $image;\
            \n\tcolor: $other;\
            \n}\
            \n"
        )
        .unwrap(),
        "body {\
        \n  background: url(file_join(\"images\", \"kittens.jpg\"));\
        \n  color: file_join(\"images\", \"kittens.jpg\");\
        \n}\
        \n"
    );
}

mod issue_1432;

// From "sass-spec/spec/libsass-closed-issues/issue_1434.hrx"
#[test]
fn issue_1434() {
    assert_eq!(
        rsass(
            ".foo {\
            \n    a: selector-nest(\'.foo\', \'.bar > .baz\');\
            \n    b: selector-nest(\'.foo\', \'.bar ~ .baz\');\
            \n    c: selector-nest(\'.foo\', \'.bar + .baz\');\
            \n    d: selector-nest(\'.foo > .bar\', \'.baz\');\
            \n    e: selector-nest(\'.foo ~ .bar\', \'.baz\');\
            \n    f: selector-nest(\'.foo + .bar\', \'.baz\');\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  a: .foo .bar > .baz;\
        \n  b: .foo .bar ~ .baz;\
        \n  c: .foo .bar + .baz;\
        \n  d: .foo > .bar .baz;\
        \n  e: .foo ~ .bar .baz;\
        \n  f: .foo + .bar .baz;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1437.hrx"
#[test]
fn issue_1437() {
    assert_eq!(
        rsass(
            "div {\r\
            \n\r\
            \n  @media screen and (min-width: 37.5em) {\r\
            \n    /* asd */\r\
            \n  }\r\
            \n\r\
            \n  @media screen and (min-width: 48em) {\r\
            \n    display: none;\r\
            \n  }\r\
            \n}"
        )
        .unwrap(),
        "@media screen and (min-width: 37.5em) {\
        \n  div {\
        \n    /* asd */\
        \n  }\
        \n}\
        \n@media screen and (min-width: 48em) {\
        \n  div {\
        \n    display: none;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1438.hrx"
#[test]
fn issue_1438() {
    assert_eq!(
        rsass(
            "@function foo() {\
            \n  @return 20150812;\
            \n}\
            \n\
            \nfoo {\
            \n  background-image: url(../test.png);\
            \n}\
            \n\
            \nbar {\
            \n  background-image: url(../test.png?v=20150812);\
            \n}\
            \n\
            \nbaz {\
            \n  background-image: url(../test.png?v=#{test()});\
            \n}\
            \n\
            \nbam {\
            \n  background-image: url(\"../test.png?v=#{test()}\");\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  background-image: url(../test.png);\
        \n}\
        \nbar {\
        \n  background-image: url(../test.png?v=20150812);\
        \n}\
        \nbaz {\
        \n  background-image: url(../test.png?v=test());\
        \n}\
        \nbam {\
        \n  background-image: url(\"../test.png?v=test()\");\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1440.hrx"
#[test]
fn issue_1440() {
    assert_eq!(
        rsass(
            "// works fine with plain @each directive\r\
            \n$i: 1;\r\
            \n$prop1: width;\r\
            \n$prop2: background-position;\r\
            \n$values: 132px,\r\
            \n    100px \"-100px -25px\",\r\
            \n    200px \"-500px -100px\";\r\
            \n\r\
            \n@each $value1, $value2 in $values{\r\
            \n  .okay#{$i} {\r\
            \n    #{$prop1}: #{$value1};\r\
            \n    #{$prop2}: #{$value2};\r\
            \n  }\r\
            \n  $i: ($i + 1);\r\
            \n}\r\
            \n\r\
            \n// when using @each inside @mixin with variable arguments($values...),\r\
            \n// $value2 is missing and no errors while compiling\r\
            \n@mixin eachProp($prop1, $prop2, $values...){\r\
            \n  $i: 1;\r\
            \n  @each $value1, $value2 in $values{\r\
            \n    .error#{$i} {\r\
            \n      #{$prop1}: #{$value1};\r\
            \n      #{$prop2}: #{$value2};\r\
            \n    }\r\
            \n    $i: ($i + 1);\r\
            \n  }\r\
            \n}\r\
            \n\r\
            \n@include eachProp($prop1, $prop2,\r\
            \n  132px,\r\
            \n    100px \"-100px -25px\",\r\
            \n    200px \"-500px -100px\"\r\
            \n);"
        )
        .unwrap(),
        ".okay1 {\
        \n  width: 132px;\
        \n}\
        \n.okay2 {\
        \n  width: 100px;\
        \n  background-position: -100px -25px;\
        \n}\
        \n.okay3 {\
        \n  width: 200px;\
        \n  background-position: -500px -100px;\
        \n}\
        \n.error1 {\
        \n  width: 132px;\
        \n}\
        \n.error2 {\
        \n  width: 100px;\
        \n  background-position: -100px -25px;\
        \n}\
        \n.error3 {\
        \n  width: 200px;\
        \n  background-position: -500px -100px;\
        \n}\
        \n"
    );
}

mod issue_1441;

// From "sass-spec/spec/libsass-closed-issues/issue_1448.hrx"
#[test]
fn issue_1448() {
    assert_eq!(
        rsass(
            ".md-card\r\
            \n{\r\
            \n    .md-info-card-highlight\r\
            \n    {\r\
            \n        background: red;\r\
            \n        color: blue;\r\
            \n\r\
            \n        .ng-md-icon\r\
            \n        {\r\
            \n            color: green;\r\
            \n        }\r\
            \n    }\r\
            \n}"
        )
        .unwrap(),
        ".md-card .md-info-card-highlight {\
        \n  background: red;\
        \n  color: blue;\
        \n}\
        \n.md-card .md-info-card-highlight .ng-md-icon {\
        \n  color: green;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1452.hrx"

// Ignoring "issue_1452", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_1459.hrx"
#[test]
fn issue_1459() {
    assert_eq!(
        rsass(
            "@font-face {\r\
            \n  font-family: \"Font Name\";\r\
            \n  src: local(\"Arial\");\r\
            \n  unicode-range: U+270C;\r\
            \n}"
        )
        .unwrap(),
        "@font-face {\
        \n  font-family: \"Font Name\";\
        \n  src: local(\"Arial\");\
        \n  unicode-range: U+270C;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1482.hrx"
#[test]
#[ignore] // wrong result
fn issue_1482() {
    assert_eq!(
        rsass(
            ".mango {\
            \n  color: red;\
            \n}\
            \n\
            \ntype {\
            \n  &__else {\
            \n    @extend .mango;\
            \n  }\
            \n}\
            \n\
            \n.qualified {\
            \n  &__else {\
            \n    @extend .mango;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".mango, .qualified__else, type__else {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1484.hrx"

// Ignoring "issue_1484", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_1486.hrx"
#[test]
fn issue_1486() {
    assert_eq!(
        rsass(
            "$a: 41px;\
            \n\
            \n@function a() {\
            \n  @return 42px;\
            \n}\
            \n\
            \nfoo {\
            \n  foo: $a -121px;\
            \n  foo: ($a -122px);\
            \n  foo: $a*3-123px;\
            \n  foo: ($a*3-124px);\
            \n  foo: $a*3 -123px;\
            \n  foo: ($a*3 -124px);\
            \n  foo: $a*3 - 123px;\
            \n  foo: ($a*3 - 124px);\
            \n  foo: $a*3- 123px;\
            \n  foo: ($a*3- 124px);\
            \n  foo: $a*3- 123px;\
            \n  foo: ($a*3- 124px);\
            \n}\
            \n\
            \nbar {\
            \n  bar: a() -121px;\
            \n  bar: (a() -122px);\
            \n  bar: a()*3-123px;\
            \n  bar: (a()*3-124px);\
            \n  bar: a()*3 -123px;\
            \n  bar: (a()*3 -124px);\
            \n  bar: a()*3 - 123px;\
            \n  bar: (a()*3 - 124px);\
            \n  bar: a()*3- 123px;\
            \n  bar: (a()*3- 124px);\
            \n  bar: a()*3- 123px;\
            \n  bar: (a()*3- 124px);\
            \n}\
            \n\
            \nbaz {\
            \n  baz: 43px -121px;\
            \n  baz: (43px -122px);\
            \n  baz: 43px*3-123px;\
            \n  baz: (43px*3-124px);\
            \n  baz: 43px*3 -123px;\
            \n  baz: (43px*3 -124px);\
            \n  baz: 43px*3 - 123px;\
            \n  baz: (43px*3 - 124px);\
            \n  baz: 43px*3- 123px;\
            \n  baz: (43px*3- 124px);\
            \n  baz: 43px*3- 123px;\
            \n  baz: (43px*3- 124px);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  foo: 41px -121px;\
        \n  foo: 41px -122px;\
        \n  foo: 0px;\
        \n  foo: -1px;\
        \n  foo: 123px -123px;\
        \n  foo: 123px -124px;\
        \n  foo: 0px;\
        \n  foo: -1px;\
        \n  foo: 0px;\
        \n  foo: -1px;\
        \n  foo: 0px;\
        \n  foo: -1px;\
        \n}\
        \nbar {\
        \n  bar: 42px -121px;\
        \n  bar: 42px -122px;\
        \n  bar: 3px;\
        \n  bar: 2px;\
        \n  bar: 126px -123px;\
        \n  bar: 126px -124px;\
        \n  bar: 3px;\
        \n  bar: 2px;\
        \n  bar: 3px;\
        \n  bar: 2px;\
        \n  bar: 3px;\
        \n  bar: 2px;\
        \n}\
        \nbaz {\
        \n  baz: 43px -121px;\
        \n  baz: 43px -122px;\
        \n  baz: 6px;\
        \n  baz: 5px;\
        \n  baz: 129px -123px;\
        \n  baz: 129px -124px;\
        \n  baz: 6px;\
        \n  baz: 5px;\
        \n  baz: 6px;\
        \n  baz: 5px;\
        \n  baz: 6px;\
        \n  baz: 5px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1487.hrx"

// Ignoring "issue_1487", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_1488.hrx"
#[test]
#[ignore] // wrong result
fn issue_1488() {
    assert_eq!(
        rsass(
            "@function foo($arg2) {\
            \n  @return type-of($arg2);\
            \n}\
            \n\
            \n@function foo_($arg2...) {\
            \n  @return type-of($arg2);\
            \n}\
            \n\
            \n@function bar($arg1, $arg2) {\
            \n  @return type-of($arg1) + \"::\" + type-of($arg2);\
            \n}\
            \n\
            \n@function bar_($arg1, $arg2...) {\
            \n  @return type-of($arg1) + \"::\" + type-of($arg2);\
            \n}\
            \n\
            \nfoo {\
            \n  foo: foo(one);\
            \n  foo: foo(one...);\
            \n  bar: bar(one, two);\
            \n  bar: bar(one, two...);\
            \n  foo: call(\'foo\', one);\
            \n  foo: call(\'foo\', one...);\
            \n  bar: call(\'bar\', one, two);\
            \n  bar: call(\'bar\', one, two...);\
            \n}\
            \n\
            \nbar {\
            \n  foo: foo_(one);\
            \n  foo: foo_(one...);\
            \n  bar: bar_(one, two);\
            \n  bar: bar_(one, two...);\
            \n  foo: call(\'foo_\', one);\
            \n  foo: call(\'foo_\', one...);\
            \n  bar: call(\'bar_\', one, two);\
            \n  bar: call(\'bar_\', one, two...);\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  foo: string;\
        \n  foo: string;\
        \n  bar: string::string;\
        \n  bar: string::string;\
        \n  foo: string;\
        \n  foo: string;\
        \n  bar: string::string;\
        \n  bar: string::string;\
        \n}\
        \nbar {\
        \n  foo: arglist;\
        \n  foo: arglist;\
        \n  bar: string::arglist;\
        \n  bar: string::arglist;\
        \n  foo: arglist;\
        \n  foo: arglist;\
        \n  bar: string::arglist;\
        \n  bar: string::arglist;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_151.hrx"

// Ignoring "issue_151", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_152.hrx"

// Ignoring "issue_152", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_1526.hrx"
#[test]
#[ignore] // wrong result
fn issue_1526() {
    assert_eq!(
        rsass(
            "foo {\
            \n  bar: (1--em-2--em);\
            \n  baz: (1--em - 2--em);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: 1 --em-2--em;\
        \n  baz: 1 --em-2 --em;\
        \n}\
        \n"
    );
}

mod issue_1527;

// From "sass-spec/spec/libsass-closed-issues/issue_1535.hrx"
#[test]
#[ignore] // wrong result
fn issue_1535() {
    assert_eq!(
        rsass(
            "foo {\
            \n    test: type-of(1--em);\
            \n    test: (1--em-2--em);\
            \n    test: (1--em- 2--em);\
            \n    test: (1--em -2--em);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  test: list;\
        \n  test: 1 --em-2--em;\
        \n  test: 1 --em- 2 --em;\
        \n  test: 1 --em -2 --em;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1537.hrx"

// Ignoring "issue_1537", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_154.hrx"
#[test]
fn issue_154() {
    assert_eq!(
        rsass(
            "test {\r\
            \n  filter:alpha(opacity=75);\r\
            \n}"
        )
        .unwrap(),
        "test {\
        \n  filter: alpha(opacity=75);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1546.hrx"

mod issue_1550;

// From "sass-spec/spec/libsass-closed-issues/issue_1557.hrx"
#[test]
#[ignore] // wrong result
fn issue_1557() {
    assert_eq!(
        rsass(
            "$xs-break: 30em;@media ALL AND (max-width: $xs-break) {header {display: none;}}\
            \n"
        )
        .unwrap(),
        "@media ALL and (max-width: 30em) {\
        \n  header {\
        \n    display: none;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1566.hrx"
#[test]
#[ignore] // wrong result
fn issue_1566() {
    assert_eq!(
        rsass(
            "@function foo($predicate) {\
            \n  @return call(\'bar\', $predicate);\
            \n}\
            \n\
            \n@function bar($predicate) {\
            \n  @return type-of($predicate);\
            \n}\
            \n\
            \ntest {\
            \n  test: foo(1 2 3);\
            \n}\
            \n"
        )
        .unwrap(),
        "test {\
        \n  test: list;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1567.hrx"
#[test]
#[ignore] // unexepected error
fn issue_1567() {
    assert_eq!(
        rsass(
            "/* any */@media/* first */\
            \n/* screen */screen /*something */ , /* else */\
            \n/* not */not/* print */print /* final */ {  /* whatever */\
            \n    body { line-height: 1.2 }\
            \n}\
            \n"
        )
        .unwrap(),
        "/* any */\
        \n@media screen, not print {\
        \n  /* whatever */\
        \n  body {\
        \n    line-height: 1.2;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1568.hrx"
#[test]
fn issue_1568() {
    assert_eq!(
        rsass(
            "body {\
            \n    font-weight: bold; // test\
            \n    font-size: 10px // test\
            \n}\
            \n"
        )
        .unwrap(),
        "body {\
        \n  font-weight: bold;\
        \n  font-size: 10px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1569.hrx"

// Ignoring "issue_1569", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_1570.hrx"
#[test]
fn issue_1570() {
    assert_eq!(
        rsass(
            "a {\
            \n    font: 12px/normal serif;\
            \n}\
            \n\
            \nb {\
            \n    font: normal 12px/normal serif;\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  font: 12px/normal serif;\
        \n}\
        \nb {\
        \n  font: normal 12px/normal serif;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1574.hrx"
#[test]
#[ignore] // wrong result
fn issue_1574() {
    assert_eq!(
        rsass(
            ".foo {\
            \n  bar: baz;\
            \n}\
            \n\
            \ninput[type=\"text\"],\
            \ninput[type=\"search\"],\
            \ninput[type=\"url\"],\
            \ninput[type=\"email\"],\
            \ninput[type=\"password\"],\
            \ninput[type=\"number\"],\
            \ninput[type=\"tel\"],\
            \ninput[type=\"date\"],\
            \ninput[type=\"range\"],\
            \ntextarea {\
            \n  @extend .foo;\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo, input[type=text],\
        \ninput[type=search],\
        \ninput[type=url],\
        \ninput[type=email],\
        \ninput[type=password],\
        \ninput[type=number],\
        \ninput[type=tel],\
        \ninput[type=date],\
        \ninput[type=range],\
        \ntextarea {\
        \n  bar: baz;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1577.hrx"

// Ignoring "issue_1577", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_1578.hrx"

// From "sass-spec/spec/libsass-closed-issues/issue_1579.hrx"
#[test]
#[ignore] // wrong result
fn issue_1579() {
    assert_eq!(
        rsass(
            "@function foo($a, $b: null, $c: false) {\
            \n  @return $c;\
            \n}\
            \n\
            \n@function bar($args...) {\
            \n  @return call(foo, $args...);\
            \n}\
            \n\
            \ntest {\
            \n  test: bar(3, $c: true);\
            \n}\
            \n"
        )
        .unwrap(),
        "test {\
        \n  test: true;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1583.hrx"
#[test]
#[ignore] // wrong result
fn issue_1583() {
    assert_eq!(
        rsass(
            "$ls: ((foo,));\
            \n\
            \nfoo {\
            \n  baz: length($ls);\
            \n  baz: type-of($ls);\
            \n  baz: inspect($ls);\
            \n}\
            \n\
            \nbar {\
            \n  baz: length(&);\
            \n  baz: type-of(&);\
            \n  baz: inspect(&);\
            \n}\
            \n\
            \nfoo {\
            \n  string: inspect(&);\
            \n  str-length: str-length(inspect(&));\
            \n  list-length: length(&);\
            \n}\
            \n\
            \nfoo, bar {\
            \n  string: inspect(&);\
            \n  str-length: str-length(inspect(&));\
            \n  list-length: length(&);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  baz: 1;\
        \n  baz: list;\
        \n  baz: (foo,);\
        \n}\
        \nbar {\
        \n  baz: 1;\
        \n  baz: list;\
        \n  baz: (bar,);\
        \n}\
        \nfoo {\
        \n  string: (foo,);\
        \n  str-length: 6;\
        \n  list-length: 1;\
        \n}\
        \nfoo, bar {\
        \n  string: foo, bar;\
        \n  str-length: 8;\
        \n  list-length: 2;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1584.hrx"
#[test]
fn issue_1584() {
    assert_eq!(
        rsass(
            "@mixin foo($out: false) {\
            \n  @if $out {\
            \n    @at-root { @content; }\
            \n  }\
            \n}\
            \n\
            \n@mixin bar() {\
            \n  @at-root { @content; }\
            \n}\
            \n\
            \n@mixin baz($string) {\
            \n  @at-root .#{$string} { @content; }\
            \n}\
            \n\
            \n.test {\
            \n  @include foo(true) {\
            \n    .nest1 {\
            \n      color: red;\
            \n    }\
            \n  }\
            \n  @include bar() {\
            \n    .nest2 {\
            \n      color: green;\
            \n    }\
            \n  }\
            \n  @include baz(\'nest3\') {\
            \n    color: blue;\
            \n  }\
            \n  @at-root {\
            \n    .nest4 {\
            \n      color: yellow;\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".nest1 {\
        \n  color: red;\
        \n}\
        \n.nest2 {\
        \n  color: green;\
        \n}\
        \n.nest3 {\
        \n  color: blue;\
        \n}\
        \n.nest4 {\
        \n  color: yellow;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1585.hrx"

// Ignoring "issue_1585", error tests are not supported yet.

mod issue_1590;

// From "sass-spec/spec/libsass-closed-issues/issue_1596.hrx"
#[test]
#[ignore] // wrong result
fn issue_1596() {
    assert_eq!(
        rsass(
            "@document url(http://www.w3.org/),\
            \n               url-prefix(http://www.w3.org/Style/),\
            \n               domain(mozilla.org),\
            \n               regexp(\"https:.*\");\
            \n"
        )
        .unwrap(),
        "@document url(http://www.w3.org/),\
        \n               url-prefix(http://www.w3.org/Style/),\
        \n               domain(mozilla.org),\
        \n               regexp(\"https:.*\");\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1601.hrx"

// Ignoring "issue_1601", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_1604.hrx"
#[test]
#[ignore] // wrong result
fn issue_1604() {
    assert_eq!(
        rsass(
            "@function test($args...) {\
            \n  $all: ();\
            \n\
            \n  @each $arg in $args {\
            \n    $all: append($all, $arg);\
            \n  }\
            \n\
            \n  @return inspect($all);\
            \n}\
            \n\
            \ntest {\
            \n  args-1: test(1 2 3);\
            \n  args-2: test(1 2, 3 4);\
            \n  args-3: test(1, 2, 3);\
            \n}\
            \n"
        )
        .unwrap(),
        "test {\
        \n  args-1: (1 2 3);\
        \n  args-2: (1 2) (3 4);\
        \n  args-3: 1 2 3;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1610.hrx"
#[test]
fn issue_1610() {
    assert_eq!(
        rsass(
            "@function foo() {\
            \n  @return \"bar\";\
            \n}\
            \n\
            \n@function bar() {\
            \n    @return \"foo\" + \",\" + bar;\
            \n}\
            \n\
            \na {\
            \n  b: foo(), \"bar\";\
            \n  b: foo(), \"bar\"\
            \n}\
            \n\
            \nb {\
            \n  b: #{foo(), \"bar\"};\
            \n  b: #{foo(), \"bar\"}\
            \n}\
            \n\
            \nc {\
            \n  b: \"foo\", bar;\
            \n}\
            \n\
            \nd {\
            \n  b: #{\"foo\", bar};\
            \n  b: #{\"foo\", bar}\
            \n}\
            \n\
            \ne {\
            \n  b: #{bar()};\
            \n  b: #{bar()}\
            \n}\
            \n\
            \nf {\
            \n  b: \"foo\" + \",\" + bar;\
            \n  b: \"foo\" + \",\" + bar\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: \"bar\", \"bar\";\
        \n  b: \"bar\", \"bar\";\
        \n}\
        \nb {\
        \n  b: bar, bar;\
        \n  b: bar, bar;\
        \n}\
        \nc {\
        \n  b: \"foo\", bar;\
        \n}\
        \nd {\
        \n  b: foo, bar;\
        \n  b: foo, bar;\
        \n}\
        \ne {\
        \n  b: foo,bar;\
        \n  b: foo,bar;\
        \n}\
        \nf {\
        \n  b: \"foo,bar\";\
        \n  b: \"foo,bar\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1612.hrx"
#[test]
fn issue_1612() {
    assert_eq!(
        rsass(
            "c {\
            \n  b: \"foo\", bar;\
            \n  b: \"foo\", bar\
            \n}\
            \n"
        )
        .unwrap(),
        "c {\
        \n  b: \"foo\", bar;\
        \n  b: \"foo\", bar;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1622.hrx"
#[test]
#[ignore] // wrong result
fn issue_1622() {
    assert_eq!(
        rsass(
            "@function foo($list) {\
            \n    @return call(bar, $list);\
            \n}\
            \n\
            \n@function bar($list, $args...) {\
            \n    @return length($list);\
            \n}\
            \n\
            \ntest {\
            \n  test: foo(1 2 3);\
            \n}\
            \n"
        )
        .unwrap(),
        "test {\
        \n  test: 3;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1624.hrx"
#[test]
fn issue_1624() {
    assert_eq!(
        rsass(
            "@function foo($foo) {\
            \n  @return $foo;\
            \n}\
            \n\
            \n@function data($foo) {\
            \n  @return \'[data-\' + $foo + \']\';\
            \n}\
            \n\
            \n#{foo(foo)} {\
            \n  #{data(\'bar\')} {\
            \n    baz: bam;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "foo [data-bar] {\
        \n  baz: bam;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1629.hrx"
#[test]
fn issue_1629() {
    assert_eq!(
        rsass(
            "foo {\
            \n  background: url(...) 2rem 3rem / auto 2rem;\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  background: url(...) 2rem 3rem/auto 2rem;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1632.hrx"
#[test]
fn issue_1632() {
    assert_eq!(
        rsass(
            "$foo: \\/ !global;\
            \n.foo#{$foo}bar { a: b; }\
            \n"
        )
        .unwrap(),
        ".foo\\/bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1634.hrx"
#[test]
fn issue_1634() {
    assert_eq!(
        rsass(
            "$empty-list: ();\
            \n\
            \n@function foo($args...) {\
            \n    @return call(bar, $args...);\
            \n}\
            \n\
            \n@function bar($list) {\
            \n    @return length($list);\
            \n}\
            \n\
            \ntest {\
            \n  test: foo($empty-list);\
            \n}"
        )
        .unwrap(),
        "test {\
        \n  test: 0;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1640.hrx"
#[test]
#[ignore] // unexepected error
fn issue_1640() {
    assert_eq!(
        rsass(
            "@mixin foo() {\
            \n    @if false {\
            \n      a { b: c }\
            \n    } @else {\
            \n      @content;\
            \n    }\
            \n}\
            \n\
            \n@include foo() {\
            \n  .foo {\
            \n    bar: baz;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  bar: baz;\
        \n}\
        \n"
    );
}

mod issue_1644;

// From "sass-spec/spec/libsass-closed-issues/issue_1645.hrx"
#[test]
#[ignore] // wrong result
fn issue_1645() {
    assert_eq!(
        rsass(
            "@function foo($a, $should-be-empty...) {\
            \n  @return length($should-be-empty);\
            \n}\
            \n\
            \n@function bar($args...) {\
            \n  @return call(foo, $args...);\
            \n}\
            \n\
            \n@function args($args...) {\
            \n  @return $args;\
            \n}\
            \n\
            \n$a: args(1, 2, 3);\
            \n\
            \ntest {\
            \n  test: bar($a);\
            \n}\
            \n"
        )
        .unwrap(),
        "test {\
        \n  test: 0;\
        \n}\
        \n"
    );
}

mod issue_1647;

// From "sass-spec/spec/libsass-closed-issues/issue_1648.hrx"
#[test]
#[ignore] // wrong result
fn issue_1648() {
    assert_eq!(
        rsass(
            "$x: 3px;\
            \n/* comment 1 */\
            \n@if/* pre 1 */$x == 3px/* post 1 */{\
            \n    /* if 1 */\
            \n}\
            \n/* comment 2 */\
            \n@elseif/* pre 2 */$x == 2px/* post 2 */{\
            \n    /* else if 2 */\
            \n}\
            \n/* comment 3 */\
            \n@else/* middle 3 */if/* pre 3  */$x == 3px/* post 3 */{\
            \n    /* else if 3 */\
            \n}\
            \n/* comment 4 */\
            \n@else/* post 4 */{\
            \n    /* else 4 */\
            \n}\
            \n/* comment 5 */"
        )
        .unwrap(),
        "/* comment 1 */\
        \n/* if 1 */\
        \n/* comment 5 */\
        \n"
    );
}

mod issue_1650;

mod issue_1651;

mod issue_1654;

// From "sass-spec/spec/libsass-closed-issues/issue_1658.hrx"

// Ignoring "issue_1658", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_1667.hrx"
#[test]
fn issue_1667() {
    assert_eq!(
        rsass(
            "$map: (\r\
            \n  1: 1,\r\
            \n  1px: 1px\r\
            \n);\r\
            \n\r\
            \nfoo {\r\
            \n  a: map-get($map, 1);\r\
            \n  b: map-get($map, 1px);\r\
            \n}\r\
            \n\r\
            \n$type-scale: (\r\
            \n\t-15:0.066667rem,\r\
            \n\t-10:0.186rem,\r\
            \n\t-9:0.211rem,\r\
            \n\t-8:0.26rem,\r\
            \n\t-7:0.295rem,\r\
            \n\t-6:0.364rem,\r\
            \n\t-5:0.413rem,\r\
            \n\t-4:0.51rem,\r\
            \n\t-3:0.578rem,\r\
            \n\t-2:0.714rem,\r\
            \n\t-1:0.809rem,\r\
            \n\t0:1rem,\r\
            \n\t1:1.133rem,\r\
            \n\t2:1.4rem,\r\
            \n\t3:1.586rem,\r\
            \n\t4:1.96rem,\r\
            \n\t5:2.221rem,\r\
            \n\t6:2.744rem,\r\
            \n\t7:3.109rem,\r\
            \n\t8:3.842rem,\r\
            \n\t9:4.353rem,\r\
            \n\t10:5.378rem,\r\
            \n\t11:6.094rem,\r\
            \n\t12:7.53rem,\r\
            \n\t13:8.531rem,\r\
            \n\t14:10.541rem,\r\
            \n\t15:11.943rem,\r\
            \n\t16:14.758rem\r\
            \n);\r\
            \n\r\
            \n@function get-size($size) {\r\
            \n\t@if map-has-key($type-scale, $size) {\r\
            \n\t\t@return map-get($type-scale, $size);\r\
            \n\t}\r\
            \n\r\
            \n\t@warn \"Not a valid size.\";\r\
            \n\t@return null;\r\
            \n}\r\
            \n\r\
            \n@function scale-size($rem-size, $steps) {\r\
            \n\t$size-key: get-key-for-value($type-scale, $rem-size);\r\
            \n\r\
            \n\t@if $size-key {\r\
            \n\t\t$new-size: $size-key + $steps;\r\
            \n\t\t@return get-size($new-size);\r\
            \n\t}\r\
            \n\r\
            \n\t@warn \"Not able to find size for \" + $rem-size;\r\
            \n\t@return null;\r\
            \n}\r\
            \n\r\
            \n@function get-key-for-value($map, $value) {\r\
            \n\t@each $map-key, $map-value in $map {\r\
            \n\t\t@if $map-value == $value {\r\
            \n\t\t\t@return $map-key\r\
            \n\t\t}\r\
            \n\t}\r\
            \n\r\
            \n\t@warn $value + \" not found in \" + $map;\r\
            \n\t@return null;\r\
            \n}\r\
            \n\r\
            \n$h1-font-size: get-size(5);\r\
            \n\r\
            \n\r\
            \nh1 {\r\
            \n  font-size: $h1-font-size;\r\
            \n\r\
            \n  small {\r\
            \n    font-size: scale-size($h1-font-size, -2);\r\
            \n    color: red;\r\
            \n  }\r\
            \n}\r\
            \n\r\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: 1;\
        \n  b: 1px;\
        \n}\
        \nh1 {\
        \n  font-size: 2.221rem;\
        \n}\
        \nh1 small {\
        \n  font-size: 1.586rem;\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1669.hrx"
#[test]
fn issue_1669() {
    assert_eq!(
        rsass(
            "foo {\
            \n  bar: #{100%/3}\
            \n}\
            \n\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: 100%/3;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_167.hrx"
#[test]
#[ignore] // wrong result
fn issue_167() {
    assert_eq!(
        rsass(
            "%l-cell, .l-cell {\r\
            \n  margin: 0 auto;\r\
            \n  max-width: 1000px;\r\
            \n}"
        )
        .unwrap(),
        ".l-cell {\
        \n  margin: 0 auto;\
        \n  max-width: 1000px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1670.hrx"

// Ignoring "issue_1670", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_1671.hrx"
#[test]
fn issue_1671() {
    assert_eq!(
        rsass(
            "$foo: 5px;\
            \na {\
            \n    background: url(\'img.png\') no-repeat 6px 0 / #{$foo};\
            \n    background: url(\'img.png\') no-repeat 6px 1 / #{$foo};\
            \n    background: url(\'img.png\') no-repeat 6px 1px / #{$foo};\
            \n    background: url(\'img.png\') no-repeat 6px #{$foo} / 0;\
            \n    background: url(\'img.png\') no-repeat 6px #{$foo} / 1;\
            \n    background: url(\'img.png\') no-repeat 6px #{$foo} / 1px;\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  background: url(\"img.png\") no-repeat 6px 0/5px;\
        \n  background: url(\"img.png\") no-repeat 6px 1/5px;\
        \n  background: url(\"img.png\") no-repeat 6px 1px/5px;\
        \n  background: url(\"img.png\") no-repeat 6px 5px/0;\
        \n  background: url(\"img.png\") no-repeat 6px 5px/1;\
        \n  background: url(\"img.png\") no-repeat 6px 5px/1px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1672.hrx"
#[test]
fn issue_1672() {
    assert_eq!(
        rsass(
            "$breakpoint: \'tablet\';\
            \n\
            \n.-#{$breakpoint} {\
            \n  color: #FFF;\
            \n}"
        )
        .unwrap(),
        ".-tablet {\
        \n  color: #FFF;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1673.hrx"
#[test]
#[ignore] // wrong result
fn issue_1673() {
    assert_eq!(
        rsass(
            "%foo {\
            \n  test: outer;\
            \n\
            \n  &-inner {\
            \n    test: inner;\
            \n  }\
            \n}\
            \n\
            \n.foo {\
            \n  @extend %foo;\
            \n\
            \n  &.inner { @extend %foo-inner; }\
            \n}"
        )
        .unwrap(),
        ".foo {\
        \n  test: outer;\
        \n}\
        \n.foo.inner {\
        \n  test: inner;\
        \n}\
        \n"
    );
}

mod issue_1681;

mod issue_1683;

// From "sass-spec/spec/libsass-closed-issues/issue_1685.hrx"
#[test]
fn issue_1685() {
    assert_eq!(
        rsass(
            "@function foo($x, $y...) { @return null }\
            \n\
            \na {\
            \n  b: foo(1 2 3...);\
            \n}"
        )
        .unwrap(),
        ""
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1706.hrx"
#[test]
fn issue_1706() {
    assert_eq!(
        rsass(
            "@function calc($e) { @return custom; }\
            \n@function -foo-calc($e) { @return custom; }\
            \n\
            \n.test {\
            \n    a: calc(1px * 1%);\
            \n    b: -foo-calc(2px * 2%);\
            \n    c: call(calc, 3px * 3%);\
            \n}\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  a: calc(1px * 1%);\
        \n  b: -foo-calc(2px * 2%);\
        \n  c: custom;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1709.hrx"
#[test]
#[ignore] // wrong result
fn issue_1709() {
    assert_eq!(
        rsass(
            "@mixin transition( $prefix_properties, $transitions... ) {\
            \n\
            \n    @if not str-index( inspect( $transitions ), \',\') {\
            \n        $transitions: ( $transitions );\
            \n    }\
            \n\
            \n    @each $prefix in -webkit-, -moz-, -ms-, -o-, \'\' {\
            \n\
            \n        $prefixed: \'\';\
            \n\
            \n        @each $transition in $transitions {\
            \n\
            \n            @if $prefix_properties and \'\' != $prefix {\
            \n                $prefixed: #{$prefix}$transition,$transition;\
            \n            } @else {\
            \n                $prefixed: $transition;\
            \n            }\
            \n\
            \n\
            \n        }\
            \n\
            \n        #{$prefix}transition: $prefixed;\
            \n    }\
            \n}\
            \n\
            \n.my-element {\
            \n    @include transition( true, transform 0.25s linear );\
            \n}\
            \n"
        )
        .unwrap(),
        ".my-element {\
        \n  -webkit-transition: -webkit- transform 0.25s linear, transform 0.25s linear;\
        \n  -moz-transition: -moz- transform 0.25s linear, transform 0.25s linear;\
        \n  -ms-transition: -ms- transform 0.25s linear, transform 0.25s linear;\
        \n  -o-transition: -o- transform 0.25s linear, transform 0.25s linear;\
        \n  transition: transform 0.25s linear;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1710.hrx"
#[test]
#[ignore] // wrong result
fn issue_1710() {
    assert_eq!(
        rsass(
            "ul, ol {\
            \n    & & {\
            \n      display: block;\
            \n    }\
            \n  }\
            \n"
        )
        .unwrap(),
        "ul ul, ul ol, ol ul, ol ol {\
        \n  display: block;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1715.hrx"

// Ignoring "issue_1715", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_1720.hrx"

// Ignoring "issue_1720", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_1722.hrx"
#[test]
fn issue_1722() {
    assert_eq!(
        rsass(
            "$score: (item-height: 1.12em);\
            \n.test {\
            \n    background-position: 0 -#{map-get($score, item-height)};\
            \n}\
            \n\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  background-position: 0 -1.12em;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1723.hrx"
#[test]
#[ignore] // wrong result
fn issue_1723() {
    assert_eq!(
        rsass(
            "test-1 test-2 test-3 test-4 test-5,\r\
            \ntest-6 test-7 test-8 test-9 test-10 {\r\
            \n    @each $set in & {\r\
            \n        set: inspect($set);\r\
            \n\r\
            \n        @each $selector in $set {\r\
            \n            selector: inspect($selector);\r\
            \n        }\r\
            \n    }\r\
            \n}\r\
            \n\r\
            \ntest-1 test-2 test-3 test-4 test-5,\r\
            \ntest-6 test-7 test-8 test-9 test-10 {\r\
            \n    @for $i from 1 through length(&) {\r\
            \n        $set: nth(&, $i);\r\
            \n        set: inspect($set);\r\
            \n\r\
            \n        @each $selector in $set {\r\
            \n            selector: inspect($selector);\r\
            \n        }\r\
            \n    }\r\
            \n}"
        )
        .unwrap(),
        "test-1 test-2 test-3 test-4 test-5,\
        \ntest-6 test-7 test-8 test-9 test-10 {\
        \n  set: test-1 test-2 test-3 test-4 test-5;\
        \n  selector: test-1;\
        \n  selector: test-2;\
        \n  selector: test-3;\
        \n  selector: test-4;\
        \n  selector: test-5;\
        \n  set: test-6 test-7 test-8 test-9 test-10;\
        \n  selector: test-6;\
        \n  selector: test-7;\
        \n  selector: test-8;\
        \n  selector: test-9;\
        \n  selector: test-10;\
        \n}\
        \ntest-1 test-2 test-3 test-4 test-5,\
        \ntest-6 test-7 test-8 test-9 test-10 {\
        \n  set: test-1 test-2 test-3 test-4 test-5;\
        \n  selector: test-1;\
        \n  selector: test-2;\
        \n  selector: test-3;\
        \n  selector: test-4;\
        \n  selector: test-5;\
        \n  set: test-6 test-7 test-8 test-9 test-10;\
        \n  selector: test-6;\
        \n  selector: test-7;\
        \n  selector: test-8;\
        \n  selector: test-9;\
        \n  selector: test-10;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1726.hrx"
#[test]
fn issue_1726() {
    assert_eq!(
        rsass(
            "item {\
            \n    background: #{2px} 2px /*red*/;\
            \n}\
            \n"
        )
        .unwrap(),
        "item {\
        \n  background: 2px 2px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1729.hrx"
#[test]
#[ignore] // wrong result
fn issue_1729() {
    assert_eq!(
        rsass(
            "%place-to-go {\
            \n  font-size: 1em;\
            \n}\
            \n\
            \na::foo(1){\
            \n  @extend %place-to-go;\
            \n}\
            \na::foo(2){\
            \n  @extend %place-to-go;\
            \n}\
            \nb::foo(1){\
            \n  @extend %place-to-go;\
            \n}\
            \nb::foo(2){\
            \n  @extend %place-to-go;\
            \n}\
            \n\
            \n:bar(1){\
            \n  @extend %place-to-go;\
            \n}\
            \n:bar(2){\
            \n  @extend %place-to-go;\
            \n}\
            \n:bar(3){\
            \n  @extend %place-to-go;\
            \n}\
            \n\
            \n[foo]{\
            \n  @extend %place-to-go;\
            \n}\
            \n[bar]{\
            \n  @extend %place-to-go;\
            \n}\
            \n[baz]{\
            \n  @extend %place-to-go;\
            \n}\
            \n\
            \n[bar=\"1\"]{\
            \n  @extend %place-to-go;\
            \n}\
            \n[bar=\"2\"]{\
            \n  @extend %place-to-go;\
            \n}\
            \n[bar=\"3\"]{\
            \n  @extend %place-to-go;\
            \n}\
            \n"
        )
        .unwrap(),
        "[bar=\"3\"], [bar=\"2\"], [bar=\"1\"], [baz], [bar], [foo], :bar(3), :bar(2), :bar(1), b::foo(2), b::foo(1), a::foo(2), a::foo(1) {\
        \n  font-size: 1em;\
        \n}\
        \n"
    );
}

mod issue_1732;

// From "sass-spec/spec/libsass-closed-issues/issue_1733.hrx"
#[test]
fn issue_1733() {
    assert_eq!(
        rsass(
            "foo {\
            \n  a: #ff6600;\
            \n  b: #ff6600\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: #ff6600;\
        \n  b: #ff6600;\
        \n}\
        \n"
    );
}

mod issue_1739;

// From "sass-spec/spec/libsass-closed-issues/issue_1741.hrx"
#[test]
#[ignore] // wrong result
fn issue_1741() {
    assert_eq!(
        rsass(
            ".header {\r\
            \n  .nav-text-link:not(&.popover-link) {\r\
            \n    margin: 10px;\r\
            \n  }\r\
            \n}"
        )
        .unwrap(),
        ".nav-text-link:not(.header.popover-link) {\
        \n  margin: 10px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1752.hrx"
#[test]
fn issue_1752() {
    assert_eq!(
        rsass(
            "@mixin some-fn($args...) {\
            \n  @each $item in $args {\
            \n    @debug($item);\
            \n  }\
            \n}\
            \n\
            \nfoo {\
            \n  @include some-fn(());\
            \n}"
        )
        .unwrap(),
        ""
    );
}

mod issue_1757;

// From "sass-spec/spec/libsass-closed-issues/issue_1765.hrx"
#[test]
fn issue_1765() {
    assert_eq!(
        rsass(
            "foo {\
            \n  bar: 20px /* height */ + 2*5px /* padding */ + 2*1px /*border*/;\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  bar: 32px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1766.hrx"
#[test]
#[ignore] // unexepected error
fn issue_1766() {
    assert_eq!(
        rsass(
            "@media all { @import \"foo.scss\" }\
            \n@media all { @import \"foo.scss\"; }\
            \n"
        )
        .unwrap(),
        "@media all {\
        \n  foo {\
        \n    bar: baz;\
        \n  }\
        \n}\
        \n@media all {\
        \n  foo {\
        \n    bar: baz;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1768.hrx"
#[test]
fn issue_1768() {
    assert_eq!(
        rsass(
            "@debug(());\
            \n@debug(foo, (), bar);\
            \n@debug(foo () bar);\
            \n@debug((foo: (), bar: baz));"
        )
        .unwrap(),
        ""
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1770.hrx"
#[test]
fn issue_1770() {
    assert_eq!(
        rsass(
            "@function returns-string() {\
            \n  @return \"selector\";\
            \n}\
            \n\
            \n#{\"selector\"} {\
            \n  color: red;\
            \n}\
            \n\
            \n#{returns-string()} {\
            \n  color: red;\
            \n}\
            \n\
            \n#{\"selector\"} selector2 {\
            \n  color: red;\
            \n}\
            \n\
            \n#{returns-string()} selector2 {\
            \n  color: red;\
            \n}"
        )
        .unwrap(),
        "selector {\
        \n  color: red;\
        \n}\
        \nselector {\
        \n  color: red;\
        \n}\
        \nselector selector2 {\
        \n  color: red;\
        \n}\
        \nselector selector2 {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1776.hrx"
#[test]
fn issue_1776() {
    assert_eq!(
        rsass(
            "h1 {\r\
            \n  width :calc(100% - 110px);\r\
            \n}"
        )
        .unwrap(),
        "h1 {\
        \n  width: calc(100% - 110px);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1786"
#[test]
#[ignore] // wrong result
fn issue_1786() {
    assert_eq!(
        rsass(
            "$input: \"\\0_\\a_\\A\";\
            \n\
            \ntest {\
            \n    bug1: \"#{\"_\\a\" + b}\";\
            \n    bug2: \"#{a $input}\";\
            \n}\
            \n"
        )
        .unwrap(),
        "@charset \"UTF-8\";\
        \ntest {\
        \n  bug1: \"_\\a b\";\
        \n  bug2: \"a �_ _ \";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1792.hrx"
#[test]
#[ignore] // wrong result
fn issue_1792() {
    assert_eq!(
        rsass(
            "test {\
            \n  test1: (3px*4in) / 1in;\
            \n  test2: ((1px*2in) + (3px*4in)) / 1in;\
            \n}"
        )
        .unwrap(),
        "test {\
        \n  test1: 0.125in;\
        \n  test2: 0.1458333333in;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1793.hrx"

// Ignoring "issue_1793", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_1794.hrx"
#[test]
#[ignore] // wrong result
fn issue_1794() {
    assert_eq!(
        rsass(
            "@media (max-width /*comment*/ : 500px) {\
            \n  foo { bar: baz; }\
            \n}"
        )
        .unwrap(),
        "@media (max-width: 500px) {\
        \n  foo {\
        \n    bar: baz;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1796.hrx"
#[test]
fn issue_1796() {
    assert_eq!(
        rsass(
            ".parent {\
            \n    .brother, .sister, .cousin {\
            \n        color: green;\
            \n        sel: &;\
            \n\
            \n        $new-sel: ();\
            \n        @each $s in & {\
            \n            $last: nth($s, -1);\
            \n            $new-sel: append($new-sel, $s #{\'+\'} $last, comma);\
            \n            x: $new-sel;\
            \n        }\
            \n        @at-root #{$new-sel} {\
            \n            debug: foo;\
            \n        }\
            \n    }\
            \n}"
        )
        .unwrap(),
        ".parent .brother, .parent .sister, .parent .cousin {\
        \n  color: green;\
        \n  sel: .parent .brother, .parent .sister, .parent .cousin;\
        \n  x: .parent .brother + .brother;\
        \n  x: .parent .brother + .brother, .parent .sister + .sister;\
        \n  x: .parent .brother + .brother, .parent .sister + .sister, .parent .cousin + .cousin;\
        \n}\
        \n.parent .brother + .brother, .parent .sister + .sister, .parent .cousin + .cousin {\
        \n  debug: foo;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1797.hrx"
#[test]
#[ignore] // wrong result
fn issue_1797() {
    assert_eq!(
        rsass(
            "%not {\
            \n  color: red;\
            \n}\
            \n.not {\
            \n  @extend %not;\
            \n}\
            \ndiv:has(%not) {\
            \n  color: black;\
            \n}\
            \n\
            \nbar {\
            \n  span:not(%not) {\
            \n    color: black;\
            \n  }\
            \n  span:not(&.foo) {\
            \n    color: black;\
            \n  }\
            \n  span:not(&%not) {\
            \n    color: black;\
            \n  }\
            \n}"
        )
        .unwrap(),
        ".not {\
        \n  color: red;\
        \n}\
        \ndiv:has(.not) {\
        \n  color: black;\
        \n}\
        \nbar span:not(.not) {\
        \n  color: black;\
        \n}\
        \nspan:not(bar.foo) {\
        \n  color: black;\
        \n}\
        \nspan:not(bar.not) {\
        \n  color: black;\
        \n}\
        \n"
    );
}

mod issue_1798;

mod issue_1801;

mod issue_1803;

mod issue_1804;

// From "sass-spec/spec/libsass-closed-issues/issue_1812.hrx"
#[test]
fn issue_1812() {
    assert_eq!(
        rsass(
            "@svg-load test url(foo.svg) {\r\
            \n  fill: red;\r\
            \n}\r\
            \n\r\
            \n.foo {\r\
            \n  background: svg-inline(test);\r\
            \n}"
        )
        .unwrap(),
        "@svg-load test url(foo.svg) {\
        \n  fill: red;\
        \n}\
        \n.foo {\
        \n  background: svg-inline(test);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1813.hrx"
#[test]
fn issue_1813() {
    assert_eq!(
        rsass(
            "@function foo($value) {\
            \n  $a: bar($value);\
            \n  @return $value;\
            \n}\
            \n\
            \n@function bar($list) {\
            \n  @while (true) {\
            \n    @return true;\
            \n  }\
            \n}\
            \n\
            \na {\
            \n  b: foo(true);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1819.hrx"
#[test]
#[ignore] // wrong result
fn issue_1819() {
    assert_eq!(
        rsass(
            "foo {\
            \n  bar: type-of(selector-unify(\'p\', \'a\'));\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  bar: null;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1822.hrx"

// Ignoring "issue_1822", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_1825.hrx"
#[test]
fn issue_1825() {
    assert_eq!(
        rsass(
            "foo {\
            \n  &-- {\
            \n    &baz {\
            \n      color: red;\
            \n    } \
            \n  } \
            \n} "
        )
        .unwrap(),
        "foo--baz {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1839.hrx"
#[test]
fn issue_1839() {
    assert_eq!(
        rsass("@custom-media --large-viewport (min-width: 1001px);").unwrap(),
        "@custom-media --large-viewport (min-width: 1001px);\
        \n"
    );
}

mod issue_185;

// From "sass-spec/spec/libsass-closed-issues/issue_1873.hrx"

// Ignoring "issue_1873", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_1886.hrx"
#[test]
fn issue_1886() {
    assert_eq!(
        rsass(
            "body {\
            \n  background: url()\
            \n}"
        )
        .unwrap(),
        "body {\
        \n  background: url();\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1889.hrx"
#[test]
fn issue_1889() {
    assert_eq!(
        rsass(
            "@media (min-width: 640px) { \
            \n  /* comment */\
            \n}\
            \n\
            \ndiv {\
            \n  @media (min-width: 320px) { \
            \n    /* comment */\
            \n  }\
            \n}"
        )
        .unwrap(),
        "@media (min-width: 640px) {\
        \n  /* comment */\
        \n}\
        \n@media (min-width: 320px) {\
        \n  div {\
        \n    /* comment */\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1890.hrx"
#[test]
#[ignore] // unexepected error
fn issue_1890() {
    assert_eq!(
        rsass(
            ".wrap {\
            \n  @media (min-width: 480px) { \
            \n    display: block;\
            \n    @at-root (without: media){ \
            \n      .box { \
            \n        display: inline-block;\
            \n      }\
            \n    } \
            \n  }\
            \n}"
        )
        .unwrap(),
        "@media (min-width: 480px) {\
        \n  .wrap {\
        \n    display: block;\
        \n  }\
        \n}\
        \n.wrap .box {\
        \n  display: inline-block;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1898.hrx"
#[test]
fn issue_1898() {
    assert_eq!(
        rsass(
            "@media (min-width: 640px) { \
            \n  /* comment */\
            \n}\
            \n\
            \ndiv {\
            \n  @media (min-width: 320px) { \
            \n    /* comment */\
            \n  }\
            \n}"
        )
        .unwrap(),
        "@media (min-width: 640px) {\
        \n  /* comment */\
        \n}\
        \n@media (min-width: 320px) {\
        \n  div {\
        \n    /* comment */\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1901.hrx"
#[test]
fn issue_1901() {
    assert_eq!(
        rsass(
            "a, b {\
            \n    &:not(c) {\
            \n        d: e;\
            \n    }\
            \n}\
            \n"
        )
        .unwrap(),
        "a:not(c), b:not(c) {\
        \n  d: e;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1904.hrx"
#[test]
fn issue_1904() {
    assert_eq!(
        rsass(
            ".foo {\
            \n  &--#{\'bar\'} {\
            \n    color: red;\
            \n  }\
            \n}"
        )
        .unwrap(),
        ".foo--bar {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1907.hrx"
#[test]
fn issue_1907() {
    assert_eq!(
        rsass(
            "foo {\
            \n  bar: \'test\' + \'1 #{2} 3\';\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  bar: \"test1 2 3\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1915.hrx"
#[test]
#[ignore] // wrong result
fn issue_1915() {
    assert_eq!(
        rsass(
            "@mixin wrapper() {\
            \n  .wrapped {\
            \n    @content;\
            \n  }\
            \n}\
            \n\
            \n%ext {\
            \n  background: #000;\
            \n}\
            \n\
            \n@include wrapper() {\
            \n  @extend %ext;\
            \n}"
        )
        .unwrap(),
        ".wrapped {\
        \n  background: #000;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1916.hrx"

// Ignoring "issue_1916", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_192.hrx"
#[test]
fn issue_192() {
    assert_eq!(
        rsass(
            "@function test($from, $to) {\r\
            \n    @warn \'Starting loop\';\r\
            \n    @for $i from $from through $to {\r\
            \n      @warn \'Step #{$i}\' ;\r\
            \n    }\r\
            \n    @warn \'Finished loop\';\r\
            \n    @return 100%;\r\
            \n}\r\
            \nbody {\r\
            \n    width: test(0, 1);\r\
            \n    height: test(-1, 1);\r\
            \n}"
        )
        .unwrap(),
        "body {\
        \n  width: 100%;\
        \n  height: 100%;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1923.hrx"

// Ignoring "issue_1923", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_1926.hrx"

// From "sass-spec/spec/libsass-closed-issues/issue_1927.hrx"
#[test]
#[ignore] // unexepected error
fn issue_1927() {
    assert_eq!(
        rsass(
            "@media screen {\
            \n  $variable: dynamic;\
            \n  .foo-#{$variable} {a: b}\
            \n  .bar {\
            \n    @extend .foo-dynamic\
            \n  }\
            \n}"
        )
        .unwrap(),
        "@media screen {\
        \n  .foo-dynamic, .bar {\
        \n    a: b;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1931.hrx"
#[test]
fn issue_1931() {
    assert_eq!(
        rsass(
            "$var: \'http://test.com\';\
            \nbody {\
            \n  background-image: url( #{$var});\
            \n}"
        )
        .unwrap(),
        "body {\
        \n  background-image: url(http://test.com);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1940.hrx"
#[test]
fn issue_1940() {
    assert_eq!(
        rsass(
            "// ----\r\
            \n// libsass (v3.3.2)\r\
            \n// ----\r\
            \n\r\
            \n$prefix:(\r\
            \n\tfoo: foo,\r\
            \n\tbar: #bar\r\
            \n);\r\
            \n\r\
            \n#{map-get($prefix, foo)} {\r\
            \n  color: red;\r\
            \n}\r\
            \n#{map-get($prefix, bar)} {\r\
            \n  color: red;\r\
            \n}\r\
            \n\r\
            \n#{map-get($prefix, foo)} .baz {\r\
            \n  color: red;\r\
            \n}\r\
            \n#{map-get($prefix, bar)} .baz {\r\
            \n  color: red;\r\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  color: red;\
        \n}\
        \n#bar {\
        \n  color: red;\
        \n}\
        \nfoo .baz {\
        \n  color: red;\
        \n}\
        \n#bar .baz {\
        \n  color: red;\
        \n}\
        \n"
    );
}

mod issue_1941;

// From "sass-spec/spec/libsass-closed-issues/issue_1944.hrx"
#[test]
fn issue_1944() {
    assert_eq!(
        rsass(
            "$count: 0;\
            \n\
            \n@function foo() {\
            \n    $count: $count + 1 !global;\
            \n    $selector: (\'.bar\' \'baz\');\
            \n    @return $selector;\
            \n}\
            \n\
            \n\
            \n#{foo()} {\
            \n    count: $count;\
            \n}\
            \n"
        )
        .unwrap(),
        ".bar baz {\
        \n  count: 1;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1945.hrx"
#[test]
fn issue_1945() {
    assert_eq!(
        rsass(
            "foo {\
            \n  bar: #{\"\\\\\"}#{\"baz\"};\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  bar: \\baz;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1947.hrx"
#[test]
fn issue_1947() {
    assert_eq!(
        rsass(
            ".a-#{quote(\'\' + b)} {\
            \n  c: d;\
            \n}\
            \n\
            \n.a-#{\'\' + b} {\
            \n  c: d;\
            \n}"
        )
        .unwrap(),
        ".a-b {\
        \n  c: d;\
        \n}\
        \n.a-b {\
        \n  c: d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1960.hrx"
#[test]
#[ignore] // wrong result
fn issue_1960() {
    assert_eq!(
        rsass(
            "foo:not(.missing) {\
            \n  a: b;\
            \n\
            \n  &:hover { c: d; }\
            \n}\
            \n\
            \nbar {\
            \n  @extend .missing;\
            \n}"
        )
        .unwrap(),
        "foo:not(.missing):not(bar) {\
        \n  a: b;\
        \n}\
        \nfoo:not(.missing):not(bar):hover {\
        \n  c: d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1969.hrx"
#[test]
fn issue_1969() {
    assert_eq!(
        rsass(
            "$base-text-color: #666;\
            \n\
            \n@function calcNavbarTextColor ($base-text-color) {\
            \n        @return $base-text-color;\
            \n}\
            \n\
            \n$header-text-color: calcNavbarTextColor($base-text-color);\
            \n\
            \n.test_class {\
            \n        color: lighten($header-text-color, 20%);\
            \n}"
        )
        .unwrap(),
        ".test_class {\
        \n  color: #999999;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1971.hrx"
#[test]
#[ignore] // wrong result
fn issue_1971() {
    assert_eq!(
        rsass(
            "%foo1 {\
            \n  @supports (flex-wrap: wrap) {\
            \n    flex: auto;\
            \n  }\
            \n}\
            \n\
            \n@supports (flex-wrap: wrap) {\
            \n  %foo2 {\
            \n    flex: auto;\
            \n  }\
            \n}\
            \n\
            \n.bar {\
            \n  @extend %foo1;\
            \n  @extend %foo2;\
            \n}\
            \n"
        )
        .unwrap(),
        "@supports (flex-wrap: wrap) {\
        \n  .bar {\
        \n    flex: auto;\
        \n  }\
        \n}\
        \n@supports (flex-wrap: wrap) {\
        \n  .bar {\
        \n    flex: auto;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1977"
#[test]
fn issue_1977() {
    assert_eq!(
        rsass(
            "body#some-\\(selector\\) {\
            \ncolor: red;\
            \n}\
            \n\
            \n#äöü  {\
            \n  color: reds;\
            \n}"
        )
        .unwrap(),
        "@charset \"UTF-8\";\
        \nbody#some-\\(selector\\) {\
        \n  color: red;\
        \n}\
        \n#äöü {\
        \n  color: reds;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1991.hrx"
#[test]
fn issue_1991() {
    assert_eq!(
        rsass(
            "$tests: (\
            \n  0: \'foo\',\
            \n  false: \'bar\',\
            \n);"
        )
        .unwrap(),
        ""
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1993.hrx"
#[test]
#[ignore] // unexepected error
fn issue_1993() {
    assert_eq!(
        rsass(
            ".test {\
            \n  @extend #{\'%test\'} !optional\
            \n}"
        )
        .unwrap(),
        ""
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1994.hrx"
#[test]
#[ignore] // wrong result
fn issue_1994() {
    assert_eq!(
        rsass(
            "%hoverbrighter {\
            \n    &:hover,\
            \n    &:focus {\
            \n        opacity: .8;\
            \n\
            \n        @supports (filter: brightness(120%)) {\
            \n            filter: brightness(120%);\
            \n        }\
            \n    }\
            \n}\
            \n\
            \n.productportal-link {\
            \n    @extend %hoverbrighter;\
            \n}"
        )
        .unwrap(),
        ".productportal-link:hover, .productportal-link:focus {\
        \n  opacity: 0.8;\
        \n}\
        \n@supports (filter: brightness(120%)) {\
        \n  .productportal-link:hover, .productportal-link:focus {\
        \n    filter: brightness(120%);\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_1996.hrx"
#[test]
#[ignore] // wrong result
fn issue_1996() {
    assert_eq!(
        rsass(
            "$foo: \"bar\";\
            \n\
            \n%class //#{$foo}\
            \n{\
            \n  &_baz {\
            \n    color: red;\
            \n  }\
            \n}"
        )
        .unwrap(),
        ""
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2000.hrx"
#[test]
#[ignore] // wrong result
fn issue_2000() {
    assert_eq!(
        rsass(
            ".m__exhibit-header--medium {\
            \n    @extend #{&}--plain;\
            \n    &--plain {\
            \n        font-size: --&;\
            \n    }\
            \n}"
        )
        .unwrap(),
        ".m__exhibit-header--medium--plain, .m__exhibit-header--medium {\
        \n  font-size: -- .m__exhibit-header--medium--plain;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2006.hrx"
#[test]
fn issue_2006() {
    assert_eq!(
        rsass(
            "@mixin main() {\
            \n  bar {\
            \n    baz: 1;\
            \n  }\
            \n}\
            \n\
            \nfoo {\
            \n  @at-root {\
            \n    @include main();\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "bar {\
        \n  baz: 1;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2007.hrx"
#[test]
#[ignore] // wrong result
fn issue_2007() {
    assert_eq!(
        rsass(
            "@mixin foo() {\
            \n  @media (mix-width: 100px) {\
            \n    @extend %bar;\
            \n  }\
            \n}\
            \n\
            \nfoo {\
            \n  @media (mix-width: 100px) {\
            \n    %bar {\
            \n      foo: bar;\
            \n    }\
            \n  }\
            \n\
            \n  @include foo;\
            \n}\
            \n"
        )
        .unwrap(),
        "@media (mix-width: 100px) {\
        \n  foo foo {\
        \n    foo: bar;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2009.hrx"
#[test]
#[ignore] // wrong result
fn issue_2009() {
    assert_eq!(
        rsass(
            "@mixin breakpoint() {\
            \n  @media (min-width: 200px) {\
            \n    @content;\
            \n  }\
            \n}\
            \n\
            \n@mixin foo() {\
            \n  @include breakpoint {\
            \n    @extend %reveal-centered;\
            \n  }\
            \n}\
            \n\
            \n\
            \nfoo {\
            \n  @include breakpoint {\
            \n    %reveal-centered {\
            \n      left: auto;\
            \n      right: auto;\
            \n      margin: 0 auto;\
            \n    }\
            \n  }\
            \n\
            \n  @include foo;\
            \n}\
            \n"
        )
        .unwrap(),
        "@media (min-width: 200px) {\
        \n  foo foo {\
        \n    left: auto;\
        \n    right: auto;\
        \n    margin: 0 auto;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_201.hrx"
#[test]
fn issue_201() {
    assert_eq!(
        rsass("a, b, { color: red; }").unwrap(),
        "a, b {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2017.hrx"
#[test]
#[ignore] // wrong result
fn issue_2017() {
    assert_eq!(
        rsass(
            "foo {\r\
            \n  bar: baz;\r\
            \n}\r\
            \n\r\
            \n@mixin link() {\r\
            \n  a:not(.btn) {\r\
            \n    color: red;\r\
            \n  }\r\
            \n}\r\
            \n\r\
            \nfoo {\r\
            \n  @include link;\r\
            \n  @extend .btn;\r\
            \n  @include link;\r\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  bar: baz;\
        \n}\
        \nfoo a:not(.btn):not(foo) {\
        \n  color: red;\
        \n}\
        \nfoo a:not(.btn):not(foo) {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2020.hrx"
#[test]
fn issue_2020() {
    assert_eq!(
        rsass(
            "form {\r\
            \n  $selector: nth(&, 1);\r\
            \n  sel: inspect($selector);\r\
            \n  $selector: nth($selector, 1);\r\
            \n  sel: inspect($selector);\r\
            \n} "
        )
        .unwrap(),
        "form {\
        \n  sel: form;\
        \n  sel: form;\
        \n}\
        \n"
    );
}

mod issue_2023;

mod issue_2031;

// From "sass-spec/spec/libsass-closed-issues/issue_2034.hrx"
#[test]
#[ignore] // wrong result
fn issue_2034() {
    assert_eq!(
        rsass(
            ":not(.thing) {\r\
            \n    color: red;\r\
            \n}\r\
            \n:not(.bar) {\r\
            \n    @extend .thing;\r\
            \n    background: blue;\r\
            \n}"
        )
        .unwrap(),
        ":not(.thing) {\
        \n  color: red;\
        \n}\
        \n:not(.bar) {\
        \n  background: blue;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2042.hrx"
#[test]
#[ignore] // wrong result
fn issue_2042() {
    assert_eq!(
        rsass(
            ".wizard-editor {\r\
            \n    transform: scale(50/1);\r\
            \n    transform: scale((50/1));\r\
            \n    transform: scale( (50/1) );\r\
            \n}"
        )
        .unwrap(),
        ".wizard-editor {\
        \n  transform: scale(50/1);\
        \n  transform: scale(50);\
        \n  transform: scale(50);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2053.hrx"
#[test]
#[ignore] // wrong result
fn issue_2053() {
    assert_eq!(
        rsass(
            ".foo[disabled] {\
            \n    @extend .foo;\
            \n    background: blue;\
            \n  }\
            \n.bar[disabled] {\
            \n  foo {\
            \n    @extend .bar;\
            \n    background: blue;\
            \n  }\
            \n}\
            \nfoo {\
            \n  .baz[disabled] {\
            \n    @extend .baz;\
            \n    background: blue;\
            \n  }\
            \n}"
        )
        .unwrap(),
        ".foo[disabled] {\
        \n  background: blue;\
        \n}\
        \n.bar[disabled] foo {\
        \n  background: blue;\
        \n}\
        \nfoo .baz[disabled] {\
        \n  background: blue;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2054.hrx"
#[test]
#[ignore] // wrong result
fn issue_2054() {
    assert_eq!(
        rsass(
            ":not(.thing) {\r\
            \n    color: red;\r\
            \n}\r\
            \n:not(.bar) {\r\
            \n    @extend .thing;\r\
            \n    background: blue;\r\
            \n}"
        )
        .unwrap(),
        ":not(.thing) {\
        \n  color: red;\
        \n}\
        \n:not(.bar) {\
        \n  background: blue;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2055.hrx"
#[test]
#[ignore] // wrong result
fn issue_2055() {
    assert_eq!(
        rsass(
            ":not(.thing) {\
            \n    color: red;\
            \n}\
            \n:not(.thing[disabled]) {\
            \n    @extend .thing;\
            \n    background: blue;\
            \n}\
            \n:has(:not(.thing[disabled])) {\
            \n    @extend .thing;\
            \n    background: blue;\
            \n}\
            \n"
        )
        .unwrap(),
        ":not(.thing) {\
        \n  color: red;\
        \n}\
        \n:not(.thing[disabled]):not([disabled]:has(:not(.thing[disabled]):not([disabled]:not(.thing[disabled])))):not([disabled]:not(.thing[disabled]):not([disabled]:has(:not(.thing[disabled]):not([disabled]:not(.thing[disabled]))))) {\
        \n  background: blue;\
        \n}\
        \n:has(:not(.thing[disabled]):not([disabled]:has(:not(.thing[disabled]):not([disabled]:not(.thing[disabled])))):not([disabled]:not(.thing[disabled]):not([disabled]:has(:not(.thing[disabled]):not([disabled]:not(.thing[disabled])))))) {\
        \n  background: blue;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2056.hrx"
#[test]
fn issue_2056() {
    assert_eq!(
        rsass(
            ":foobar(.baz) {\
            \n    color: red;\
            \n}\
            \n"
        )
        .unwrap(),
        ":foobar(.baz) {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2057.hrx"
#[test]
#[ignore] // wrong result
fn issue_2057() {
    assert_eq!(
        rsass(
            ":not(.thing) {\r\
            \n    color: red;\r\
            \n}\r\
            \n:not(.bar) {\r\
            \n    @extend .thing;\r\
            \n    background: blue;\r\
            \n}"
        )
        .unwrap(),
        ":not(.thing) {\
        \n  color: red;\
        \n}\
        \n:not(.bar) {\
        \n  background: blue;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2074.hrx"
#[test]
fn issue_2074() {
    assert_eq!(
        rsass(
            "@function calc_foo() {\r\
            \n  @return \"bar\";\r\
            \n}\r\
            \nfoo {\r\
            \n  test1: calc-foo();\r\
            \n  test2: calc_foo();\r\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  test1: \"bar\";\
        \n  test2: \"bar\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2081.hrx"

// Ignoring "issue_2081", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_2095.hrx"
#[test]
#[ignore] // wrong result
fn issue_2095() {
    assert_eq!(
        rsass(
            "@media all {\
            \n  @mixin foo() {}\
            \n}\
            \n"
        )
        .unwrap(),
        ""
    );
}

mod issue_2106;

// From "sass-spec/spec/libsass-closed-issues/issue_2112.hrx"
#[test]
fn issue_2112() {
    assert_eq!(
        rsass(
            "$color: #1caf9a;\r\
            \n\r\
            \nbody {\r\
            \n  test-01: change-color($color, $hue: -240);\r\
            \n  test-03: change-color($color, $hue: 120);\r\
            \n  test-02: change-color($color, $hue: 480);\r\
            \n}"
        )
        .unwrap(),
        "body {\
        \n  test-01: #1caf1c;\
        \n  test-03: #1caf1c;\
        \n  test-02: #1caf1c;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2116.hrx"
#[test]
#[ignore] // wrong result
fn issue_2116() {
    assert_eq!(
        rsass(
            "@function foo() {\
            \n  @return if(& != null, green, red);\
            \n}\
            \n\
            \ntest {\
            \n  color: foo();\
            \n}\
            \n"
        )
        .unwrap(),
        "test {\
        \n  color: green;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2120"
#[test]
fn issue_2120() {
    assert_eq!(
        rsass("@import url(//xyz.cöm/ürl)").unwrap(),
        "@charset \"UTF-8\";\
        \n@import url(//xyz.cöm/ürl);\
        \n"
    );
}

mod issue_2123;

// From "sass-spec/spec/libsass-closed-issues/issue_2124.hrx"
#[test]
fn issue_2124() {
    assert_eq!(
        rsass(
            "test {\r\
            \n  test-01: #{if(&, \'true\', \'false\')};\r\
            \n  test-01: #{if(0, \'true\', \'false\')};\r\
            \n  test-01: #{if(\'\', \'true\', \'false\')};\r\
            \n  test-01: #{if(\'0\', \'true\', \'false\')};\r\
            \n  test-01: #{if(null, \'true\', \'false\')};\r\
            \n  test-01: #{if(false, \'true\', \'false\')};\r\
            \n}\r\
            \n\r\
            \n#{if(&, \'has-parent\', \'parentless\')} {\r\
            \n  test: parent;\r\
            \n}\r\
            \n\r\
            \n@mixin with-js() {\r\
            \n  .js:root #{if(&, \'&\', \'\')} {\r\
            \n    @content;\r\
            \n  }\r\
            \n}\r\
            \n\r\
            \n@include with-js() {\r\
            \n  .bou {\r\
            \n    content: \'bar\';\r\
            \n  }\r\
            \n}\r\
            \n\r\
            \n.bou {\r\
            \n  @include with-js() {\r\
            \n    .bar {\r\
            \n      content: \'baz\';\r\
            \n    }\r\
            \n  }\r\
            \n}\r\
            \n\r\
            \n"
        )
        .unwrap(),
        "test {\
        \n  test-01: true;\
        \n  test-01: true;\
        \n  test-01: true;\
        \n  test-01: true;\
        \n  test-01: false;\
        \n  test-01: false;\
        \n}\
        \nparentless {\
        \n  test: parent;\
        \n}\
        \n.js:root .bou {\
        \n  content: \"bar\";\
        \n}\
        \n.js:root .bou .bar {\
        \n  content: \"baz\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2139.hrx"
#[test]
#[ignore] // wrong result
fn issue_2139() {
    assert_eq!(
        rsass(
            ".foo {\
            \n  color: #FFF;\
            \n}\
            \n\
            \n.bar .baz {\
            \n  @extend .foo;\
            \n\
            \n  border: \'1px\';\
            \n}\
            \n\
            \n*:not(.foo) {\
            \n  display: none;\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo, .bar .baz {\
        \n  color: #FFF;\
        \n}\
        \n.bar .baz {\
        \n  border: \"1px\";\
        \n}\
        \n*:not(.foo) {\
        \n  display: none;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2140.hrx"
#[test]
#[ignore] // wrong result
fn issue_2140() {
    assert_eq!(
        rsass(
            "$red: red;\
            \n$foo: red;\
            \n\
            \n:root {\
            \n  --red: #f00;\
            \n  // --red: $red;\
            \n  // --red: $foo;\
            \n}\
            \n\
            \na { content: var(--red) }\
            \na { content: var(--$red) }\
            \na { content: var(--$foo) }\
            \n\
            \nb { content: (- red) }\
            \nb { content: (- $red) }\
            \nb { content: (- $foo) }\
            \n\
            \nc { content: (+- red) }\
            \nc { content: (+- $red) }\
            \nc { content: (+- $foo) }\
            \n\
            \nd { content: (-red) }\
            \nd { content: (-$red) }\
            \nd { content: (-$foo) }\
            \n\
            \ne { content: (+ red) }\
            \ne { content: (+ $red) }\
            \ne { content: (+ $foo) }"
        )
        .unwrap(),
        ":root {\
        \n  --red: #f00;\
        \n}\
        \na {\
        \n  content: var(--red);\
        \n}\
        \na {\
        \n  content: var(-- red);\
        \n}\
        \na {\
        \n  content: var(-- red);\
        \n}\
        \nb {\
        \n  content: -red;\
        \n}\
        \nb {\
        \n  content: -red;\
        \n}\
        \nb {\
        \n  content: -red;\
        \n}\
        \nc {\
        \n  content: +-red;\
        \n}\
        \nc {\
        \n  content: +-red;\
        \n}\
        \nc {\
        \n  content: +-red;\
        \n}\
        \nd {\
        \n  content: -red;\
        \n}\
        \nd {\
        \n  content: -red;\
        \n}\
        \nd {\
        \n  content: -red;\
        \n}\
        \ne {\
        \n  content: +red;\
        \n}\
        \ne {\
        \n  content: +red;\
        \n}\
        \ne {\
        \n  content: +red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2143.hrx"

// Ignoring "issue_2143", error tests are not supported yet.

mod issue_2147;

// From "sass-spec/spec/libsass-closed-issues/issue_2149.hrx"
#[test]
fn issue_2149() {
    assert_eq!(
        rsass(
            ".foo {\
            \n  background: url(\'background.png\') -10px -10px/110% no-repeat\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  background: url(\"background.png\") -10px -10px/110% no-repeat;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2150.hrx"
#[test]
#[ignore] // wrong result
fn issue_2150() {
    assert_eq!(
        rsass(
            "@media (min-width: 100px) {\
            \n  .parent > %child {\
            \n    color: blue;\
            \n  }\
            \n}\
            \n\
            \n.foo {\
            \n  @extend %child;\
            \n}\
            \n"
        )
        .unwrap(),
        "@media (min-width: 100px) {\
        \n  .parent > .foo {\
        \n    color: blue;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2153.hrx"
#[test]
fn issue_2153() {
    assert_eq!(
        rsass(
            "foo {\
            \n  a: \"\\\"foo\\\"\" + \"\";\
            \n  b: \"\" + \"\\\"foo\\\"\";\
            \n  c: \"\" + \"\\\"foo\";\
            \n  d: \"\\\"foo\\\"\" + \"bar\";\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: \'\"foo\"\';\
        \n  b: \'\"foo\"\';\
        \n  c: \'\"foo\';\
        \n  d: \'\"foo\"bar\';\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2154.hrx"
#[test]
#[ignore] // wrong result
fn issue_2154() {
    assert_eq!(
        rsass(
            "@media (min-width: 1px) {\
            \n  .first {\
            \n    font-weight: 100;\
            \n\
            \n    @media (min-width: 2px) {}\
            \n  }\
            \n  .second {\
            \n    font-weight: 200;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@media (min-width: 1px) {\
        \n  .first {\
        \n    font-weight: 100;\
        \n  }\
        \n  .second {\
        \n    font-weight: 200;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2155.hrx"

// Ignoring "issue_2155", error tests are not supported yet.

mod issue_2156;

// From "sass-spec/spec/libsass-closed-issues/issue_2169.hrx"
#[test]
fn issue_2169() {
    assert_eq!(
        rsass(
            "@function test($a, $b) {\
            \n  @return \"#{$a}\" + \"#{$b}\" + \"\" + \"\";\
            \n}\
            \n\
            \nfoo {\
            \n  content: test(\'bim\', \'baz\');\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  content: \"bimbaz\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2175.hrx"

// Ignoring "issue_2175", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_2177.hrx"

// From "sass-spec/spec/libsass-closed-issues/issue_2179.hrx"
#[test]
#[ignore] // wrong result
fn issue_2179() {
    assert_eq!(
        rsass(
            "@mixin sprite-arrow() {\
            \n  @extend %hidden-text;\
            \n}\
            \n\
            \n%hidden-text {\
            \n  text-indent: -999em;\
            \n}\
            \n\
            \n// button.scss\
            \n.button-left,\
            \n.button-right,\
            \n.button-plus,\
            \n.button-min {\
            \n  &:after {\
            \n      @include sprite-arrow();\
            \n  }\
            \n}\
            \n\
            \n.banner {\
            \n  &:after {\
            \n      @include sprite-arrow();\
            \n  }\
            \n}\
            \n\
            \n.calculator {\
            \n  .btn-down,\
            \n  .btn-up {\
            \n      &:after {\
            \n          @include sprite-arrow();\
            \n      }\
            \n  }\
            \n}"
        )
        .unwrap(),
        ".calculator .btn-down:after,\
        \n.calculator .btn-up:after, .banner:after, .button-left:after,\
        \n.button-right:after,\
        \n.button-plus:after,\
        \n.button-min:after {\
        \n  text-indent: -999em;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2185.hrx"
#[test]
#[ignore] // wrong result
fn issue_2185() {
    assert_eq!(
        rsass(
            "$bar: true;\r\
            \n\r\
            \n@mixin mixin() {\r\
            \n    mix: in;\r\
            \n}\r\
            \n\r\
            \n@mixin include() {\r\
            \n    @content;\r\
            \n}\r\
            \n\r\
            \np {\r\
            \n    @at-root {\r\
            \n        @if $bar {\r\
            \n            #if {\r\
            \n                height: 100px;\r\
            \n            }\r\
            \n        }\r\
            \n        @if (not $bar) {\r\
            \n        } @else if($bar) {\r\
            \n            #elseif {\r\
            \n                width: 100px;\r\
            \n            }\r\
            \n        }\r\
            \n        @if (not $bar) {\r\
            \n        } @else {\r\
            \n            #else {\r\
            \n                width: 100px;\r\
            \n            }\r\
            \n        }\r\
            \n        @for $i from 1 through 1 {\r\
            \n            #for {\r\
            \n                foo: bar#{$i};\r\
            \n            }\r\
            \n        }\r\
            \n        $i: 0;\r\
            \n        @while ($i == 0) {\r\
            \n            $i: $i + 1;\r\
            \n            #while {\r\
            \n                foo: bar#{$i};\r\
            \n            }\r\
            \n        }\r\
            \n        @each $i in (1) {\r\
            \n            #each {\r\
            \n                foo: bar#{$i};\r\
            \n            }\r\
            \n        }\r\
            \n        #inc {\r\
            \n            @include mixin();\r\
            \n            @include include() {\r\
            \n                inc: lude;\r\
            \n            }\r\
            \n        }\r\
            \n        @supports (display: flex) {\r\
            \n            a {display: flex}\r\
            \n        }\r\
            \n        @foo {\r\
            \n           bar: bat;\r\
            \n        }\r\
            \n    }\r\
            \n}"
        )
        .unwrap(),
        "#if {\
        \n  height: 100px;\
        \n}\
        \n#elseif {\
        \n  width: 100px;\
        \n}\
        \n#else {\
        \n  width: 100px;\
        \n}\
        \n#for {\
        \n  foo: bar1;\
        \n}\
        \n#while {\
        \n  foo: bar1;\
        \n}\
        \n#each {\
        \n  foo: bar1;\
        \n}\
        \n#inc {\
        \n  mix: in;\
        \n  inc: lude;\
        \n}\
        \n@supports (display: flex) {\
        \n  a {\
        \n    display: flex;\
        \n  }\
        \n}\
        \n@foo {\
        \n  bar: bat;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2198.hrx"
#[test]
fn issue_2198() {
    assert_eq!(
        rsass(
            "@mixin test() {\
            \n  @at-root {\
            \n    @include foo();\
            \n  }\
            \n}\
            \n\
            \n@mixin foo() {\
            \n  #{\'.foo\'} {\
            \n    bar: baz;\
            \n  }\
            \n}\
            \n\
            \n.test {\
            \n  @include test();\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  bar: baz;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2200.hrx"
#[test]
#[ignore] // wrong result
fn issue_2200() {
    assert_eq!(
        rsass(
            ".media-object-section:last-child:not(:nth-child(2)) {\
            \n  color: blue;\
            \n}\
            \n\
            \n%orbit-control {\
            \n  color: red;\
            \n}\
            \n\
            \n.orbit-previous {\
            \n  @extend %orbit-control;\
            \n}\
            \n"
        )
        .unwrap(),
        ".media-object-section:last-child:not(:nth-child(2)) {\
        \n  color: blue;\
        \n}\
        \n.orbit-previous {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2202.hrx"
#[test]
fn issue_2202() {
    assert_eq!(
        rsass(
            "@customAtRule;\r\
            \ntest {\r\
            \n  content: bar\r\
            \n}"
        )
        .unwrap(),
        "@customAtRule;\
        \ntest {\
        \n  content: bar;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2205.hrx"
#[test]
#[ignore] // wrong result
fn issue_2205() {
    assert_eq!(
        rsass(
            "$params1: #fff .5;\r\
            \ntest {\r\
            \n  color: rgba($params1...);\r\
            \n}\r\
            \n\r\
            \n$params2: 10 250 130 .5;\r\
            \ntest {\r\
            \n  color: rgba($params2...);\r\
            \n}"
        )
        .unwrap(),
        "test {\
        \n  color: rgba(255, 255, 255, 0.5);\
        \n}\
        \ntest {\
        \n  color: rgba(10, 250, 130, 0.5);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_221255.hrx"

// Ignoring "issue_221255", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_221289.hrx"
#[test]
fn issue_221289() {
    assert_eq!(
        rsass(
            "foo {\r\
            \n  bar: if(0,0<0,0);\r\
            \n}"
        )
        .unwrap(),
        "foo {\
        \n  bar: false;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2233.hrx"
#[test]
#[ignore] // unexepected error
fn issue_2233() {
    assert_eq!(
        rsass(
            "@media all and (min-width: 100px) {\
            \n  @import \"foo\"\
            \n}\
            \n"
        )
        .unwrap(),
        "@media all and (min-width: 100px) {\
        \n  a {\
        \n    b: c;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_224.hrx"
#[test]
fn issue_224() {
    assert_eq!(
        rsass(
            "$list: (\"a\", \"b\", \"c\");\
            \n\
            \ntest {\
            \n    content: nth($list, -1);\
            \n    content: nth($list, -2);\
            \n    content: nth($list, -3);\
            \n    content: nth($list, -1) == nth($list, length($list));\
            \n}\
            \n"
        )
        .unwrap(),
        "test {\
        \n  content: \"c\";\
        \n  content: \"b\";\
        \n  content: \"a\";\
        \n  content: true;\
        \n}\
        \n"
    );
}

mod issue_2243;

// From "sass-spec/spec/libsass-closed-issues/issue_2246.hrx"
#[test]
#[ignore] // wrong result
fn issue_2246() {
    assert_eq!(
        rsass(
            "@mixin foo($option: \'foo\') {\
            \n     // Create a unique, random placeholder to store styles\
            \n    $placeholder : $option + random(9999);\
            \n\
            \n    // Store the styles in the placeholder\
            \n    @at-root %#{$placeholder} {\
            \n        content: \'foo\';\
            \n    }\
            \n\
            \n    @at-root {\
            \n        .bar {\
            \n            @extend %#{$placeholder};\
            \n        }\
            \n    }\
            \n}\
            \n\
            \n@mixin bar($option) {\
            \n    @include foo($option);\
            \n}\
            \n\
            \n.foo {\
            \n    @include bar(\'baz\');\
            \n}"
        )
        .unwrap(),
        ".bar {\
        \n  content: \"foo\";\
        \n}\
        \n"
    );
}

mod issue_2260;

// From "sass-spec/spec/libsass-closed-issues/issue_2261.hrx"
#[test]
fn issue_2261() {
    assert_eq!(
        rsass(
            "$seven: 7;\
            \n\
            \n.test {\
            \n\
            \n  equal-01: (7 == 7);\
            \n  equal-02: (\'7\' == \'7\');\
            \n  equal-03: (\'#{7}\' == \'#{7}\');\
            \n\
            \n  equal-04: (7 == \'7\');\
            \n  equal-05: (\'7\' == 7);\
            \n  equal-06: (7 == \'#{7}\');\
            \n  equal-07: (\'#{7}\' == 7);\
            \n  equal-08: (\'7\' == \'#{7}\');\
            \n  equal-09: (\'#{7}\' == \'7\');\
            \n\
            \n  equal-10: ($seven == 7);\
            \n  equal-11: ($seven == \'7\');\
            \n  equal-13: ($seven == \'#{7}\');\
            \n\
            \n  equal-14: (7 == $seven);\
            \n  equal-15: (\'7\' == $seven);\
            \n  equal-16: (\'#{7}\' == $seven);\
            \n\
            \n  equal-17: (\'#{$seven}\' == 7);\
            \n  equal-18: (\'#{$seven}\' == \'7\');\
            \n  equal-19: (\'#{$seven}\' == \'#{7}\');\
            \n\
            \n  equal-20: (7 == \'#{$seven}\');\
            \n  equal-21: (\'7\' == \'#{$seven}\');\
            \n  equal-22: (\'#{7}\' == \'#{$seven}\');\
            \n\
            \n  equal-23: (\'#{$seven}\' == $seven);\
            \n  equal-24: (\'#{$seven}\' == \'#{$seven}\');\
            \n\
            \n}"
        )
        .unwrap(),
        ".test {\
        \n  equal-01: true;\
        \n  equal-02: true;\
        \n  equal-03: true;\
        \n  equal-04: false;\
        \n  equal-05: false;\
        \n  equal-06: false;\
        \n  equal-07: false;\
        \n  equal-08: true;\
        \n  equal-09: true;\
        \n  equal-10: true;\
        \n  equal-11: false;\
        \n  equal-13: false;\
        \n  equal-14: true;\
        \n  equal-15: false;\
        \n  equal-16: false;\
        \n  equal-17: false;\
        \n  equal-18: true;\
        \n  equal-19: true;\
        \n  equal-20: false;\
        \n  equal-21: true;\
        \n  equal-22: true;\
        \n  equal-23: false;\
        \n  equal-24: true;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2289.hrx"
#[test]
#[ignore] // wrong result
fn issue_2289() {
    assert_eq!(
        rsass(
            ".foo:baz:baz {\
            \n  float: left;\
            \n}\
            \n\
            \n.bar {\
            \n  @extend .foo;\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo:baz:baz, .bar:baz:baz {\
        \n  float: left;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2291.hrx"
#[test]
#[ignore] // wrong result
fn issue_2291() {
    assert_eq!(
        rsass(
            ".m__exhibit-header--medium {\
            \n    @extend #{&}--plain;\
            \n    &--plain {\
            \n        font-size: 1em;\
            \n    }\
            \n}\
            \n\
            \nfoo {\
            \n  bar[baz=\"#{&}\"][str=\"&\"] {\
            \n    asd: qwe;\
            \n  }\
            \n}\
            \n\
            \nA, B, C {\
            \n  #{&}-foo#{&}-bar {\
            \n    color: blue;\
            \n  }\
            \n  #{\"A, B, C\"}-foo#{\"A, B, C\"}-bar {\
            \n    color: blue;\
            \n  }\
            \n}\
            \n\
            \nA {\
            \n  B#{&}C {\
            \n    .b, .c, .d {\
            \n      #{&}-foo {\
            \n        parent: &bar;\
            \n        itpl: #{&}bar;\
            \n      }\
            \n      #{\"A .b, A .c, A .d\"}-foo {\
            \n        parent: &bar;\
            \n        itpl: #{&}bar;\
            \n      }\
            \n    }\
            \n  }\
            \n}"
        )
        .unwrap(),
        ".m__exhibit-header--medium--plain, .m__exhibit-header--medium {\
        \n  font-size: 1em;\
        \n}\
        \nfoo bar[baz=foo][str=\"&\"] {\
        \n  asd: qwe;\
        \n}\
        \nA A, A B, A C-fooA, A B, A C-bar, B A, B B, B C-fooA, B B, B C-bar, C A, C B, C C-fooA, C B, C C-bar {\
        \n  color: blue;\
        \n}\
        \nA A, A B, A C-fooA, A B, A C-bar, B A, B B, B C-fooA, B B, B C-bar, C A, C B, C C-fooA, C B, C C-bar {\
        \n  color: blue;\
        \n}\
        \nA BAC .b A BAC .b, A BAC .b A BAC .c, A BAC .b A BAC .d-foo, A BAC .c A BAC .b, A BAC .c A BAC .c, A BAC .c A BAC .d-foo, A BAC .d A BAC .b, A BAC .d A BAC .c, A BAC .d A BAC .d-foo {\
        \n  parent: A BAC .b A BAC .b, A BAC .b A BAC .c, A BAC .b A BAC .d-foo, A BAC .c A BAC .b, A BAC .c A BAC .c, A BAC .c A BAC .d-foo, A BAC .d A BAC .b, A BAC .d A BAC .c, A BAC .d A BAC .d-foo bar;\
        \n  itpl: A BAC .b A BAC .b, A BAC .b A BAC .c, A BAC .b A BAC .d-foo, A BAC .c A BAC .b, A BAC .c A BAC .c, A BAC .c A BAC .d-foo, A BAC .d A BAC .b, A BAC .d A BAC .c, A BAC .d A BAC .d-foobar;\
        \n}\
        \nA BAC .b A .b, A BAC .b A .c, A BAC .b A .d-foo, A BAC .c A .b, A BAC .c A .c, A BAC .c A .d-foo, A BAC .d A .b, A BAC .d A .c, A BAC .d A .d-foo {\
        \n  parent: A BAC .b A .b, A BAC .b A .c, A BAC .b A .d-foo, A BAC .c A .b, A BAC .c A .c, A BAC .c A .d-foo, A BAC .d A .b, A BAC .d A .c, A BAC .d A .d-foo bar;\
        \n  itpl: A BAC .b A .b, A BAC .b A .c, A BAC .b A .d-foo, A BAC .c A .b, A BAC .c A .c, A BAC .c A .d-foo, A BAC .d A .b, A BAC .d A .c, A BAC .d A .d-foobar;\
        \n}\
        \n"
    );
}

mod issue_2295;

// From "sass-spec/spec/libsass-closed-issues/issue_2303.hrx"
#[test]
#[ignore] // wrong result
fn issue_2303() {
    assert_eq!(
        rsass(
            ".wrapper-class {\r\
            \n  @import \'module\';\r\
            \n}"
        )
        .unwrap(),
        ".wrapper-class .okay {\
        \n  background: green;\
        \n}\
        \n.wrapper-class .broken {\
        \n  background: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2304.hrx"

// Ignoring "issue_2304", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_2307.hrx"

// Ignoring "issue_2307", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_2309.hrx"
#[test]
#[ignore] // wrong result
fn issue_2309() {
    assert_eq!(
        rsass(
            "$button-sizes: (\r\
            \n  \'xs\': (\r\
            \n    \'line-height\': 16 / 12,\r\
            \n  ),\r\
            \n  \'s\': (\r\
            \n    \'line-height\': 18 / 14,\r\
            \n  ),\r\
            \n  \'m\': (\r\
            \n    \'line-height\': 18 / 14,\r\
            \n  ),\r\
            \n  \'l\': (\r\
            \n    \'line-height\': 22 / 16,\r\
            \n  )\r\
            \n);\r\
            \n\r\
            \n@each $size in $button-sizes {\r\
            \n  $size-metrics: nth($size, 2);\r\
            \n\r\
            \n  .c-button__icon {\r\
            \n    min-height: map-get($size-metrics, \'line-height\') * 1em;\r\
            \n  }\r\
            \n}"
        )
        .unwrap(),
        ".c-button__icon {\
        \n  min-height: 1.3333333333em;\
        \n}\
        \n.c-button__icon {\
        \n  min-height: 1.2857142857em;\
        \n}\
        \n.c-button__icon {\
        \n  min-height: 1.2857142857em;\
        \n}\
        \n.c-button__icon {\
        \n  min-height: 1.375em;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_231.hrx"
#[test]
fn issue_231() {
    assert_eq!(
        rsass(
            "// test.scss:\r\
            \na {\r\
            \n  background-image: url(fn(\"s\"));\r\
            \n}"
        )
        .unwrap(),
        "a {\
        \n  background-image: url(fn(\"s\"));\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2320"
#[test]
fn issue_2320() {
    assert_eq!(
        rsass(
            "$char-f: \'\\66\';\r\
            \n$char-g: \'\\67\';\r\
            \n\r\
            \n.test-1 {\r\
            \n  content: \'#{$char-f}\\feff\';\r\
            \n}\r\
            \n\r\
            \n.test-2 {\r\
            \n  content: \'#{$char-g}\\feff\';\r\
            \n}\r\
            \n\r\
            \n// this is broken\r\
            \n.test-3 {\r\
            \n  content: \'\\feff#{$char-f}\';\r\
            \n}\r\
            \n\r\
            \n.test-4 {\r\
            \n  content: \'\\feff#{$char-g}\';\r\
            \n}"
        )
        .unwrap(),
        "@charset \"UTF-8\";\
        \n.test-1 {\
        \n  content: \"f\u{feff}\";\
        \n}\
        \n.test-2 {\
        \n  content: \"g\u{feff}\";\
        \n}\
        \n.test-3 {\
        \n  content: \"\u{feff}f\";\
        \n}\
        \n.test-4 {\
        \n  content: \"\u{feff}g\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2321.hrx"
#[test]
#[ignore] // wrong result
fn issue_2321() {
    assert_eq!(
        rsass(
            "a {\
            \n  b: if(true, b, c...);\
            \n  c: if(false, b, c...);\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: b;\
        \n  c: c;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2330.hrx"
#[test]
fn issue_2330() {
    assert_eq!(
        rsass(
            "@function test () {\r\
            \n  $m: ();\r\
            \n  $abc: (a b c d e f g h i j k);\r\
            \n\r\
            \n  @for $index from 1 through length($abc) {;\r\
            \n    $m: map-merge($m, (nth($abc, $index):$index) );\r\
            \n  }\r\
            \n\r\
            \n  @return $m;\r\
            \n}\r\
            \n\r\
            \ntest {\r\
            \n  content: inspect(test());\r\
            \n}"
        )
        .unwrap(),
        "test {\
        \n  content: (a: 1, b: 2, c: 3, d: 4, e: 5, f: 6, g: 7, h: 8, i: 9, j: 10, k: 11);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2333.hrx"
#[test]
#[ignore] // wrong result
fn issue_2333() {
    assert_eq!(
        rsass("test { test: inspect((a:1,b:(foo,bar),c:3)); }").unwrap(),
        "test {\
        \n  test: (a: 1, b: (foo, bar), c: 3);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2341.hrx"
#[test]
#[ignore] // wrong result
fn issue_2341() {
    assert_eq!(
        rsass(
            "@function aFunction() {\r\
            \n    @return 1em;\r\
            \n}\r\
            \n\r\
            \n@media (max-width: 1em) {\r\
            \n    %placeholder {\r\
            \n        color: red;\r\
            \n    }\r\
            \n}\r\
            \n\r\
            \n@media (max-width: aFunction()) {\r\
            \n    .test {\r\
            \n        @extend %placeholder;\r\
            \n    }\r\
            \n}"
        )
        .unwrap(),
        "@media (max-width: 1em) {\
        \n  .test {\
        \n    color: red;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2346.hrx"
#[test]
#[ignore] // wrong result
fn issue_2346() {
    assert_eq!(
        rsass(
            "$items: 3;\r\
            \nli {\r\
            \n  &:nth-child(#{$items}n - #{$items}) {\r\
            \n    color: red;\r\
            \n  }\r\
            \n}"
        )
        .unwrap(),
        "li:nth-child(3n-3) {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2347.hrx"
#[test]
#[ignore] // unexepected error
fn issue_2347() {
    assert_eq!(
        rsass(
            "%baz2 {\r\
            \n    display: flex;\r\
            \n}\r\
            \n%baz3 {\r\
            \n    display: flex;\r\
            \n}\r\
            \n\r\
            \ncustom2, [custom2], .custom2 {\r\
            \n    @extend %baz2\r\
            \n}\r\
            \n\r\
            \n[custom3], custom3, .custom3 {\r\
            \n    @extend %baz3\r\
            \n}"
        )
        .unwrap(),
        "custom2, [custom2], .custom2 {\
        \n  display: flex;\
        \n}\
        \n[custom3], custom3, .custom3 {\
        \n  display: flex;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2349.hrx"
#[test]
fn issue_2349() {
    assert_eq!(
        rsass(
            "$path1: assets/images; // no errors thrown\r\
            \n$path2: /images;       // errors thrown\r\
            \n.test {\r\
            \n  background: url(#{$path1}/image.png);\r\
            \n  background: url(#{$path2}/image.png);\r\
            \n}"
        )
        .unwrap(),
        ".test {\
        \n  background: url(assets/images/image.png);\
        \n  background: url(/images/image.png);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2352.hrx"

// Ignoring "issue_2352", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_2358.hrx"
#[test]
#[ignore] // wrong result
fn issue_2358() {
    assert_eq!(
        rsass(
            ".outer {\r\
            \n  @at-root .root {\r\
            \n    .inner {\r\
            \n      .element {\r\
            \n        --modifier#{&}--another-modifier {\r\
            \n          content: \"#{&}\";\r\
            \n        }\r\
            \n        &--modifier#{&}--another-modifier {\r\
            \n          content: \"#{&}\";\r\
            \n        }\r\
            \n      }\r\
            \n    }\r\
            \n  }\r\
            \n}\r\
            \n\r\
            \n@at-root .block {\r\
            \n  &__element {\r\
            \n    #{&} {\r\
            \n      content: \"#{&}\";\r\
            \n    }\r\
            \n    &--modifier {\r\
            \n      content: \"#{&}\";\r\
            \n    }\r\
            \n    --modifier#{&}--another-modifier {\r\
            \n      content: \"#{&}\";\r\
            \n    }\r\
            \n    &--modifier#{&}--another-modifier {\r\
            \n      content: \"#{&}\";\r\
            \n    }\r\
            \n  }\r\
            \n}\r\
            \n"
        )
        .unwrap(),
        ".root .inner .element --modifier.root .inner .element--another-modifier {\
        \n  content: \".root .inner .element --modifier.root .inner .element--another-modifier\";\
        \n}\
        \n.root .inner .element--modifier.root .inner .element--another-modifier {\
        \n  content: \".root .inner .element--modifier.root .inner .element--another-modifier\";\
        \n}\
        \n.block__element .block__element {\
        \n  content: \".block__element .block__element\";\
        \n}\
        \n.block__element--modifier {\
        \n  content: \".block__element--modifier\";\
        \n}\
        \n.block__element --modifier.block__element--another-modifier {\
        \n  content: \".block__element --modifier.block__element--another-modifier\";\
        \n}\
        \n.block__element--modifier.block__element--another-modifier {\
        \n  content: \".block__element--modifier.block__element--another-modifier\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2360.hrx"

// From "sass-spec/spec/libsass-closed-issues/issue_2365.hrx"

// Ignoring "issue_2365", error tests are not supported yet.

mod issue_2366;

// From "sass-spec/spec/libsass-closed-issues/issue_2369.hrx"

// Ignoring "issue_2369", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_2371.hrx"

// Ignoring "issue_2371", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_2374.hrx"
#[test]
#[ignore] // wrong result
fn issue_2374() {
    assert_eq!(
        rsass(
            "$colors: (\r\
            \n    yellow: #ffeb3b\r\
            \n);\r\
            \n@each $name, $color in $colors {\r\
            \n    $amount: 40;\r\
            \n    @for $i from 0 through 9 {\r\
            \n        .#{$name}-#{($i*100)} { background-color: lighten($color, $amount) };\r\
            \n        $amount: $amount - 2;\r\
            \n    }\r\
            \n}\r\
            \n\r\
            \n$colors: (\r\
            \n    yellow: yellow,\r\
            \n    red: red,\r\
            \n    blue: blue,\r\
            \n    \r\
            \n);\r\
            \n@each $name, $color in $colors {\r\
            \n    @for $i from 0 through 2 {\r\
            \n        .#{$name}-#{($i*100)} { \r\
            \n          background-color: lighten($color, 10) \r\
            \n        };\r\
            \n    }\r\
            \n}\r\
            \n\r\
            \n"
        )
        .unwrap(),
        ".yellow-0 {\
        \n  background-color: white;\
        \n}\
        \n.yellow-100 {\
        \n  background-color: #fffffd;\
        \n}\
        \n.yellow-200 {\
        \n  background-color: #fffef3;\
        \n}\
        \n.yellow-300 {\
        \n  background-color: #fffde8;\
        \n}\
        \n.yellow-400 {\
        \n  background-color: #fffcde;\
        \n}\
        \n.yellow-500 {\
        \n  background-color: #fffbd4;\
        \n}\
        \n.yellow-600 {\
        \n  background-color: #fffaca;\
        \n}\
        \n.yellow-700 {\
        \n  background-color: #fff9c0;\
        \n}\
        \n.yellow-800 {\
        \n  background-color: #fff7b5;\
        \n}\
        \n.yellow-900 {\
        \n  background-color: #fff6ab;\
        \n}\
        \n.yellow-0 {\
        \n  background-color: #ffff33;\
        \n}\
        \n.yellow-100 {\
        \n  background-color: #ffff33;\
        \n}\
        \n.yellow-200 {\
        \n  background-color: #ffff33;\
        \n}\
        \n.red-0 {\
        \n  background-color: #ff3333;\
        \n}\
        \n.red-100 {\
        \n  background-color: #ff3333;\
        \n}\
        \n.red-200 {\
        \n  background-color: #ff3333;\
        \n}\
        \n.blue-0 {\
        \n  background-color: #3333ff;\
        \n}\
        \n.blue-100 {\
        \n  background-color: #3333ff;\
        \n}\
        \n.blue-200 {\
        \n  background-color: #3333ff;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2376.hrx"
#[test]
fn issue_2376() {
    assert_eq!(
        rsass(
            ".test {\r\
            \n\tbackground:url(//img12.360buyimg.com/..);\r\
            \n\t.a{\r\
            \n\t\theight: 100px;\r\
            \n\t}\r\
            \n}"
        )
        .unwrap(),
        ".test {\
        \n  background: url(//img12.360buyimg.com/..);\
        \n}\
        \n.test .a {\
        \n  height: 100px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2382.hrx"
#[test]
fn issue_2382() {
    assert_eq!(
        rsass(
            ".test {\r\
            \n  font: normal normal 400 16px/calc(16px * 1.4) Oxygen;\r\
            \n}"
        )
        .unwrap(),
        ".test {\
        \n  font: normal normal 400 16px/calc(16px * 1.4) Oxygen;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_238760.hrx"

// Ignoring "issue_238760", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_239.hrx"
#[test]
#[ignore] // wrong result
fn issue_239() {
    assert_eq!(
        rsass(
            "$gutter: 100% / 36.2;\r\
            \n    $gutter_em: 1rem; //This needs to be rem to not mess up margins\r\
            \n\r\
            \n// This calculate the gutter\r\
            \n@function col_width($n, $use_calc: false) {\r\
            \n    $divisor: 12 / $n;\r\
            \n    @if ($use_calc) {\r\
            \n        $gutter_offset: $gutter_em * ($divisor - 1);\r\
            \n        @return calc((100% - #{$gutter_offset}) / #{$divisor});\r\
            \n    }\r\
            \n    @else {\r\
            \n        @return (100% - $gutter * ($divisor - 1)) / $divisor;\r\
            \n    }\r\
            \n}\r\
            \n\r\
            \n// Each number here becomes a grid: onecol, twocol etc. \r\
            \n$grids: one, two, three, four, five, six, seven, eight, nine, ten, eleven, twelve;\r\
            \n$i: 1;\r\
            \n@each $grid in $grids {\r\
            \n    .#{$grid}col {\r\
            \n        width: col_width( $i );\r\
            \n        width: col_width( $i, true );\r\
            \n    }\r\
            \n\r\
            \n    %#{$grid}col {\r\
            \n        width: col_width( $i );\r\
            \n        width: col_width( $i, true );\r\
            \n    }\r\
            \n    $i: $i + 1;\r\
            \n}"
        )
        .unwrap(),
        ".onecol {\
        \n  width: 5.8011049724%;\
        \n  width: calc((100% - 11rem) / 12);\
        \n}\
        \n.twocol {\
        \n  width: 14.364640884%;\
        \n  width: calc((100% - 5rem) / 6);\
        \n}\
        \n.threecol {\
        \n  width: 22.9281767956%;\
        \n  width: calc((100% - 3rem) / 4);\
        \n}\
        \n.fourcol {\
        \n  width: 31.4917127072%;\
        \n  width: calc((100% - 2rem) / 3);\
        \n}\
        \n.fivecol {\
        \n  width: 40.0552486188%;\
        \n  width: calc((100% - 1.4rem) / 2.4);\
        \n}\
        \n.sixcol {\
        \n  width: 48.6187845304%;\
        \n  width: calc((100% - 1rem) / 2);\
        \n}\
        \n.sevencol {\
        \n  width: 57.182320442%;\
        \n  width: calc((100% - 0.7142857143rem) / 1.7142857143);\
        \n}\
        \n.eightcol {\
        \n  width: 65.7458563536%;\
        \n  width: calc((100% - 0.5rem) / 1.5);\
        \n}\
        \n.ninecol {\
        \n  width: 74.3093922652%;\
        \n  width: calc((100% - 0.3333333333rem) / 1.3333333333);\
        \n}\
        \n.tencol {\
        \n  width: 82.8729281768%;\
        \n  width: calc((100% - 0.2rem) / 1.2);\
        \n}\
        \n.elevencol {\
        \n  width: 91.4364640884%;\
        \n  width: calc((100% - 0.0909090909rem) / 1.0909090909);\
        \n}\
        \n.twelvecol {\
        \n  width: 100%;\
        \n  width: calc((100% - 0rem) / 1);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2394.hrx"
#[test]
#[ignore] // unexepected error
fn issue_2394() {
    assert_eq!(
        rsass(
            "@mixin brokenTest($color: red, $variableArguments...) {\r\
            \n  $width: map-get(keywords($variableArguments), width);\r\
            \n  a {\r\
            \n    width: $width;\r\
            \n    color: $color;\r\
            \n  }\r\
            \n}\r\
            \n\r\
            \n@mixin workingTest($variableArguments...) {\r\
            \n  $width: map-get(keywords($variableArguments), width);\r\
            \n  $color: map-get(keywords($variableArguments), color);\r\
            \n  a {\r\
            \n    width: $width;\r\
            \n    color: $color;\r\
            \n  }\r\
            \n}\r\
            \n\r\
            \n@include brokenTest($width: 30px, $color: blue); // #1 fails\r\
            \n@include brokenTest($color: blue, $width: 30px); // #2 fails\r\
            \n@include brokenTest(blue, $width: 30px); // #3 works (!)\r\
            \n@include workingTest($width: 30px, $color: blue); // #4 works\r\
            \n@include workingTest($color: blue, $width: 30px); // #5 works\r\
            \n"
        )
        .unwrap(),
        "a {\
        \n  width: 30px;\
        \n  color: blue;\
        \n}\
        \na {\
        \n  width: 30px;\
        \n  color: blue;\
        \n}\
        \na {\
        \n  width: 30px;\
        \n  color: blue;\
        \n}\
        \na {\
        \n  width: 30px;\
        \n  color: blue;\
        \n}\
        \na {\
        \n  width: 30px;\
        \n  color: blue;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2399.hrx"
#[test]
#[ignore] // wrong result
fn issue_2399() {
    assert_eq!(
        rsass(
            ".thing {\r\
            \n\tcolor: black;\r\
            \n}\r\
            \n\r\
            \n.a,\r\
            \n.b,\r\
            \n.c,\r\
            \n.d,\r\
            \n.e {\r\
            \n\t&:not(.thing) { @extend .thing; }\r\
            \n}"
        )
        .unwrap(),
        ".thing, .a:not(.thing),\
        \n.b:not(.thing),\
        \n.c:not(.thing),\
        \n.d:not(.thing),\
        \n.e:not(.thing) {\
        \n  color: black;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2429.hrx"
#[test]
fn issue_2429() {
    assert_eq!(
        rsass(
            "input[type=url] {\r\
            \n  content: bar\r\
            \n}"
        )
        .unwrap(),
        "input[type=url] {\
        \n  content: bar;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2444.hrx"
#[test]
#[ignore] // unexepected error
fn issue_2444() {
    assert_eq!(
        rsass(
            "a {\
            \n  @at-root (with: rule) {\
            \n    b: c;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c;\
        \n}\
        \n"
    );
}

// Ignoring "issue_2446", tests with expected error not implemented yet.

// Ignoring "issue_245443", tests with expected error not implemented yet.

// From "sass-spec/spec/libsass-closed-issues/issue_246.hrx"
#[test]
fn issue_246() {
    assert_eq!(
        rsass(
            "$content-width: 960px;\r\
            \n\r\
            \n/* demo.css: */\r\
            \n.selector {\r\
            \n  padding: 0 calc(100%/2 - #{$content-width/2})\r\
            \n}\r\
            \n\r\
            \n\r\
            \n/* bin/sassc demo.scss */\r\
            \n.selector {\r\
            \n  padding: 0 calc(100%/2 - #{$content-width/2}); }"
        )
        .unwrap(),
        "/* demo.css: */\
        \n.selector {\
        \n  padding: 0 calc(100%/2 - 480px);\
        \n}\
        \n/* bin/sassc demo.scss */\
        \n.selector {\
        \n  padding: 0 calc(100%/2 - 480px);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2462.hrx"
#[test]
fn issue_2462() {
    assert_eq!(
        rsass(
            "b {\
            \n    color: lighten(Crimson, 10%);\
            \n}\
            \n"
        )
        .unwrap(),
        "b {\
        \n  color: #ed365b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2464.hrx"
#[test]
fn issue_2464() {
    assert_eq!(
        rsass(
            ":host(:not(.foo)) {\r\
            \n    left: 0;\r\
            \n}\r\
            \n\r\
            \nfoobar {\r\
            \n    :host(:not(.foo)) {\r\
            \n        left: 0;\r\
            \n    }\r\
            \n}"
        )
        .unwrap(),
        ":host(:not(.foo)) {\
        \n  left: 0;\
        \n}\
        \nfoobar :host(:not(.foo)) {\
        \n  left: 0;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2465.hrx"
#[test]
#[ignore] // wrong result
fn issue_2465() {
    assert_eq!(
        rsass(
            "foo {\
            \n  a: 4e2px;\
            \n  b: 5e-2px;\
            \n  c: 6e2px + 3px;\
            \n  d: 7e-2px + 3px;\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: 400px;\
        \n  b: 0.05px;\
        \n  c: 603px;\
        \n  d: 3.07px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2467.hrx"
#[test]
fn issue_2467() {
    assert_eq!(
        rsass(
            "foo {\
            \n  a: [footer-right] / 120px;\
            \n  b: [footer-right]/ 120px;\
            \n  c: [footer-right] /120px;\
            \n  d: [footer-right]/120px;\
            \n  e: [footer-right] / 120px 1fr;\
            \n  f: [footer-right]/ 120px 1fr;\
            \n  g: [footer-right] /120px 1fr;\
            \n  h: [footer-right]/120px 1fr;\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: [footer-right]/120px;\
        \n  b: [footer-right]/120px;\
        \n  c: [footer-right]/120px;\
        \n  d: [footer-right]/120px;\
        \n  e: [footer-right]/120px 1fr;\
        \n  f: [footer-right]/120px 1fr;\
        \n  g: [footer-right]/120px 1fr;\
        \n  h: [footer-right]/120px 1fr;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2468.hrx"
#[test]
#[ignore] // wrong result
fn issue_2468() {
    assert_eq!(
        rsass(
            "%matches {\
            \n  :matches(oh, no) {\
            \n      x: 1;\
            \n      y: 2;\
            \n  }\
            \n}\
            \nmatches {\
            \n  @extend %matches;\
            \n  @extend oh;\
            \n}\
            \n\
            \n%any {\
            \n  :any(oh, no) {\
            \n      x: 1;\
            \n      y: 2;\
            \n  }\
            \n}\
            \nany {\
            \n  @extend %any;\
            \n  @extend oh;\
            \n}\
            \n"
        )
        .unwrap(),
        "matches :matches(oh, any, matches, no) {\
        \n  x: 1;\
        \n  y: 2;\
        \n}\
        \nany :any(oh, any, matches, no) {\
        \n  x: 1;\
        \n  y: 2;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2472.hrx"
#[test]
#[ignore] // unexepected error
fn issue_2472() {
    assert_eq!(
        rsass(
            "@function dark(\r\
            \n  $color,\r\
            \n  $args...\r\
            \n) {\r\
            \n  @return call(\'darken\', $color, $args...);\r\
            \n}\r\
            \n\r\
            \n@function dark2(\r\
            \n  $args...\r\
            \n) {\r\
            \n  @return call(\'darken\', $args...);\r\
            \n}\r\
            \n\r\
            \n$arg: join((), 5%);\r\
            \n\r\
            \n.single {\r\
            \n  direct: darken(#102030, 5%);\r\
            \n  arg: darken(#102030, $arg...);\r\
            \n  call: call(\'darken\', #102030, $arg...);\r\
            \n  function: dark(#102030, 5%);\r\
            \n  function2: dark2(#102030, 5%);\r\
            \n}"
        )
        .unwrap(),
        ".single {\
        \n  direct: #0a131d;\
        \n  arg: #0a131d;\
        \n  call: #0a131d;\
        \n  function: #0a131d;\
        \n  function2: #0a131d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2480.hrx"
#[test]
#[ignore] // wrong result
fn issue_2480() {
    assert_eq!(
        rsass(
            "@mixin main(\r\
            \n  $param1: param1,\r\
            \n  $param2: param2,\r\
            \n  $param3: param3\r\
            \n) {\r\
            \n  param1-value: $param1;\r\
            \n  param2-value: $param2;\r\
            \n  param3-value: $param3;\r\
            \n}\r\
            \n\r\
            \n@mixin router($args...) {\r\
            \n  @if (true) { @include main($args...) }\r\
            \n  @else { @include main2($args...) }\r\
            \n}\r\
            \n\r\
            \n@mixin helper($args...) {\r\
            \n  @include router($param2: param__2, $args...)\r\
            \n}\r\
            \n\r\
            \n.ordinal-arguments {\r\
            \n  @include helper(param___1);\r\
            \n}\r\
            \n\r\
            \n.named-arguments {\r\
            \n  @include helper($param1: param___1);\r\
            \n}"
        )
        .unwrap(),
        ".ordinal-arguments {\
        \n  param1-value: param___1;\
        \n  param2-value: param__2;\
        \n  param3-value: param3;\
        \n}\
        \n.named-arguments {\
        \n  param1-value: param___1;\
        \n  param2-value: param__2;\
        \n  param3-value: param3;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2482.hrx"

// Ignoring "issue_2482", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_2509.hrx"
#[test]
#[ignore] // unexepected error
fn issue_2509() {
    assert_eq!(
        rsass(
            "[charset i] {\r\
            \n\tdisplay: block;\r\
            \n}\r\
            \n\r\
            \n[charset I] {\r\
            \n\tdisplay: block;\r\
            \n}\r\
            \n\r\
            \n[charset=\"utf-8\" i] {\r\
            \n\tdisplay: block;\r\
            \n}\r\
            \n\r\
            \n[charset=\"utf-8\" I] {\r\
            \n\tdisplay: block;\r\
            \n}"
        )
        .unwrap(),
        "[charset i] {\
        \n  display: block;\
        \n}\
        \n[charset I] {\
        \n  display: block;\
        \n}\
        \n[charset=\"utf-8\" i] {\
        \n  display: block;\
        \n}\
        \n[charset=\"utf-8\" I] {\
        \n  display: block;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2520.hrx"
#[test]
#[ignore] // wrong result
fn issue_2520() {
    assert_eq!(
        rsass(
            "// ----\r\
            \n// Sass (v3.4.21)\r\
            \n// Compass (v1.0.3)\r\
            \n// ----\r\
            \n\r\
            \n@function remove-modifiers($selector) {\r\
            \n    // convert selector to a string\r\
            \n    $selector: inspect(nth($selector, 1));\r\
            \n  \r\
            \n    $modifier: \'\';\r\
            \n  \r\
            \n    // Find out where the first modifier starts\r\
            \n    $modifier-index: str-index($selector, \'\"--\');\r\
            \n  \r\
            \n    @if $modifier-index {\r\
            \n      // Strip the first part of the selector up until the first modifier\r\
            \n      $modifier: str-slice($selector, $modifier-index);\r\
            \n      // Find out where the modifier ends\r\
            \n      $modifier-end: str-index($modifier, \'\"]\');\r\
            \n      // Isolate the modifier\r\
            \n      $modifier: str-slice($modifier, 0, $modifier-end);\r\
            \n      // Remove the modifier from the selector\r\
            \n      $selector: str-replace($selector, $modifier, \'\');\r\
            \n      // Remove junk characters\r\
            \n      $selector: str-replace($selector, \'[class*=]\', \'\');\r\
            \n      // Recurse the function to eliminate any remainig modifiers\r\
            \n      $selector: remove-modifiers($selector);\r\
            \n    }\r\
            \n  \r\
            \n    @return $selector;\r\
            \n  }\r\
            \n  \r\
            \n  @function remove-duplicates($list, $recursive: false) {\r\
            \n    $result: ();\r\
            \n    \r\
            \n    @each $item in $list {\r\
            \n      @if not index($result, $item) {\r\
            \n        @if length($item) > 1 and $recursive {\r\
            \n          $result: append($result, remove-duplicates($item, $recursive));\r\
            \n        }\r\
            \n        @else {\r\
            \n          $result: append($result, $item);\r\
            \n        }\r\
            \n      }\r\
            \n    }\r\
            \n    \r\
            \n    @return $result;\r\
            \n  }\r\
            \n  \r\
            \n  @function str-replace($string, $search, $replace) { \r\
            \n    $index: str-index($string, $search);\r\
            \n  \r\
            \n    @if $index {\r\
            \n      @return str-slice($string, 1, $index - 1) + $replace + str-replace(\r\
            \n        str-slice($string, $index + str-length($search)), $search, $replace\r\
            \n      );\r\
            \n    }\r\
            \n  \r\
            \n    @return $string;\r\
            \n  }\r\
            \n  \r\
            \n  @function module-tree($selector) {\r\
            \n    $parent-module: $module;\r\
            \n  \r\
            \n    // Remove any modifers\r\
            \n    $selectors: remove-modifiers($selector);\r\
            \n  \r\
            \n    // Remove any junk characters\r\
            \n    $selectors: str-replace($selectors, \'.\', \'\');\r\
            \n    $selectors: str-replace($selectors, \'[class*=\"--\', \'\');\r\
            \n    $selectors: str-replace($selectors, \'[class*=\"\', \'\');\r\
            \n    $selectors: str-replace($selectors, \'--\"]\', \'\');\r\
            \n    $selectors: str-replace($selectors, \'\"]\', \'\');\r\
            \n  \r\
            \n    // Spoof our modules into a list\r\
            \n    $selectors: str-replace($selectors, \' \', \', \');\r\
            \n    $selectors: selector-parse($selectors);\r\
            \n  \r\
            \n    @return $selectors;\r\
            \n  }\r\
            \n  \r\
            \n  @function list-remove($list, $value, $recursive: false) { \r\
            \n      $result: ();\r\
            \n  \r\
            \n      @for $i from 1 through length($list) {\r\
            \n          @if type-of(nth($list, $i)) == list and $recursive {\r\
            \n              $result: append($result, list-remove(nth($list, $i), $value, $recursive), comma);\r\
            \n          } @else if nth($list, $i) != $value {\r\
            \n              $result: append($result, nth($list, $i), comma);\r\
            \n          }\r\
            \n      }\r\
            \n  \r\
            \n      @return $result;\r\
            \n   }\r\
            \n   \r\
            \n  @function this($options...) {\r\
            \n    $value: map-get($config, $options...);\r\
            \n    $debug: true;\r\
            \n  \r\
            \n    $this: &;\r\
            \n  \r\
            \n    @if length($this) > 0 {\r\
            \n      @if str-index(inspect(nth($this, 1)), \'%\') == 1 {\r\
            \n        $debug: false;\r\
            \n      }\r\
            \n    }\r\
            \n  \r\
            \n    @if $debug and not $value and $value != false {\r\
            \n        @warn \'#{$options} not found in #{$module} config\';\r\
            \n    }\r\
            \n  \r\
            \n    @return $value;\r\
            \n  }\r\
            \n  \r\
            \n  @function config($map-old, $map-new) {\r\
            \n      // Merge default and custom options\r\
            \n      $map-merged: map-merge($map-old, $map-new);\r\
            \n    \r\
            \n      // Store config in global variable\r\
            \n      $config: $map-merged !global;\r\
            \n  \r\
            \n      // Return merged map\r\
            \n      @return $map-merged;\r\
            \n  }\r\
            \n  \r\
            \n  @mixin module($module: $module) {\r\
            \n    $nested: &;\r\
            \n  \r\
            \n    @if not $nested {\r\
            \n      $module: $module !global;\r\
            \n    }\r\
            \n    \r\
            \n    $selectors: ();\r\
            \n  \r\
            \n    @each $item in $module {\r\
            \n      $selectors: join($selectors, \'.#{$module}\', comma);\r\
            \n      $selectors: join($selectors, \'[class*=\"#{$module}--\"]\', comma);\r\
            \n    }\r\
            \n      \r\
            \n    #{$selectors} {\r\
            \n      @content;\r\
            \n    }\r\
            \n  }\r\
            \n  \r\
            \n  @mixin component($components: null, $glue: \'__\') {\r\
            \n      $selectors: \'[class*=\"#{$module}#{$glue}\"]\';\r\
            \n      $this: &;\r\
            \n      $tree: module-tree($this);\r\
            \n      $namespace: nth($tree, length($tree));\r\
            \n  \r\
            \n      @if $components {\r\
            \n        $selectors: ();\r\
            \n  \r\
            \n        @each $component in $components {\r\
            \n          $selectors: join(\r\
            \n            $selectors, \r\
            \n            \'.#{$namespace}#{$glue}#{$component}, [class*=\"#{$namespace}#{$glue}#{$component}-\"]\', \r\
            \n            comma\r\
            \n          );\r\
            \n        }\r\
            \n      }\r\
            \n  \r\
            \n      $parents: ();\r\
            \n  \r\
            \n      @each $selector in & {\r\
            \n        // spoof the selector into a list\r\
            \n        $selector: str-replace(inspect($selector), \' \', \', \');\r\
            \n        $selector: selector-parse($selector);\r\
            \n  \r\
            \n        // if the last item isn\'t a modifier, remove it\r\
            \n        @if not str-index(inspect(nth($selector, length($selector))), \'[class*=\"--\') {\r\
            \n          $selector: list-remove($selector, nth($selector, length($selector)));\r\
            \n        }\r\
            \n  \r\
            \n        // convert the selector back into a string\r\
            \n        @if length($selector) == 1 {\r\
            \n          $selector: nth($selector, 1);\r\
            \n        }\r\
            \n        $selector: str-replace(inspect($selector), \', \', \' \');\r\
            \n  \r\
            \n        // Re-build the parent selector\r\
            \n        $parents: append($parents, $selector, comma);\r\
            \n      }\r\
            \n  \r\
            \n      // remove duplicate selectors\r\
            \n      $parents: remove-duplicates($parents);\r\
            \n  \r\
            \n      @if length($parents) == 1 {\r\
            \n        $parents: nth($parents, 1);\r\
            \n      }\r\
            \n  \r\
            \n      @if ($parents == \'()\') {\r\
            \n        @at-root #{$selectors} {\r\
            \n          @content;\r\
            \n        }\r\
            \n      } @else {\r\
            \n        @at-root #{$parents} {\r\
            \n          #{$selectors} {\r\
            \n              @content;\r\
            \n          }\r\
            \n        }\r\
            \n      }\r\
            \n  \r\
            \n  }\r\
            \n  \r\
            \n  @mixin modifier($modifier) {\r\
            \n    $selectors: &;\r\
            \n  \r\
            \n    @if str-index(inspect(&), \'.#{$module}\') {\r\
            \n      $selectors: ();\r\
            \n  \r\
            \n      @for $i from 1 through length(&) {\r\
            \n        @if $i % 2 == 0 {\r\
            \n          $selectors: append($selectors, nth(&, $i), comma);\r\
            \n        }\r\
            \n      }\r\
            \n    }\r\
            \n  \r\
            \n    @at-root #{$selectors} {\r\
            \n      $modifier-selectors: ();\r\
            \n      \r\
            \n      @each $item in $modifier {\r\
            \n        $modifier-selectors: join(\r\
            \n          $modifier-selectors, \'&[class*=\"--#{$modifier}\"]\', comma\r\
            \n        );\r\
            \n      }\r\
            \n  \r\
            \n      #{$modifier-selectors} {\r\
            \n        @content;\r\
            \n      }\r\
            \n    }\r\
            \n  }\r\
            \n  \r\
            \n  @mixin button($custom:()) {\r\
            \n    $buttons: config((\r\
            \n      \'group-spacing\': 1em\r\
            \n    ), $custom);\r\
            \n  \r\
            \n    @include module(\'button\') {\r\
            \n      @include component(\'group\') {\r\
            \n        content: \'fizzbuzz\';\r\
            \n        @include module {\r\
            \n          margin-left: this(\'group-spacing\');\r\
            \n          &:first-child {\r\
            \n            margin-left: 0;\r\
            \n          }\r\
            \n        }\r\
            \n      }\r\
            \n    }\r\
            \n  }\r\
            \n  \r\
            \n  @include button();\r\
            \n  \r\
            \n  @include module(\'modal\') {\r\
            \n    @include modifier(\'animate\') {\r\
            \n      @include modifier(\'zoom\') {\r\
            \n        content: \"fizzbuzz\"\r\
            \n      }\r\
            \n    }\r\
            \n  }"
        )
        .unwrap(),
        ".button__group, [class*=\"button__group-\"] {\
        \n  content: \'fizzbuzz\';\
        \n}\
        \n.button__group .button, .button__group [class*=\"button--\"], [class*=\"button__group-\"] .button, [class*=\"button__group-\"] [class*=\"button--\"] {\
        \n  margin-left: 1em;\
        \n}\
        \n.button__group .button:first-child, .button__group [class*=\"button--\"]:first-child, [class*=\"button__group-\"] .button:first-child, [class*=\"button__group-\"] [class*=\"button--\"]:first-child {\
        \n  margin-left: 0;\
        \n}\
        \n[class*=\"modal--\"][class*=\"--animate\"][class*=\"--zoom\"] {\
        \n  content: \"fizzbuzz\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_254.hrx"
#[test]
fn issue_254() {
    assert_eq!(
        rsass(
            "@mixin simple-media-query($max-width, $min-width) {\r\
            \n      @media only screen and (max-width: $max-width) and (min-width: $min-width) {\r\
            \n        @content;\r\
            \n      }\r\
            \n}\r\
            \n\r\
            \n@mixin test($value) {\r\
            \n    border-color: $value;\r\
            \n}\r\
            \n\r\
            \nbody \r\
            \n{\r\
            \n    @include test(\"#ccc\");\r\
            \n    @include simple-media-query(900px, 400px) {\r\
            \n        border-color: black;\r\
            \n    }\r\
            \n}"
        )
        .unwrap(),
        "body {\
        \n  border-color: \"#ccc\";\
        \n}\
        \n@media only screen and (max-width: 900px) and (min-width: 400px) {\
        \n  body {\
        \n    border-color: black;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2560.hrx"
#[test]
fn issue_2560() {
    assert_eq!(
        rsass(
            "$x: 10px / 5px;\r\
            \n\r\
            \ntest {\r\
            \n    font-size: $x;\r\
            \n    font-size: #{$x};\r\
            \n}"
        )
        .unwrap(),
        "test {\
        \n  font-size: 2;\
        \n  font-size: 2;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2569.hrx"

// Ignoring "issue_2569", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_257.hrx"
#[test]
fn issue_257() {
    assert_eq!(
        rsass("body{background:blue; a{color:black;}}").unwrap(),
        "body {\
        \n  background: blue;\
        \n}\
        \nbody a {\
        \n  color: black;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2582.hrx"
#[test]
fn issue_2582() {
    assert_eq!(
        rsass(
            ".test {\r\
            \n  font-size: (16px / 16px) + 0em;\r\
            \n  font-size: (16px / 16px  + 0em);\r\
            \n  font-size: 16px / 16px  + 0em;\r\
            \n}"
        )
        .unwrap(),
        ".test {\
        \n  font-size: 1em;\
        \n  font-size: 1em;\
        \n  font-size: 1em;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2625.hrx"
#[test]
fn issue_2625() {
    assert_eq!(
        rsass(
            "something\\:{ padding: 2px; }\
            \n"
        )
        .unwrap(),
        "something\\: {\
        \n  padding: 2px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2633.hrx"
#[test]
#[ignore] // unexepected error
fn issue_2633() {
    assert_eq!(
        rsass(
            "$sel1: \'.something__child + .something__child--mod1\';\
            \n$sel2: \'.something__child ~ .something__child--mod2\';\
            \n$result1: selector-unify($sel1, $sel2);\
            \n\
            \n#{$result1} {\
            \n  /* nothing */\
            \n}\
            \n\
            \n.a {\
            \n  color: blue;\
            \n  & > * {\
            \n    @at-root #{selector-unify(&, \'.b\')} {\
            \n      color: red;\
            \n    }\
            \n  }\
            \n}\
            \n\
            \n.a, .b {\
            \n  color: blue;\
            \n  & > * {\
            \n    @at-root #{selector-unify(&, \'.c, .d\')} {\
            \n      color: red;\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".something__child + .something__child--mod1.something__child--mod2 {\
        \n  /* nothing */\
        \n}\
        \n.a {\
        \n  color: blue;\
        \n}\
        \n.a > .b {\
        \n  color: red;\
        \n}\
        \n.a, .b {\
        \n  color: blue;\
        \n}\
        \n.a > .c, .a > .d, .b > .c, .b > .d {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_267.hrx"
#[test]
fn issue_267() {
    assert_eq!(
        rsass(
            "$x: foo;\r\
            \n@keyframes $x {\r\
            \n  to {\r\
            \n    blah: blah;\r\
            \n  }\r\
            \n}"
        )
        .unwrap(),
        "@keyframes $x {\
        \n  to {\
        \n    blah: blah;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2697.hrx"
#[test]
fn issue_2697() {
    assert_eq!(
        rsass(
            ".Card {\
            \n  &:not(.is-open, .is-static) {\
            \n    .CardContents {\
            \n      display: none;\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".Card:not(.is-open, .is-static) .CardContents {\
        \n  display: none;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_274.hrx"
#[test]
#[ignore] // wrong result
fn issue_274() {
    assert_eq!(
        rsass(
            "input[type=submit],\r\
            \ninput[type=reset],\r\
            \ninput[type=button]\r\
            \n{\r\
            \n       filter:chroma(color=#000000);\r\
            \n}"
        )
        .unwrap(),
        "input[type=submit],\
        \ninput[type=reset],\
        \ninput[type=button] {\
        \n  filter: chroma(color=#000000);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2779.hrx"

// Ignoring "issue_2779", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_279.hrx"
#[test]
#[ignore] // wrong result
fn issue_279() {
    assert_eq!(
        rsass(
            ".theme {\
            \n  @import \"foo.scss\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".theme .test-hello, .theme .test-world {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2808.hrx"
#[test]
fn issue_2808() {
    assert_eq!(
        rsass(
            "test {\
            \n  content: str-slice(abcdef, -10, 2)\
            \n}\
            \n"
        )
        .unwrap(),
        "test {\
        \n  content: ab;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2863.hrx"

// From "sass-spec/spec/libsass-closed-issues/issue_2884.hrx"
#[test]
#[ignore] // wrong result
fn issue_2884() {
    assert_eq!(
        rsass(
            "$titles: \"foo\", \"bar\", \"BaZ\";\
            \n\
            \n%border {\
            \n        border: 1px solid;\
            \n}\
            \n\
            \n@mixin border-red {\
            \n        border-color: red;\
            \n}\
            \n\
            \n@mixin border-blue {\
            \n        border-color: blue;\
            \n}\
            \n\
            \n@each $t in $titles {\
            \n        p[title=\"#{$t}\" i] {\
            \n                @extend %border;\
            \n                @include border-red;\
            \n        }\
            \n        p[title=\"#{$t}\"] {\
            \n                @extend %border;\
            \n                @include border-blue;\
            \n        }\
            \n}\
            \n"
        )
        .unwrap(),
        "p[title=BaZ], p[title=BaZ i], p[title=bar], p[title=bar i], p[title=foo], p[title=foo i] {\
        \n  border: 1px solid;\
        \n}\
        \np[title=foo i] {\
        \n  border-color: red;\
        \n}\
        \np[title=foo] {\
        \n  border-color: blue;\
        \n}\
        \np[title=bar i] {\
        \n  border-color: red;\
        \n}\
        \np[title=bar] {\
        \n  border-color: blue;\
        \n}\
        \np[title=BaZ i] {\
        \n  border-color: red;\
        \n}\
        \np[title=BaZ] {\
        \n  border-color: blue;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_289.hrx"
#[test]
fn issue_289() {
    assert_eq!(
        rsass(
            "@import url(http://fonts.googleapis.com/css?family=Titillium+Web:400,300,200,600);"
        )
        .unwrap(),
        "@import url(http://fonts.googleapis.com/css?family=Titillium+Web:400,300,200,600);\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2959.hrx"
#[test]
#[ignore] // wrong result
fn issue_2959() {
    assert_eq!(
        rsass(
            "%color {\
            \n\tcolor: blue;\
            \n}\
            \n\
            \n@mixin getOverridedSelector {\
            \n\t&#{&} {\
            \n\t\t@content;\
            \n\t}\
            \n}\
            \n\
            \n.foo {\
            \n\t@include getOverridedSelector {\
            \n\t\t@extend %color;\
            \n\t}\
            \n}\
            \n\
            \n.bar {\
            \n\t@include getOverridedSelector {\
            \n\t\tcolor: red;\
            \n\t}\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo.foo {\
        \n  color: blue;\
        \n}\
        \n.bar.bar {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2975.hrx"
#[test]
fn issue_2975() {
    assert_eq!(
        rsass(
            "@mixin test($name: false) {\
            \n    $a: \"\";\
            \n    $b: \"\";\
            \n    @if $name {\
            \n        $a: \"-#{$name}\";    // works as expected\
            \n        $b: -$name;         // here occurs the bug\
            \n    } @else {\
            \n        $a: \"\";\
            \n        $b: \"\";\
            \n    }\
            \n    \
            \n    .test-a#{$a} {\
            \n        display: block;\
            \n    }\
            \n    .test-b#{$b} {\
            \n        display: block;\
            \n    }\
            \n}\
            \n\
            \n@include test;\
            \n@include test(asdf);\
            \n@include test(foo1);\
            \n@include test(bar1);\
            \n// @include test(\"foo2\");\
            \n// @include test(\"bar2\");\
            \n"
        )
        .unwrap(),
        ".test-a {\
        \n  display: block;\
        \n}\
        \n.test-b {\
        \n  display: block;\
        \n}\
        \n.test-a-asdf {\
        \n  display: block;\
        \n}\
        \n.test-b-asdf {\
        \n  display: block;\
        \n}\
        \n.test-a-foo1 {\
        \n  display: block;\
        \n}\
        \n.test-b-foo1 {\
        \n  display: block;\
        \n}\
        \n.test-a-bar1 {\
        \n  display: block;\
        \n}\
        \n.test-b-bar1 {\
        \n  display: block;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2980.hrx"
#[test]
fn issue_2980() {
    assert_eq!(
        rsass(
            "$config: (\
            \n        phone: (\
            \n                break-point-width:0px,\
            \n                break-point-name: xs\
            \n        ),\
            \n        tablet: (\
            \n                break-point-width:600px,\
            \n                break-point-name: sm\
            \n        ),\
            \n        laptop: (\
            \n                break-point-width:900px,\
            \n                break-point-name: md\
            \n        ),\
            \n        desktop: (\
            \n                break-point-width:1200px,\
            \n                break-point-name:lg\
            \n        ),\
            \n);\
            \n\
            \n@each $key, $map in $config {\
            \n  $break-point-width: map_get($map, break-point-width);\
            \n  $break-point-name: map_get($map, break-point-name);\
            \n  $infix: if($break-point-width == 0px, null, -$break-point-name);\
            \n      .foo#{$infix} {\
            \n        content: \'#{$break-point-name}\';\
            \n      }\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  content: \"xs\";\
        \n}\
        \n.foo-sm {\
        \n  content: \"sm\";\
        \n}\
        \n.foo-md {\
        \n  content: \"md\";\
        \n}\
        \n.foo-lg {\
        \n  content: \"lg\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_2994.hrx"
#[test]
#[ignore] // wrong result
fn issue_2994() {
    assert_eq!(
        rsass(
            ".one-screen-page {\
            \n\t@extend %context-dark;\
            \n}\
            \n\
            \n%context-dark {\
            \n\t.button-secondary-outline {\
            \n\t\t&:hover,\
            \n\t\t&:focus,\
            \n\t\t&:active,\
            \n\t\t&:hover {\
            \n\t\t\tcolor: #fca;\
            \n\t\t}\
            \n\t}\
            \n}\
            \n"
        )
        .unwrap(),
        ".one-screen-page .button-secondary-outline:hover, .one-screen-page .button-secondary-outline:focus, .one-screen-page .button-secondary-outline:active {\
        \n  color: #fca;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_308.hrx"
#[test]
fn issue_308() {
    assert_eq!(
        rsass(
            "$var: orange;\
            \n\
            \n.test {\
            \n  color: $var;\
            \n}\
            \n\
            \n.#{$var} {\
            \n  color: #C0362C;\
            \n}\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  color: orange;\
        \n}\
        \n.orange {\
        \n  color: #C0362C;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_309.hrx"
#[test]
fn issue_309() {
    assert_eq!(
        rsass(
            "$zzz: zzz;\r\
            \na[data-foo=\"#{$zzz}\"] { a: b; }"
        )
        .unwrap(),
        "a[data-foo=zzz] {\
        \n  a: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_312.hrx"
#[test]
fn issue_312() {
    assert_eq!(
        rsass(
            "@for $i from 0 through 10 {\r\
            \n    .foo [index = \"#{$i}\"] {\r\
            \n        transform: translateY($i * 100%);\r\
            \n    }\r\
            \n}"
        )
        .unwrap(),
        ".foo [index=\"0\"] {\
        \n  transform: translateY(0%);\
        \n}\
        \n.foo [index=\"1\"] {\
        \n  transform: translateY(100%);\
        \n}\
        \n.foo [index=\"2\"] {\
        \n  transform: translateY(200%);\
        \n}\
        \n.foo [index=\"3\"] {\
        \n  transform: translateY(300%);\
        \n}\
        \n.foo [index=\"4\"] {\
        \n  transform: translateY(400%);\
        \n}\
        \n.foo [index=\"5\"] {\
        \n  transform: translateY(500%);\
        \n}\
        \n.foo [index=\"6\"] {\
        \n  transform: translateY(600%);\
        \n}\
        \n.foo [index=\"7\"] {\
        \n  transform: translateY(700%);\
        \n}\
        \n.foo [index=\"8\"] {\
        \n  transform: translateY(800%);\
        \n}\
        \n.foo [index=\"9\"] {\
        \n  transform: translateY(900%);\
        \n}\
        \n.foo [index=\"10\"] {\
        \n  transform: translateY(1000%);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_338.hrx"
#[test]
#[ignore] // unexepected error
fn issue_338() {
    assert_eq!(
        rsass(
            "$list: (\"a\", \"b\");\
            \n\
            \ntest {\
            \n    content: if( length($list) > 2, nth($list, 3), nth($list, 1) );\
            \n}\
            \n"
        )
        .unwrap(),
        "test {\
        \n  content: \"a\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_344.hrx"
#[test]
fn issue_344() {
    assert_eq!(
        rsass(
            "$variable: 1;\
            \n\
            \n$foo: #{$variable}px;\
            \n$bar: #{1}px;\
            \n$baz: \"1px\";\
            \n\
            \ndiv {\
            \n  top: -$foo;\
            \n  top: -$bar;\
            \n  top: -$baz;\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  top: -1px;\
        \n  top: -1px;\
        \n  top: -\"1px\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_346.hrx"
#[test]
fn issue_346() {
    assert_eq!(
        rsass(
            "$mediaquery: \'and (min-width: 300px)\';\
            \n\
            \n@media all #{$mediaquery} {\
            \n  div {\
            \n    display: block;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@media all and (min-width: 300px) {\
        \n  div {\
        \n    display: block;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_349.hrx"
#[test]
fn issue_349() {
    assert_eq!(
        rsass(
            "div {\
            \n  blah: not true;\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  blah: false;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_368.hrx"
#[test]
fn issue_368() {
    assert_eq!(
        rsass(
            "@if true {\
            \n  div {\
            \n    background: green;\
            \n  }\
            \n}\
            \n@if not true {\
            \n  div {\
            \n    background: red;\
            \n  }\
            \n}\
            \n@if not not true {\
            \n  div {\
            \n    background: blue;\
            \n  }\
            \n}\
            \n@if not (true or false) {\
            \n  div {\
            \n    background: black;\
            \n  }\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  background: green;\
        \n}\
        \ndiv {\
        \n  background: blue;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_394.hrx"
#[test]
fn issue_394() {
    assert_eq!(
        rsass(
            "$list1: alpha beta gamma;\
            \n$list2: one two three;\
            \n\
            \n$map: (alpha: one, beta: two, gamma: three);\
            \n\
            \n.ma-list {\
            \n  @each $item1, $item2 in zip($list1, $list2) {\
            \n    #{$item1}: $item2;\
            \n  }\
            \n}\
            \n\
            \n.ma-map {\
            \n  @each $key, $value in $map {\
            \n    #{$key}: $value;\
            \n  }\
            \n}"
        )
        .unwrap(),
        ".ma-list {\
        \n  alpha: one;\
        \n  beta: two;\
        \n  gamma: three;\
        \n}\
        \n.ma-map {\
        \n  alpha: one;\
        \n  beta: two;\
        \n  gamma: three;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_424.hrx"
#[test]
fn issue_424() {
    assert_eq!(
        rsass(
            "footer {\r\
            \n    color: red;\r\
            \n}\r\
            \n\r\
            \n// Ampersand in SassScript:\r\
            \n/*.button {\r\
            \n    &-primary {\r\
            \n        background: orange;\r\
            \n    }\r\
            \n\r\
            \n    &-secondary {\r\
            \n        background: blue;\r\
            \n    }\r\
            \n}*/\r\
            \n\r\
            \n// Output:\r\
            \n.button-primary {\r\
            \n    background: orange;\r\
            \n}\r\
            \n\r\
            \n.button-secondary {\r\
            \n    background: blue;\r\
            \n}"
        )
        .unwrap(),
        "footer {\
        \n  color: red;\
        \n}\
        \n/*.button {\
        \n    &-primary {\
        \n        background: orange;\
        \n    }\
        \n    &-secondary {\
        \n        background: blue;\
        \n    }\
        \n}*/\
        \n.button-primary {\
        \n  background: orange;\
        \n}\
        \n.button-secondary {\
        \n  background: blue;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_435.hrx"
#[test]
fn issue_435() {
    assert_eq!(
        rsass(
            "$skin-name: \"CMS_Black\";\r\
            \n\r\
            \n$QUOTE: unquote(\'\"\');\r\
            \n$EMPTY_STRING: unquote( \"\" );\r\
            \n$SLASH: unquote(\"/\");\r\
            \n\r\
            \n$SKINS_PATH: unquote(\"/CMS/Skins\");\r\
            \n$URL_SEPARATOR: $SLASH;\r\
            \n$URL_PREFIX: $EMPTY_STRING;\r\
            \n$URL_SUFFIX: $EMPTY_STRING;\r\
            \n\r\
            \n$_URL_PREFIX: $URL_PREFIX + $EMPTY_STRING;\r\
            \n$_URL_SUFFIX: $URL_SUFFIX + $EMPTY_STRING;\r\
            \n$_URL_SEPARATOR: $URL_SEPARATOR + $EMPTY_STRING;\r\
            \n$_SKINS_PATH: $SKINS_PATH + $EMPTY_STRING;\r\
            \n\r\
            \n@function webresource-image-url( $skin, $control, $file ) \r\
            \n{\r\
            \n\t$_url: $EMPTY_STRING;\r\
            \n\t$_path: $_SKINS_PATH $skin $control;\r\
            \n\r\
            \n\t@each $_part in $_path {\r\
            \n\t\t$_url: $_url + $_part + $_URL_SEPARATOR\r\
            \n\t}\r\
            \n\r\
            \n\t@return $_URL_PREFIX + $QUOTE + $_url + $file + $QUOTE + $_URL_SUFFIX;\r\
            \n}\r\
            \n\r\
            \n@function global-image-url( $skin, $control, $file ) {\r\
            \n\t@return webresource-image-url( $skin, $control, $file );\r\
            \n}\r\
            \n\r\
            \n@function skin-image-url( $control, $file ) {\r\
            \n\t@return global-image-url( $skin-name, $control, $file );\r\
            \n}\r\
            \n\r\
            \n$actions-sprite: skin-image-url( \"Common\", \"radActionsSprite.png\" );\r\
            \n\r\
            \n.test \r\
            \n{\r\
            \n\tbackground-image: url( $actions-sprite );\r\
            \n}\r\
            \n\r\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  background-image: url(\"/CMS/Skins/CMS_Black/Common/radActionsSprite.png\");\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_439.hrx"
#[test]
#[ignore] // wrong result
fn issue_439() {
    assert_eq!(
        rsass(
            "@mixin odd( $selector, $n) {\
            \n  $selector: \"& + \" + $selector + \" + \" + $selector;\
            \n  $placeholder: unique_id();\
            \n  %#{$placeholder} { @content; }\
            \n  #{$selector}:first-child {\
            \n    #{$selector} { @extend %#{$placeholder}; }\
            \n  }\
            \n}\
            \n\
            \nul > {\
            \n  @include odd( li, 5 ) { background: #ccc;  }\
            \n}\
            \n"
        )
        .unwrap(),
        "ul > + li + li:first-child + li + li {\
        \n  background: #ccc;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_442.hrx"
#[test]
fn issue_442() {
    assert_eq!(
        rsass(
            "$lhs: (100/10)#{rem};\
            \n$rhs: 10rem;\
            \n\
            \nfoo {\
            \n  a: $lhs;\
            \n  a: $rhs;\
            \n  a: $lhs == $rhs;\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: 10 rem;\
        \n  a: 10rem;\
        \n  a: false;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_45.hrx"
#[test]
#[ignore] // unexepected error
fn issue_45() {
    assert_eq!(
        rsass(
            "p:after {\r\
            \ncontent:url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAACXBIWXMAAAsTAAALEwEAmpwYAAAABGdBTUEAALGOfPtRkwAAACBjSFJNAAB6JQAAgIMAAPn/AACA6QAAdTAAAOpgAAA6mAAAF2+SX8VGAAACeElEQVR42nySy29McRTHP/fOnTvT6bQNrdHKMGGhFkTSELGxwoJY8Q9YWFhYEUJsRSKCsJWikjYSJBIbinpVPJLSRlEkKK2WTnXmvl+/Y4F4tPVJPqtvzjcnJ0cTEQxdY/miFH6gcAJpaWrQl86t05rR9axSKD8UZ6KqJscm5bMdyDDgAYgIBoCORm2G1u0b6w8unJ/bmDG1QtpUmIYiZ8Zk0zEpYmW76tujV9J3/Ep04v0XdR2IDYAdWxYt27Sa8/l8btWIlaYSupgqpNaMUYbC0DUa8qKXWpLGNSvZEETpZO/Z4B5gGQCRMio1xdVfioUIa3AQJ/ZARWhJgkQJKq3wfJ3RwETGhRtPgx7ABtBEhCVNBqViU2tn5+5bLfXmgurIYwJrGFEJmqZh2T4jo2X0YIreZ+7dfeejrcCEiKADfCon3O4fHzp25Nx+8nnqF65lXnEphQUtNBYKaKkMcRgxVY29093JUWCCn+gAORMaTLh0dbCjo/1KO3X1kC6BGIR+QLVioSc+F+9HnW/G1DX+QAcw0j8c/QaHj3UfeN0/MMicEmSL+J5P6DkMDUcfLvZGJ4FwWoHl/lAEXo344zv3dO3ynXJIpg7XdnBtj46bwSnblwH+QQdQ8lsNeNg32nOm/fIh3CGS0OXOQHCv90XYwUyICM2NNX85f26WUnOu5smFzX0vu9qktZjeNtusAbB+XdvfAWDZnjeurX2XST1Y8X6s7zmzYABUrHBaYNshYRC4k340FcZU/1vg2JVpgeP4uJXypHK8soD134In/W+mb+AJvffvvC022It/ve1MaCJCXU6f4UCQy1CbNVONH7/Gw7Md8fsAtddMUh5fveYAAAAASUVORK5CYII=);\r\
            \n}"
        )
        .unwrap(),
        "p:after {\
        \n  content: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAACXBIWXMAAAsTAAALEwEAmpwYAAAABGdBTUEAALGOfPtRkwAAACBjSFJNAAB6JQAAgIMAAPn/AACA6QAAdTAAAOpgAAA6mAAAF2+SX8VGAAACeElEQVR42nySy29McRTHP/fOnTvT6bQNrdHKMGGhFkTSELGxwoJY8Q9YWFhYEUJsRSKCsJWikjYSJBIbinpVPJLSRlEkKK2WTnXmvl+/Y4F4tPVJPqtvzjcnJ0cTEQxdY/miFH6gcAJpaWrQl86t05rR9axSKD8UZ6KqJscm5bMdyDDgAYgIBoCORm2G1u0b6w8unJ/bmDG1QtpUmIYiZ8Zk0zEpYmW76tujV9J3/Ep04v0XdR2IDYAdWxYt27Sa8/l8btWIlaYSupgqpNaMUYbC0DUa8qKXWpLGNSvZEETpZO/Z4B5gGQCRMio1xdVfioUIa3AQJ/ZARWhJgkQJKq3wfJ3RwETGhRtPgx7ABtBEhCVNBqViU2tn5+5bLfXmgurIYwJrGFEJmqZh2T4jo2X0YIreZ+7dfeejrcCEiKADfCon3O4fHzp25Nx+8nnqF65lXnEphQUtNBYKaKkMcRgxVY29093JUWCCn+gAORMaTLh0dbCjo/1KO3X1kC6BGIR+QLVioSc+F+9HnW/G1DX+QAcw0j8c/QaHj3UfeN0/MMicEmSL+J5P6DkMDUcfLvZGJ4FwWoHl/lAEXo344zv3dO3ynXJIpg7XdnBtj46bwSnblwH+QQdQ8lsNeNg32nOm/fIh3CGS0OXOQHCv90XYwUyICM2NNX85f26WUnOu5smFzX0vu9qktZjeNtusAbB+XdvfAWDZnjeurX2XST1Y8X6s7zmzYABUrHBaYNshYRC4k340FcZU/1vg2JVpgeP4uJXypHK8soD134In/W+mb+AJvffvvC022It/ve1MaCJCXU6f4UCQy1CbNVONH7/Gw7Md8fsAtddMUh5fveYAAAAASUVORK5CYII=);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_453.hrx"
#[test]
fn issue_453() {
    assert_eq!(
        rsass(
            "div {\
            \n    --a: 2px;\
            \n    top: var(--a);\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  --a: 2px;\
        \n  top: var(--a);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_456.hrx"
#[test]
fn issue_456() {
    assert_eq!(
        rsass(
            "body {\
            \n  -webkit-filter: invert(100%);\
            \n}\
            \n"
        )
        .unwrap(),
        "body {\
        \n  -webkit-filter: invert(100%);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_469.hrx"
#[test]
#[ignore] // wrong result
fn issue_469() {
    assert_eq!(
        rsass(
            "/*!\
            \n*/\
            \n\
            \n@charset \"utf-8\";\
            \n\
            \na {\
            \n  color: red;\
            \n}\
            \n\
            \n@import url(\"x\");\
            \n"
        )
        .unwrap(),
        "/*!\
        \n*/\
        \n@import url(\"x\");\
        \na {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_472.hrx"
#[test]
#[ignore] // wrong result
fn issue_472() {
    assert_eq!(
        rsass(
            "div {\
            \n  display: block;\
            \n  @keyframes {\
            \n    from {\
            \n      foo: bar;\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  display: block;\
        \n}\
        \n@keyframes {\
        \n  from {\
        \n    foo: bar;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_478.hrx"
#[test]
fn issue_478() {
    assert_eq!(
        rsass(
            "$x: \"x\";\
            \n$y: \"y\";\
            \n#{$x}--#{$y} {\
            \n  a: 1\
            \n}\
            \n"
        )
        .unwrap(),
        "x--y {\
        \n  a: 1;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_485.hrx"
#[test]
#[ignore] // wrong result
fn issue_485() {
    assert_eq!(
        rsass(
            "@media not all and (monochrome) { a {foo: bar} }\
            \n@media not screen and (color), print and (color) { a {foo: bar} }\
            \n@media (not (screen and (color))), print and (color) { a {foo: bar} }\
            \n"
        )
        .unwrap(),
        "@media not all and (monochrome) {\
        \n  a {\
        \n    foo: bar;\
        \n  }\
        \n}\
        \n@media not screen and (color), print and (color) {\
        \n  a {\
        \n    foo: bar;\
        \n  }\
        \n}\
        \n@media (false), print and (color) {\
        \n  a {\
        \n    foo: bar;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_487.hrx"
#[test]
fn issue_487() {
    assert_eq!(
        rsass(
            "\
            \n@mixin flex($grow: 1, $shrink: null, $basis: null) {\
            \n  -webkit-box-flex: $grow;\
            \n  -webkit-flex: $grow $shrink $basis;\
            \n  -moz-box-flex: $grow;\
            \n  -moz-flex: $grow $shrink $basis;\
            \n  -ms-flex: $grow $shrink $basis;\
            \n  flex: $grow $shrink $basis;\
            \n}\
            \n\
            \n[flex] {\
            \n  @include flex;\
            \n}\
            \n"
        )
        .unwrap(),
        "[flex] {\
        \n  -webkit-box-flex: 1;\
        \n  -webkit-flex: 1;\
        \n  -moz-box-flex: 1;\
        \n  -moz-flex: 1;\
        \n  -ms-flex: 1;\
        \n  flex: 1;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_492.hrx"
#[test]
fn issue_492() {
    assert_eq!(
        rsass(
            "$map: (\
            \n  foo: bar,\
            \n  baz: monkey,\
            \n);\
            \n\
            \n.css {\
            \n  @each $key, $value in $map {\
            \n    #{$key}: $value;\
            \n  }\
            \n}\
            \n\
            \n$list: one two, three four five, six seven;\
            \n\
            \n.list {\
            \n  @each $foo, $bar, $baz in $list {\
            \n    #{$foo}: $bar $baz;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".css {\
        \n  foo: bar;\
        \n  baz: monkey;\
        \n}\
        \n.list {\
        \n  one: two;\
        \n  three: four five;\
        \n  six: seven;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_495.hrx"
#[test]
fn issue_495() {
    assert_eq!(
        rsass(
            "/* Testing to make sure that a trailing comma doesn\'t break the tests */\
            \n$map: (\
            \n  hello: world,\
            \n);\
            \n"
        )
        .unwrap(),
        "/* Testing to make sure that a trailing comma doesn\'t break the tests */\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_502.hrx"
#[test]
#[ignore] // unexepected error
fn issue_502() {
    assert_eq!(
        rsass(
            "$a: 1;;\
            \n;;\
            \n"
        )
        .unwrap(),
        ""
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_506.hrx"
#[test]
fn issue_506() {
    assert_eq!(
        rsass(
            "$list: foo bar baz;\
            \n$list--comma: foo, bar, baz;\
            \n$single: foo;\
            \n\
            \ndiv {\
            \n  _list-space: list-separator($list);\
            \n  _list-comma: list-separator($list--comma);\
            \n  _single-item: list-separator($single);\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  _list-space: space;\
        \n  _list-comma: comma;\
        \n  _single-item: space;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_509.hrx"
#[test]
#[ignore] // unexepected error
fn issue_509() {
    assert_eq!(
        rsass(
            "$foo: (\
            \n    (key1): (value-1-0),\
            \n    key2: value-2-0,\
            \n    (key6): (value-6-0),\
            \n    key-3-0 key-3-1 key-3-2: value-3-0 value-3-1 value-3-2,\
            \n    key4: (value-4-0, value-4-1, value-4-2),\
            \n    key5: (key-5-0: value-5-1),\
            \n    (key-7-0: key-7-1): (value-7-0: value-7-1),\
            \n    (key-8-0, key-8-1, key-8-2): (value-8-0, value-8-1, value-8-2),\
            \n);\
            \n\
            \ndiv {\
            \n    foo: map-get((foo: 1, bar: 2), foo);\
            \n    foo: map-get((foo: 1, bar: 2), bar);\
            \n    foo: map-get((foo: 1, bar: 2), baz);\
            \n    foo: map-get((), foo);\
            \n    foo: map-get($foo, (key-5-0: value-5-1));\
            \n    foo: map-get($foo, (key2));\
            \n    foo: map-get($foo, (key-3-0 key-3-1 key-3-2));\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  foo: 1;\
        \n  foo: 2;\
        \n  foo: value-2-0;\
        \n  foo: value-3-0 value-3-1 value-3-2;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_510.hrx"
#[test]
fn issue_510() {
    assert_eq!(
        rsass(
            "$before: map-remove((foo: 1, bar: 2, baz: 3, burp: 4), bar, baz);\
            \n$after: (foo: 1, burp: 4);\
            \n\
            \ndiv {\
            \n  foo: $before == $after;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  foo: true;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_512.hrx"
#[test]
fn issue_512() {
    assert_eq!(
        rsass(
            "$list: a b c;\
            \n.css {\
            \n  debug: index($list, a);\
            \n\
            \n  @if type-of(index($list, 2)) == \"null\" {\
            \n    debug: foo;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".css {\
        \n  debug: 1;\
        \n  debug: foo;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_534.hrx"
#[test]
fn issue_534() {
    assert_eq!(
        rsass(
            "$foo: (\
            \n    1: foo1 bar1,\
            \n    10: foo2 bar2,\
            \n    100: foo3 bar3,\
            \n);\
            \n\
            \ndiv {\
            \n    foo: map-get($foo, 1);\
            \n    foo: map-get($foo, 10);\
            \n    foo: map-get($foo, 100);\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  foo: foo1 bar1;\
        \n  foo: foo2 bar2;\
        \n  foo: foo3 bar3;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_535.hrx"
#[test]
fn issue_535() {
    assert_eq!(
        rsass(
            "$width: 10;\
            \n\
            \n.test {\
            \n  margin-left: - 54 * $width - 1;\
            \n}\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  margin-left: -541;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_54.hrx"
#[test]
#[ignore] // wrong result
fn issue_54() {
    assert_eq!(
        rsass(
            "@mixin opacity($percent) {\r\
            \n  foo { test: opacity($percent); }\r\
            \n}\r\
            \n\r\
            \n@-webkit-keyframes uiDelayedFadeIn {\r\
            \n  0% { @include opacity(0.01); }\r\
            \n  50% { @include opacity(0.01); }\r\
            \n  100% { @include opacity(1); }\r\
            \n}\r\
            \n\r\
            \n@-webkit-keyframes bounce {\r\
            \n  from {\r\
            \n    left: 0px;\r\
            \n  }\r\
            \n  to {\r\
            \n    left: 200px;\r\
            \n  }\r\
            \n}\r\
            \n"
        )
        .unwrap(),
        "@-webkit-keyframes uiDelayedFadeIn {\
        \n  0% {\
        \n    foo {\
        \n      test: opacity(0.01);\
        \n    }\
        \n  }\
        \n  50% {\
        \n    foo {\
        \n      test: opacity(0.01);\
        \n    }\
        \n  }\
        \n  100% {\
        \n    foo {\
        \n      test: opacity(1);\
        \n    }\
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
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_548.hrx"
#[test]
fn issue_548() {
    assert_eq!(
        rsass(
            ".parent-sel-value {\
            \n  font-family: &;\
            \n  .parent-sel-interpolation {\
            \n    font-family: #{&};\
            \n     .parent-sel-value-concat {\
            \n        font-family: \"Current parent: \" + &;\
            \n     }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".parent-sel-value {\
        \n  font-family: .parent-sel-value;\
        \n}\
        \n.parent-sel-value .parent-sel-interpolation {\
        \n  font-family: .parent-sel-value .parent-sel-interpolation;\
        \n}\
        \n.parent-sel-value .parent-sel-interpolation .parent-sel-value-concat {\
        \n  font-family: \"Current parent: .parent-sel-value .parent-sel-interpolation .parent-sel-value-concat\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_549.hrx"
#[test]
#[ignore] // wrong result
fn issue_549() {
    assert_eq!(
        rsass(
            "$value: 10;\
            \n\
            \nfoo {\
            \n  filter: foo(opacity=$value*100);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  filter: foo(opacity=1000);\
        \n}\
        \n"
    );
}

mod issue_550;

// From "sass-spec/spec/libsass-closed-issues/issue_552.hrx"
#[test]
#[ignore] // wrong result
fn issue_552() {
    assert_eq!(
        rsass(
            "a,\
            \ndiv {\
            \n    top: 0;\
            \n}\
            \n\
            \n.a,\
            \n.b {\
            \n    &.c {\
            \n        color: red;\
            \n    }\
            \n}\
            \n"
        )
        .unwrap(),
        "a,\
        \ndiv {\
        \n  top: 0;\
        \n}\
        \n.a.c,\
        \n.b.c {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_553.hrx"
#[test]
#[ignore] // wrong result
fn issue_553() {
    assert_eq!(
        rsass(
            "$foo\\bar: 1;\
            \n\
            \n@function foo\\func() { @return 1; }\
            \n@mixin foo\\mixin() { mixin-value: 1; }\
            \n\
            \n.test {\
            \n    var-value: $foo\\bar;\
            \n    func-value: foo\\func();\
            \n    @include foo\\mixin();\
            \n}\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  var-value: 1;\
        \n  func-value: 1;\
        \n  mixin-value: 1;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_555.hrx"
#[test]
#[ignore] // wrong result
fn issue_555() {
    assert_eq!(
        rsass(
            "\
            \n@function hello($name) {\
            \n    @return $name;\
            \n}\
            \n\
            \n$foo: (\
            \n  bar() : baz,\
            \n  bar(\"foo\") : blah,\
            \n  hello(\"bob\") : bam,\
            \n);\
            \n\
            \na {\
            \n  foo: map-get($foo, \"bar()\");\
            \n  foo: map-get($foo, \"bar(\\\"foo\\\")\");\
            \n  foo: map-get($foo, \'bar(\"foo\")\');\
            \n  foo: map-get($foo, \"bob\");\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  foo: baz;\
        \n  foo: blah;\
        \n  foo: blah;\
        \n  foo: bam;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_556.hrx"
#[test]
#[ignore] // wrong result
fn issue_556() {
    assert_eq!(
        rsass(
            "$test: (\
            \n  one: 1,\
            \n  two: 2,\
            \n);\
            \n\
            \n$expect: (\
            \n  two: 2,\
            \n  one: 1,\
            \n);\
            \n\
            \n.test {\
            \n  equal: $test == $expect;\
            \n}\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  equal: true;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_557.hrx"
#[test]
fn issue_557() {
    assert_eq!(
        rsass(
            "\
            \na {\
            \n  foo: map-get((foo: 1, bar: 2), \"bar\");\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  foo: 2;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_558.hrx"
#[test]
fn issue_558() {
    assert_eq!(
        rsass(
            "@function is_gold($c) {\r\
            \n    @if ($c == gold) {\r\
            \n        @return \'yes\';\r\
            \n    }\r\
            \n    @return \'no\';\r\
            \n}\r\
            \n\r\
            \ndiv {\r\
            \n    foo: is_gold(gold);\r\
            \n    bar: is_gold(white);\r\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  foo: \"yes\";\
        \n  bar: \"no\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_56.hrx"
#[test]
fn issue_56() {
    assert_eq!(
        rsass(
            "@media (min-width: 980px) {\r\
            \n    a {\r\
            \n        color: red;\r\
            \n    }\r\
            \n}"
        )
        .unwrap(),
        "@media (min-width: 980px) {\
        \n  a {\
        \n    color: red;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_574.hrx"
#[test]
fn issue_574() {
    assert_eq!(
        rsass(
            "$flow: left;\
            \n\
            \n$map: (\
            \n  margin-#{$flow}: 3em,\
            \n  foo: bar,\
            \n);\
            \n\
            \n.test {\
            \n  margin-left: map-get($map, margin-left);\
            \n}\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  margin-left: 3em;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_575.hrx"
#[test]
fn issue_575() {
    assert_eq!(
        rsass(
            ".test {\
            \n  @if (foo: bar) == (foo: bar) {\
            \n    foo: bar;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  foo: bar;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_577.hrx"
#[test]
fn issue_577() {
    assert_eq!(
        rsass(
            "@function map-each($map) {\
            \n  $values: ();\
            \n\
            \n  @each $key, $value in $map {\
            \n    $values: append($values, $value);\
            \n  }\
            \n\
            \n  @return $values;\
            \n}\
            \n\
            \n$map: (foo: bar);\
            \n\
            \n.test {\
            \n  -map-test: map-each($map);\
            \n}\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  -map-test: bar;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_578.hrx"
#[test]
fn issue_578() {
    assert_eq!(
        rsass(
            "$list: one foo three bar six seven;\
            \n$pos: set-nth($list, 2, two);\
            \n$neg: set-nth($pos, -3, four five);\
            \n\
            \n.test {\
            \n  -positive: $pos;\
            \n  -negative: $neg;\
            \n}\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  -positive: one two three bar six seven;\
        \n  -negative: one two three four five six seven;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_579.hrx"
#[test]
#[ignore] // wrong result
fn issue_579() {
    assert_eq!(
        rsass(
            "$map: (\
            \n  foo: fump,\
            \n  bar: bump,\
            \n);\
            \n\
            \n@mixin vararg-test($foo, $bar) {\
            \n  foo: $foo;\
            \n  bar: $bar;\
            \n}\
            \n\
            \n.test {\
            \n  @include vararg-test($map...);\
            \n}\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  foo: fump;\
        \n  bar: bump;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_58.hrx"
#[test]
fn issue_58() {
    assert_eq!(
        rsass(
            "test {\r\
            \n  background: url(/static_loc/img/beta.png);\r\
            \n}"
        )
        .unwrap(),
        "test {\
        \n  background: url(/static_loc/img/beta.png);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_59.hrx"
#[test]
fn issue_59() {
    assert_eq!(
        rsass(
            "@mixin apply-to-ie6-only {\r\
            \n  * html {\r\
            \n    @content;\r\
            \n  }\r\
            \n}\r\
            \n@include apply-to-ie6-only {\r\
            \n  #logo {\r\
            \n    background-image: url(/logo.gif);\r\
            \n  }\r\
            \n}"
        )
        .unwrap(),
        "* html #logo {\
        \n  background-image: url(/logo.gif);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_590.hrx"
#[test]
fn issue_590() {
    assert_eq!(
        rsass(
            "foo {\
            \n  foo: 1/2;\
            \n  foo: 0.5;\
            \n  foo: (1/2);\
            \n  foo: 1/2 == 0.5;\
            \n  foo: (1/2) == 0.5;\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  foo: 1/2;\
        \n  foo: 0.5;\
        \n  foo: 0.5;\
        \n  foo: true;\
        \n  foo: true;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_592.hrx"
#[test]
#[ignore] // wrong result
fn issue_592() {
    assert_eq!(
        rsass(
            "%a::-webkit-scrollbar {\
            \n    color: green;\
            \n}\
            \n\
            \n.a {\
            \n    .b {\
            \n        @extend %a;\
            \n    }\
            \n\
            \n    .c .b {\
            \n        @extend %a;\
            \n    }\
            \n}\
            \n"
        )
        .unwrap(),
        ".a .c .b::-webkit-scrollbar, .a .b::-webkit-scrollbar {\
        \n  color: green;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_593.hrx"
#[test]
#[ignore] // wrong result
fn issue_593() {
    assert_eq!(
        rsass(
            "h1:nth-of-type(#{2 + \'n + 1\'}) {\
            \n    color: red;\
            \n}\
            \n\
            \nh1:nth-of-type(#{2 + \'n   +  1\'}) {\
            \n    color: red;\
            \n}\
            \n"
        )
        .unwrap(),
        "h1:nth-of-type(2n + 1) {\
        \n  color: red;\
        \n}\
        \nh1:nth-of-type(2n + 1) {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_595.hrx"
#[test]
fn issue_595() {
    assert_eq!(
        rsass(
            "a {\
            \n    color: red;\
            \n};\
            \n"
        )
        .unwrap(),
        "a {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_6.hrx"
#[test]
fn issue_6() {
    assert_eq!(
        rsass(
            "*[class|=\"has-background\"] {\r\
            \n    background: #efefef;\r\
            \n    padding: 7px;\r\
            \n    border: 1px solid #888;\r\
            \n    margin-bottom: 5px;\r\
            \n    }"
        )
        .unwrap(),
        "*[class|=has-background] {\
        \n  background: #efefef;\
        \n  padding: 7px;\
        \n  border: 1px solid #888;\
        \n  margin-bottom: 5px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_602"
#[test]
fn issue_602() {
    assert_eq!(
        rsass(
            "#foo.\\bar {\
            \n  color: red;\
            \n}\
            \n\
            \n#foo.b\\ar {\
            \n  color: red;\
            \n}\
            \n\
            \n#foo\\.bar {\
            \n  color: red;\
            \n}\
            \n\
            \n#foo\\bar {\
            \n  color: red;\
            \n}\
            \n\
            \n#fo\\o.bar {\
            \n  color: red;\
            \n}\
            \n"
        )
        .unwrap(),
        "@charset \"UTF-8\";\
        \n#foo.ºr {\
        \n  color: red;\
        \n}\
        \n#foo.b\\a r {\
        \n  color: red;\
        \n}\
        \n#foo\\.bar {\
        \n  color: red;\
        \n}\
        \n#fooºr {\
        \n  color: red;\
        \n}\
        \n#foo.bar {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_610.hrx"
#[test]
#[ignore] // wrong result
fn issue_610() {
    assert_eq!(
        rsass(
            "@mixin vararg-test($a, $b, $c, $d) {\
            \n  a: $a;\
            \n  b: $b;\
            \n  c: $c;\
            \n  d: $d;\
            \n}\
            \n\
            \nfoo {\
            \n  @include vararg-test(a, b, c, d);\
            \n}\
            \n\
            \nfoo {\
            \n  @include vararg-test(a b c d...);\
            \n}\
            \n\
            \nfoo {\
            \n  @include vararg-test((a b c d)...);\
            \n}\
            \n\
            \nfoo {\
            \n  @include vararg-test((a, b, c, d)...);\
            \n}\
            \n\
            \nfoo {\
            \n  @include vararg-test((a: a, b: b, c: c, d: d)...);\
            \n}\
            \n\
            \nfoo {\
            \n  @include vararg-test((\"a\": a, \"b\": b, \"c\": c, \"d\": d)...);\
            \n}\
            \n\
            \nfoo {\
            \n  @include vararg-test(a b..., (c: c, d: d)...);\
            \n}\
            \n\
            \nfoo {\
            \n  @include vararg-test(a, b c..., (d: d)...);\
            \n}\
            \n\
            \nfoo {\
            \n  @include vararg-test($c: c, (a: a, b: b, d: d)...);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: a;\
        \n  b: b;\
        \n  c: c;\
        \n  d: d;\
        \n}\
        \nfoo {\
        \n  a: a;\
        \n  b: b;\
        \n  c: c;\
        \n  d: d;\
        \n}\
        \nfoo {\
        \n  a: a;\
        \n  b: b;\
        \n  c: c;\
        \n  d: d;\
        \n}\
        \nfoo {\
        \n  a: a;\
        \n  b: b;\
        \n  c: c;\
        \n  d: d;\
        \n}\
        \nfoo {\
        \n  a: a;\
        \n  b: b;\
        \n  c: c;\
        \n  d: d;\
        \n}\
        \nfoo {\
        \n  a: a;\
        \n  b: b;\
        \n  c: c;\
        \n  d: d;\
        \n}\
        \nfoo {\
        \n  a: a;\
        \n  b: b;\
        \n  c: c;\
        \n  d: d;\
        \n}\
        \nfoo {\
        \n  a: a;\
        \n  b: b;\
        \n  c: c;\
        \n  d: d;\
        \n}\
        \nfoo {\
        \n  a: a;\
        \n  b: b;\
        \n  c: c;\
        \n  d: d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_613.hrx"
#[test]
fn issue_613() {
    assert_eq!(
        rsass(
            "$var: 1;\
            \n\
            \n@mixin test {\
            \n  $var: 2;\
            \n}\
            \n\
            \n@function test() {\
            \n  $var: 3;\
            \n  @return \"dummy\";\
            \n}\
            \n\
            \n.selector {\
            \n  $var: 4;\
            \n  @include test;\
            \n  $dummy: test();\
            \n  content: $var;\
            \n}\
            \n\
            \n.other-selector {\
            \n    content: $var;\
            \n}\
            \n"
        )
        .unwrap(),
        ".selector {\
        \n  content: 4;\
        \n}\
        \n.other-selector {\
        \n  content: 1;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_615.hrx"
#[test]
#[ignore] // wrong result
fn issue_615() {
    assert_eq!(
        rsass(
            "$foo: \"bar\";\
            \n%#{\"foo--#{$foo}\"} {\
            \n  foo: bar;\
            \n}\
            \n\
            \na {\
            \n  @extend %foo--bar;\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  foo: bar;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_622.hrx"
#[test]
fn issue_622() {
    assert_eq!(
        rsass(
            "@media screen {\
            \n    a {\
            \n        color: red;\
            \n    }\
            \n}\
            \n\
            \n.link {\
            \n    @media (foo: bar) {\
            \n        display: flex;\
            \n    }\
            \n}\
            \n"
        )
        .unwrap(),
        "@media screen {\
        \n  a {\
        \n    color: red;\
        \n  }\
        \n}\
        \n@media (foo: bar) {\
        \n  .link {\
        \n    display: flex;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_623.hrx"
#[test]
#[ignore] // wrong result
fn issue_623() {
    assert_eq!(
        rsass(
            "a {\
            \n  filter: alpha(opacity=.3); }\
            \n\
            \ndiv {\
            \n  filter: alpha(opacity=0.7); }\
            \n"
        )
        .unwrap(),
        "a {\
        \n  filter: alpha(opacity=0.3);\
        \n}\
        \ndiv {\
        \n  filter: alpha(opacity=0.7);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_628.hrx"

// Ignoring "issue_628", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_63.hrx"
#[test]
fn issue_63() {
    assert_eq!(
        rsass(
            "@mixin testComments {\r\
            \n\t/* crash */\r\
            \n\tp {\r\
            \n\t\twidth: 100px;\r\
            \n\t}\r\
            \n}\r\
            \n\r\
            \n@media screen and (orientation:landscape) {\r\
            \n\t@include testComments;\t\r\
            \n}\r\
            \n"
        )
        .unwrap(),
        "@media screen and (orientation: landscape) {\
        \n  /* crash */\
        \n  p {\
        \n    width: 100px;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_639.hrx"
#[test]
fn issue_639() {
    assert_eq!(
        rsass(
            "$quoted_list: \"foo\", \"bar\", \"baz\";\
            \n$unquoted_list: foo, bar, baz;\
            \n\
            \nfoo {\
            \n  foo: #{foo, bar, baz};\
            \n  foo: #{\"foo\", \"bar\", \"baz\"};\
            \n  foo: #{$quoted_list};\
            \n  foo: #{$unquoted_list};\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  foo: foo, bar, baz;\
        \n  foo: foo, bar, baz;\
        \n  foo: foo, bar, baz;\
        \n  foo: foo, bar, baz;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_64.hrx"
#[test]
fn issue_64() {
    assert_eq!(
        rsass(
            "$var: 10px;\r\
            \np {\r\
            \n\twidth: -$var;\r\
            \n}"
        )
        .unwrap(),
        "p {\
        \n  width: -10px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_641.hrx"
#[test]
fn issue_641() {
    assert_eq!(
        rsass(".#{\"foo\"}--1 { width:100%; }").unwrap(),
        ".foo--1 {\
        \n  width: 100%;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_643.hrx"
#[test]
fn issue_643() {
    assert_eq!(
        rsass(
            "$map: (foo: bar, bar: baz);\
            \n\
            \nfoo {\
            \n  a: nth($map, 2);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: bar baz;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_644.hrx"
#[test]
fn issue_644() {
    assert_eq!(
        rsass(
            "foo {\
            \n  background-image: url(foo/#{\"bar\"}/baz.jpg);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  background-image: url(foo/bar/baz.jpg);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_646.hrx"
#[test]
fn issue_646() {
    assert_eq!(
        rsass(
            "@function foo() {\
            \n  /* $bar: 1; */\
            \n @return true;\
            \n}\
            \n\
            \nfoo {\
            \n  foo: foo();\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  foo: true;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_652.hrx"
#[test]
fn issue_652() {
    assert_eq!(
        rsass(
            "$map: (\
            \n    purple: foo,\
            \n    rgba(1,2,3,1): bar,\
            \n    #ffffff: baz,\
            \n);\
            \n\
            \na {\
            \n  name: map-get($map, purple) == foo;\
            \n  func: map-get($map, rgba(1,2,3,1)) == bar;\
            \n  hex: map-get($map, #ffffff) == baz;\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  name: true;\
        \n  func: true;\
        \n  hex: true;\
        \n}\
        \n"
    );
}

mod issue_659;

// From "sass-spec/spec/libsass-closed-issues/issue_660.hrx"
#[test]
fn issue_660() {
    assert_eq!(
        rsass(
            "$foo: true;\
            \n\
            \ndiv {\
            \n  blah: $foo;\
            \n}\
            \n\
            \ndiv {\
            \n  blah: not $foo;\
            \n}\
            \n\
            \ndiv {\
            \n  blah: not ($foo);\
            \n}\
            \n\
            \ndiv {\
            \n  blah: not (true);\
            \n}\
            \n\
            \n"
        )
        .unwrap(),
        "div {\
        \n  blah: true;\
        \n}\
        \ndiv {\
        \n  blah: false;\
        \n}\
        \ndiv {\
        \n  blah: false;\
        \n}\
        \ndiv {\
        \n  blah: false;\
        \n}\
        \n"
    );
}

mod issue_666;

// From "sass-spec/spec/libsass-closed-issues/issue_67.hrx"
#[test]
fn issue_67() {
    assert_eq!(
        rsass("foo {bar: 70% - 40%}").unwrap(),
        "foo {\
        \n  bar: 30%;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_672.hrx"
#[test]
fn issue_672() {
    assert_eq!(
        rsass(
            "@mixin test($arglist...) {\
            \n    $map: keywords($arglist);\
            \n    answer: if($map, \"Yep\", \"Nope\");\
            \n}\
            \n\
            \nwith-keyword-args{\
            \n    @include test($arg1: one, $arg2: two, $arg3: three);\
            \n}\
            \nwith-no-args {\
            \n    @include test();\
            \n}\
            \nwithout-keyword-args {\
            \n    @include test(not-a-keyword-arg-1 , not-a-keyword-arg-2);\
            \n}\
            \n"
        )
        .unwrap(),
        "with-keyword-args {\
        \n  answer: \"Yep\";\
        \n}\
        \nwith-no-args {\
        \n  answer: \"Yep\";\
        \n}\
        \nwithout-keyword-args {\
        \n  answer: \"Yep\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_673.hrx"

// Ignoring "issue_673", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_674.hrx"
#[test]
fn issue_674() {
    assert_eq!(
        rsass(
            "\
            \n$base-path:\'../images/\';\
            \n$base-attr:\'data-\';\
            \n\
            \n@function url($src, $path:\'\'){\
            \n  @return unquote(\'url(\'+$base-path + $path+ $src +\')\');\
            \n}\
            \n@function url2($src, $path:\'\'){\
            \n  @return unquote(\'url(\'+ $base-path + $path+ $src +\')\');\
            \n}\
            \n@function attr($arg1, $arg2:\'\'){\
            \n  @return unquote(\'attr(\'+$base-attr + $arg1 + $arg2 +\')\');\
            \n}\
            \n\
            \ndiv {\
            \n    background: url(\'image.png\');\
            \n    background: url(\'image.png\',\'img/\');\
            \n    background: url2(\'image.png\',\'img/\');\
            \n\
            \n  &:after {\
            \n    content: attr(value);\
            \n    content: attr(value, -extra);\
            \n    content: url(\'icon.png\');\
            \n    content: url(\'icon.png\',\'gfx/\');\
            \n    content: url2(\'icon.png\',\'gfx/\');\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  background: url(../images/image.png);\
        \n  background: url(../images/img/image.png);\
        \n  background: url(../images/img/image.png);\
        \n}\
        \ndiv:after {\
        \n  content: attr(data-value);\
        \n  content: attr(data-value-extra);\
        \n  content: url(../images/icon.png);\
        \n  content: url(../images/gfx/icon.png);\
        \n  content: url(../images/gfx/icon.png);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_683.hrx"
#[test]
fn issue_683() {
    assert_eq!(
        rsass(
            "foo {\
            \n    filter: progid:DXImageTransform.Microsoft.AlphaImageLoader(src=\"data:image/png;base64,ABCD\",sizingMethod=crop);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  filter: progid:DXImageTransform.Microsoft.AlphaImageLoader(src=\"data:image/png;base64,ABCD\",sizingMethod=crop);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_688.hrx"
#[test]
fn issue_688() {
    assert_eq!(
        rsass(
            "test {\
            \n  /* Convert to px  */\
            \n  px-to-px: 0px + 1px;\
            \n  pt-to-px: 0px + 1pt;\
            \n  pc-to-px: 0px + 1pc;\
            \n  in-to-px: 0px + 1in;\
            \n  mm-to-px: 0px + 1mm;\
            \n  cm-to-px: 0px + 1cm;\
            \n  /* Convert to pt  */\
            \n  px-to-pt: 0pt + 1px;\
            \n  pt-to-pt: 0pt + 1pt;\
            \n  pc-to-pt: 0pt + 1pc;\
            \n  in-to-pt: 0pt + 1in;\
            \n  mm-to-pt: 0pt + 1mm;\
            \n  cm-to-pt: 0pt + 1cm;\
            \n  /* Convert to pc  */\
            \n  px-to-pc: 0pc + 1px;\
            \n  pt-to-pc: 0pc + 1pt;\
            \n  pc-to-pc: 0pc + 1pc;\
            \n  in-to-pc: 0pc + 1in;\
            \n  mm-to-pc: 0pc + 1mm;\
            \n  cm-to-pc: 0pc + 1cm;\
            \n  /* Convert to in  */\
            \n  px-to-in: 0in + 1px;\
            \n  pt-to-in: 0in + 1pt;\
            \n  pc-to-in: 0in + 1pc;\
            \n  in-to-in: 0in + 1in;\
            \n  mm-to-in: 0in + 1mm;\
            \n  cm-to-in: 0in + 1cm;\
            \n  /* Convert to mm  */\
            \n  px-to-mm: 0mm + 1px;\
            \n  pt-to-mm: 0mm + 1pt;\
            \n  pc-to-mm: 0mm + 1pc;\
            \n  in-to-mm: 0mm + 1in;\
            \n  mm-to-mm: 0mm + 1mm;\
            \n  cm-to-mm: 0mm + 1cm; \
            \n  /* Convert to cm  */\
            \n  px-to-cm: 0cm + 1px;\
            \n  pt-to-cm: 0cm + 1pt;\
            \n  pc-to-cm: 0cm + 1pc;\
            \n  in-to-cm: 0cm + 1in;\
            \n  mm-to-cm: 0cm + 1mm;\
            \n  cm-to-cm: 0cm + 1cm;   \
            \n}\
            \n"
        )
        .unwrap(),
        "test {\
        \n  /* Convert to px  */\
        \n  px-to-px: 1px;\
        \n  pt-to-px: 1.3333333333px;\
        \n  pc-to-px: 16px;\
        \n  in-to-px: 96px;\
        \n  mm-to-px: 3.7795275591px;\
        \n  cm-to-px: 37.7952755906px;\
        \n  /* Convert to pt  */\
        \n  px-to-pt: 0.75pt;\
        \n  pt-to-pt: 1pt;\
        \n  pc-to-pt: 12pt;\
        \n  in-to-pt: 72pt;\
        \n  mm-to-pt: 2.8346456693pt;\
        \n  cm-to-pt: 28.3464566929pt;\
        \n  /* Convert to pc  */\
        \n  px-to-pc: 0.0625pc;\
        \n  pt-to-pc: 0.0833333333pc;\
        \n  pc-to-pc: 1pc;\
        \n  in-to-pc: 6pc;\
        \n  mm-to-pc: 0.2362204724pc;\
        \n  cm-to-pc: 2.3622047244pc;\
        \n  /* Convert to in  */\
        \n  px-to-in: 0.0104166667in;\
        \n  pt-to-in: 0.0138888889in;\
        \n  pc-to-in: 0.1666666667in;\
        \n  in-to-in: 1in;\
        \n  mm-to-in: 0.0393700787in;\
        \n  cm-to-in: 0.3937007874in;\
        \n  /* Convert to mm  */\
        \n  px-to-mm: 0.2645833333mm;\
        \n  pt-to-mm: 0.3527777778mm;\
        \n  pc-to-mm: 4.2333333333mm;\
        \n  in-to-mm: 25.4mm;\
        \n  mm-to-mm: 1mm;\
        \n  cm-to-mm: 10mm;\
        \n  /* Convert to cm  */\
        \n  px-to-cm: 0.0264583333cm;\
        \n  pt-to-cm: 0.0352777778cm;\
        \n  pc-to-cm: 0.4233333333cm;\
        \n  in-to-cm: 2.54cm;\
        \n  mm-to-cm: 0.1cm;\
        \n  cm-to-cm: 1cm;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_690.hrx"
#[test]
fn issue_690() {
    assert_eq!(
        rsass(
            "test {\
            \n  left: expression(callSomeFunc());\
            \n  content: expression(\"Smile :-)\");\
            \n}\
            \n"
        )
        .unwrap(),
        "test {\
        \n  left: expression(callSomeFunc());\
        \n  content: expression(\"Smile :-)\");\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_694.hrx"
#[test]
fn issue_694() {
    assert_eq!(
        rsass(
            "// test for libsass 694:\
            \n// parser should be smarter about handling quoted quotes\
            \n\
            \n$str: \'{\' + \'\"foo\": \"bar\"\' + \'}\';\
            \n$str2: \'\"hello world\"\';\
            \n$str3: \"hello world\";\
            \n.interpolation-test {\
            \n  test: \"#{$str}\";\
            \n  test: \"#{$str2}\";\
            \n  test: \"#{$str3}\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".interpolation-test {\
        \n  test: \'{\"foo\": \"bar\"}\';\
        \n  test: \'\"hello world\"\';\
        \n  test: \"hello world\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_698.hrx"

// Ignoring "issue_698", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_699.hrx"
#[test]
#[ignore] // wrong result
fn issue_699() {
    assert_eq!(
        rsass(
            ".selector {\
            \n  color: invert(rebeccapurple);\
            \n}"
        )
        .unwrap(),
        ".selector {\
        \n  color: #99cc66;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_700.hrx"
#[test]
fn issue_700() {
    assert_eq!(
        rsass(
            ".selector {\
            \n  color: invert(transparent);\
            \n}"
        )
        .unwrap(),
        ".selector {\
        \n  color: rgba(255, 255, 255, 0);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_701.hrx"
#[test]
fn issue_701() {
    assert_eq!(
        rsass(
            ".test-1 {\
            \n  content: null;\
            \n  content: inspect(null);\
            \n  content: inspect(false);\
            \n  content: inspect(true);\
            \n  content: inspect(42);\
            \n  content: inspect(42.3);\
            \n  content: inspect(42px);\
            \n  content: inspect(\"string\");\
            \n  $list: 1, 2, 3;\
            \n  content: inspect($list);\
            \n  $map: ( a: 1, b: 2, c: 3 );\
            \n  content: inspect($map);\
            \n}\
            \n"
        )
        .unwrap(),
        ".test-1 {\
        \n  content: null;\
        \n  content: false;\
        \n  content: true;\
        \n  content: 42;\
        \n  content: 42.3;\
        \n  content: 42px;\
        \n  content: \"string\";\
        \n  content: 1, 2, 3;\
        \n  content: (a: 1, b: 2, c: 3);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_702.hrx"
#[test]
fn issue_702() {
    assert_eq!(
        rsass(
            ".foo {\
            \n  content: function-exists(\"feature-exists\");\
            \n  content: feature-exists(\"foo\");\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  content: true;\
        \n  content: false;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_703.hrx"
#[test]
fn issue_703() {
    assert_eq!(
        rsass(
            ".test-1 {\
            \n  @for $i from 1 through 3 {\
            \n    content: $i;\
            \n  }\
            \n}\
            \n\
            \n.test-2 {\
            \n  @for $i from 3 through 1 {\
            \n    content: $i;\
            \n  }\
            \n}\
            \n\
            \n.test-3 {\
            \n  @for $i from 1 to 3 {\
            \n    content: $i;\
            \n  }\
            \n}\
            \n\
            \n.test-4 {\
            \n  @for $i from 3 to 1 {\
            \n    content: $i;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".test-1 {\
        \n  content: 1;\
        \n  content: 2;\
        \n  content: 3;\
        \n}\
        \n.test-2 {\
        \n  content: 3;\
        \n  content: 2;\
        \n  content: 1;\
        \n}\
        \n.test-3 {\
        \n  content: 1;\
        \n  content: 2;\
        \n}\
        \n.test-4 {\
        \n  content: 3;\
        \n  content: 2;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_708.hrx"
#[test]
fn issue_708() {
    assert_eq!(
        rsass(
            "@function foobar($x, $y, $z : 3) {\
            \n  @return $x + $y * 2 + $z\
            \n}\
            \n\
            \n.foobar {\
            \n  content: foobar($y:2, $x:4);\
            \n  content: foobar($y: 2, $x: 4);\
            \n  content: foobar($y : 2, $x : 4);\
            \n  content: foobar($y:2px, $x:4);\
            \n  content: foobar($y: 2px, $x: 4);\
            \n  content: foobar($y : 2px, $x : 4);\
            \n}"
        )
        .unwrap(),
        ".foobar {\
        \n  content: 11;\
        \n  content: 11;\
        \n  content: 11;\
        \n  content: 11px;\
        \n  content: 11px;\
        \n  content: 11px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_712.hrx"

// Ignoring "issue_712", error tests are not supported yet.

mod issue_713;

// From "sass-spec/spec/libsass-closed-issues/issue_72.hrx"
#[test]
#[ignore] // wrong result
fn issue_72() {
    assert_eq!(
        rsass(
            "test {\r\
            \n  filter: progid:DXImageTransform.Microsoft.gradient( startColorstr=\'#223344\', endColorstr=\'#112233\',GradientType=0 );\r\
            \n}\r\
            \n\r\
            \n@mixin opacity($opacity) {\r\
            \n    opacity: $opacity / 100;\r\
            \n    filter: alpha(opacity=$opacity);\r\
            \n}"
        )
        .unwrap(),
        "test {\
        \n  filter: progid:DXImageTransform.Microsoft.gradient( startColorstr=\"#223344\", endColorstr=\"#112233\",GradientType=0 );\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_73.hrx"
#[test]
fn issue_73() {
    assert_eq!(
        rsass(
            "@mixin box-shadow($shadow...) { \r\
            \n  -webkit-box-shadow: $shadow;\r\
            \n     -moz-box-shadow: $shadow;\r\
            \n          box-shadow: $shadow;\r\
            \n}"
        )
        .unwrap(),
        ""
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_733.hrx"
#[test]
fn issue_733() {
    assert_eq!(
        rsass(
            "@function getter() {\
            \n  @return 42px;\
            \n}\
            \n\
            \ntest {\
            \n  content: getter()-1;\
            \n  content: getter()- 1;\
            \n  content: getter() -1;\
            \n}\
            \n"
        )
        .unwrap(),
        "test {\
        \n  content: 41px;\
        \n  content: 41px;\
        \n  content: 42px -1;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_736.hrx"
#[test]
fn issue_736() {
    assert_eq!(
        rsass(
            "// libsass issue 736:  @return does not cause function exit\
            \n// https://github.com/sass/libsass/issues/736\
            \n\
            \n@function contains-true($list) {\
            \n  @each $bool in $list {\
            \n    @if $bool {\
            \n      @return \"found true\";\
            \n    }\
            \n  }\
            \n  @return \"nothing found\";\
            \n}\
            \n\
            \n.test {\
            \n  out: contains-true(true false false);\
            \n  out: contains-true(false true false);\
            \n  out: contains-true(false false true);\
            \n}\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  out: \"found true\";\
        \n  out: \"found true\";\
        \n  out: \"found true\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_738.hrx"
#[test]
fn issue_738() {
    assert_eq!(
        rsass(
            ".foo {\
            \n  &--bar { color: red; }\
            \n  &--1bar { color: blue;}\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo--bar {\
        \n  color: red;\
        \n}\
        \n.foo--1bar {\
        \n  color: blue;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_740.hrx"
#[test]
fn issue_740() {
    assert_eq!(
        rsass(
            "$foo: null;\
            \n$foo: #fff !default;\
            \n$bar: #000;\
            \n$bar: #f00 !default;\
            \n\
            \nfoo {\
            \n  foo: $foo;\
            \n  bar: $bar;\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  foo: #fff;\
        \n  bar: #000;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_748.hrx"
#[test]
#[ignore] // wrong result
fn issue_748() {
    assert_eq!(
        rsass(
            "// problem: not expression is currently returning false on values other than true, false or null\
            \n\
            \n@function truthyfalsey($bool: null) {\
            \n  @if not $bool {\
            \n    @return falsey;\
            \n  } @else {\
            \n    @return truthy;\
            \n  }\
            \n}\
            \n\
            \n.test {\
            \n  debug: truthyfalsey(true); // expect truthy\
            \n  debug: truthyfalsey(false); // expect falsey\
            \n  debug: truthyfalsey(); // expect falsey (default arg is null)\
            \n  debug: truthyfalsey(5); // expect truthy\
            \n  debug: truthyfalsey(string); // expect truthy\
            \n  debug: truthyfalsey((alpha: 1, bravo: 2)); // expect truthy\
            \n  debug: truthyfalsey(this is a list); // expect truthy\
            \n  debug: truthyfalsey(\'true\'); // expect truthy\
            \n}\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  debug: truthy;\
        \n  debug: falsey;\
        \n  debug: falsey;\
        \n  debug: truthy;\
        \n  debug: truthy;\
        \n  debug: truthy;\
        \n  debug: truthy;\
        \n  debug: truthy;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_759.hrx"
#[test]
fn issue_759() {
    assert_eq!(
        rsass(
            "$a: 10px !global !default;\
            \n$b: 20px !default !global;\
            \n$c: 30px !default !default !default !global !global !global;\
            \n$d: 40px !global !global !global !default !default !default;\
            \n$e: 50px !global !default !global !default !global !default;\
            \n\
            \nfoo {\
            \n  a: $a;\
            \n  b: $b;\
            \n  c: $c;\
            \n  d: $d;\
            \n  e: $e;\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: 10px;\
        \n  b: 20px;\
        \n  c: 30px;\
        \n  d: 40px;\
        \n  e: 50px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_760.hrx"
#[test]
fn issue_760() {
    assert_eq!(
        rsass(
            "foo {\
            \n  quoted: str-slice(\"abcd\", 1, 0);\
            \n  unquoted: str-slice(abcd, 1, 0);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  quoted: \"\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_763.hrx"
#[test]
fn issue_763() {
    assert_eq!(
        rsass(
            "foo {\
            \n  a: str-slice(\"abcd\", 1, 1);\
            \n  b: str-slice(\'abcd\', 1, 1);\
            \n  c: str-slice(abcd, 1, 1);\
            \n\
            \n  d: str-insert(\"abcd\", \"X\", 1);\
            \n  e: str-insert(\"abcd\", \'X\', 1);\
            \n  f: str-insert(\'abcd\', \"X\", 1);\
            \n  g: str-insert(\'abcd\', \'X\', 1);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  a: \"a\";\
        \n  b: \"a\";\
        \n  c: a;\
        \n  d: \"Xabcd\";\
        \n  e: \"Xabcd\";\
        \n  f: \"Xabcd\";\
        \n  g: \"Xabcd\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_77.hrx"
#[test]
fn issue_77() {
    assert_eq!(
        rsass(
            "@mixin m {\r\
            \n  .m {\r\
            \n    color: red;\r\
            \n    @content;\r\
            \n  }\r\
            \n}\r\
            \ndiv.a {\r\
            \n  @include m;\r\
            \n}"
        )
        .unwrap(),
        "div.a .m {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_783"
#[test]
#[ignore] // wrong result
fn issue_783() {
    assert_eq!(
        rsass(
            "// $a: 12px / 1em;\
            \n// $b: 6px / 1em;\
            \n// $c: 10em;\
            \n// $x: -9999em;\
            \n// $aa: 1px * 1px;\
            \n\
            \na {\
            \n  $foo: 2em;\
            \n  $bar: 2em;\
            \n\
            \n  foo: $foo;          // 2em  ✔\
            \n  bar: $bar;          // 2em  ✔\
            \n  // a: $foo * $bar;     // 4em*em isn\'t a valid CSS value.  ✔\
            \n  a: $foo / $bar;     // 1  ✔\
            \n  a: $foo + $bar;     // 4em  ✔\
            \n  a: $foo - $bar;     // 0em  ✔\
            \n\
            \n\
            \n  $foo: 2px;\
            \n  $bar: 2em;\
            \n\
            \n  foo: $foo;          // 2px  ✔\
            \n  bar: $bar;          // 2em  ✔\
            \n  // a: $foo * $bar;     // 4em*px isn\'t a valid CSS value.  ✔\
            \n  // a: $foo / $bar;     // 1px/em isn\'t a valid CSS value.  ✔\
            \n  // a: $foo + $bar;     // Incompatible units: \'em\' and \'px\'.  ✔\
            \n  // a: $foo - $bar;     // Incompatible units: \'em\' and \'px\'.  ✔\
            \n\
            \n\
            \n  $foo: 2em;\
            \n  $bar: 2px;\
            \n\
            \n  foo: $foo;          // 2em  ✔\
            \n  bar: $bar;          // 2px  ✔\
            \n  // a: $foo * $bar;     // 4em*px isn\'t a valid CSS value.  ✔\
            \n  // a: $foo / $bar;     // 1em/px isn\'t a valid CSS value.  ✔\
            \n  // a: $foo + $bar;     // Incompatible units: \'px\' and \'em\'.  ✔\
            \n  // a: $foo - $bar;     // Incompatible units: \'px\' and \'em\'.  ✔\
            \n\
            \n\
            \n  $foo: 2px / 2em;\
            \n  $bar: 2px;\
            \n\
            \n  // foo: $foo;          // 1px/em isn\'t a valid CSS value.  ✔\
            \n  bar: $bar;          // 2px  ✔\
            \n  // a: $foo * $bar;     // 2px*px/em isn\'t a valid CSS value.  ✔\
            \n  // a: $foo / $bar;     // 0.5/em isn\'t a valid CSS value.  ✔\
            \n  // a: $foo + $bar;     // Incompatible units: \'\' and \'em\'.\
            \n  // a: $foo - $bar;     // Incompatible units: \'\' and \'em\'.\
            \n\
            \n\
            \n  $foo: 2em / 2px;\
            \n  $bar: 2px;\
            \n\
            \n  // foo: $foo;          // 1em/px isn\'t a valid CSS value.  ✔\
            \n  bar: $bar;          // 2px  ✔\
            \n  a: $foo * $bar;     // 2em  ✔\
            \n  // a: $foo / $bar;     // 0.5em/px*px isn\'t a valid CSS value.  ✔\
            \n  // a: $foo + $bar;     // Incompatible units: \'px\' and \'em\'.\
            \n  // a: $foo - $bar;     // Incompatible units: \'px\' and \'em\'.\
            \n\
            \n\
            \n  $foo: 2em / 2px;\
            \n  $bar: 2em / 2px;\
            \n\
            \n  // foo: $foo;          // 1em/px isn\'t a valid CSS value.  ✔\
            \n  // bar: $bar;          // 1em/px isn\'t a valid CSS value.  ✔\
            \n  // a: $foo * $bar;     // 1em*em/px*px isn\'t a valid CSS value.  ✔\
            \n  a: $foo / $bar;     // 1  ✔\
            \n  // a: $foo + $bar;     // 2em/px isn\'t a valid CSS value.  ✔\
            \n  // a: $foo - $bar;     // 0em/px isn\'t a valid CSS value.  ✔\
            \n\
            \n\
            \n  $foo: 2px / 2em;\
            \n  $bar: 2em / 2px;\
            \n\
            \n  // foo: $foo;          // 1px/em isn\'t a valid CSS value.  ✔\
            \n  // bar: $bar;          // 1em/px isn\'t a valid CSS value.  ✔\
            \n  a: $foo * $bar;     // 1  ✔\
            \n  // a: $foo / $bar;     // 1px*px/em*em isn\'t a valid CSS value.  ✔\
            \n  // a: $foo + $bar;     // Incompatible units: \'em\' and \'px\'.\
            \n  // a: $foo - $bar;     // Incompatible units: \'em\' and \'px\'.\
            \n\
            \n\
            \n  $foo: 2px;\
            \n  $bar: 2px / 2em;\
            \n\
            \n  foo: $foo;          // 2px  ✔\
            \n  // bar: $bar;          // 1px/em isn\'t a valid CSS value.  ✔\
            \n  // a: $foo * $bar;     // 2px*px/em isn\'t a valid CSS value.  ✔\
            \n  a: $foo / $bar;     // 2em  ✔\
            \n  // a: $foo + $bar;     // Incompatible units: \'em\' and \'\'.\
            \n  // a: $foo - $bar;     // Incompatible units: \'em\' and \'\'.\
            \n\
            \n\
            \n  $foo: 2px;\
            \n  $bar: 2em / 2px;\
            \n\
            \n  foo: $foo;          // 2px  ✔\
            \n  // bar: $bar;          // 1em/px isn\'t a valid CSS value.  ✔\
            \n  a: $foo * $bar;     // 2em  ✔\
            \n  // a: $foo / $bar;     // 2px*px/em isn\'t a valid CSS value.  ✔\
            \n  // a: $foo + $bar;     // Incompatible units: \'em\' and \'px\'.\
            \n  // a: $foo - $bar;     // Incompatible units: \'em\' and \'px\'.\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  foo: 2em;\
        \n  bar: 2em;\
        \n  a: 1;\
        \n  a: 4em;\
        \n  a: 0em;\
        \n  foo: 2px;\
        \n  bar: 2em;\
        \n  foo: 2em;\
        \n  bar: 2px;\
        \n  bar: 2px;\
        \n  bar: 2px;\
        \n  a: 2em;\
        \n  a: 1;\
        \n  a: 1;\
        \n  foo: 2px;\
        \n  a: 2em;\
        \n  foo: 2px;\
        \n  a: 2em;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_784.hrx"
#[test]
fn issue_784() {
    assert_eq!(
        rsass(
            ".foo {\
            \n  @each $item in (a: 1, b: 2, c: 3) {\
            \n    each: $item;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  each: a 1;\
        \n  each: b 2;\
        \n  each: c 3;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_803.hrx"
#[test]
#[ignore] // wrong result
fn issue_803() {
    assert_eq!(
        rsass(
            "\
            \n$query-string: \"(min-width: 0) and (max-width: 599px),  (min-width: 600px) and (max-width: 899px)\";\
            \n@media #{$query-string} {\
            \n  .foo {\
            \n    content: bar;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@media (min-width: 0) and (max-width: 599px), (min-width: 600px) and (max-width: 899px) {\
        \n  .foo {\
        \n    content: bar;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_813.hrx"
#[test]
#[ignore] // wrong result
fn issue_813() {
    assert_eq!(
        rsass(
            "@function foo($one, $two) {\
            \n  @return $one + $two;\
            \n}\
            \n\
            \n$nums: 1px 2px;\
            \n\
            \n.foo {\
            \n  left: foo($nums...);\
            \n  bottom: $nums 3px;\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  left: 3px;\
        \n  bottom: 1px 2px 3px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_815.hrx"
#[test]
fn issue_815() {
    assert_eq!(
        rsass(
            "foo {\
            \n  foo: str-slice(\"bar\", 1, 2);\
            \n  bar: str-slice(\"bar\", 3);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  foo: \"ba\";\
        \n  bar: \"r\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_817.hrx"
#[test]
fn issue_817() {
    assert_eq!(
        rsass(
            "foo {\
            \n  foo: url(\'foo/bar.baz\');\
            \n  foo: url(\"foo/bar.baz\");\
            \n  foo: url(foo/bar.baz);\
            \n  foo: foo(\'foo/bar.baz\', \"bar\", 55);\
            \n  foo: foo(\"foo/bar.baz\", \'bar\', 55);\
            \n  foo: foo(\"foo/bar.baz\", bar, 55); }\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  foo: url(\"foo/bar.baz\");\
        \n  foo: url(\"foo/bar.baz\");\
        \n  foo: url(foo/bar.baz);\
        \n  foo: foo(\"foo/bar.baz\", \"bar\", 55);\
        \n  foo: foo(\"foo/bar.baz\", \"bar\", 55);\
        \n  foo: foo(\"foo/bar.baz\", bar, 55);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_820"
#[test]
fn issue_820() {
    assert_eq!(
        rsass(
            "@charset \"UTF-8\";\
            \n/*!  Force output of above line by adding a unicode character. ♫ */\
            \nhtml, body {\
            \n  height: 100%; }\
            \n"
        )
        .unwrap(),
        "@charset \"UTF-8\";\
        \n/*!  Force output of above line by adding a unicode character. ♫ */\
        \nhtml, body {\
        \n  height: 100%;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_823.hrx"
#[test]
#[ignore] // wrong result
fn issue_823() {
    assert_eq!(
        rsass(
            "%test {\
            \n  > {\
            \n    .red {\
            \n      color: #F00;\
            \n    }\
            \n  }\
            \n}\
            \n\
            \np {\
            \n  @extend %test;\
            \n\
            \n  > {\
            \n    a {\
            \n      @extend %test;\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "p > a > .red, p > .red {\
        \n  color: #F00;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_828.hrx"
#[test]
#[ignore] // wrong result
fn issue_828() {
    assert_eq!(
        rsass(
            "foo {\
            \n  box-shadow: inset -1.5em 0 1.5em -0.75em rgba(0, 0, 0, 0.25);\
            \n  box-shadow: inset -1.5em 0 1.5em - 0.75em rgba(0, 0, 0, 0.25);\
            \n  box-shadow: inset -1.5em 0 1.5em- 0.75em rgba(0, 0, 0, 0.25);\
            \n  box-shadow: inset -1.5em 0 1.5em-0.75em rgba(0, 0, 0, 0.25);\
            \n  box-shadow: inset -1.5em 0 1.5em -.75em rgba(0, 0, 0, .25);\
            \n  box-shadow: inset -1.5em 0 1.5em - .75em rgba(0, 0, 0, .25);\
            \n  box-shadow: inset -1.5em 0 1.5em- .75em rgba(0, 0, 0, .25);\
            \n  box-shadow: inset -1.5em 0 1.5em-.75em rgba(0, 0, 0, .25);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  box-shadow: inset -1.5em 0 1.5em -0.75em rgba(0, 0, 0, 0.25);\
        \n  box-shadow: inset -1.5em 0 0.75em rgba(0, 0, 0, 0.25);\
        \n  box-shadow: inset -1.5em 0 1.5em- 0.75em rgba(0, 0, 0, 0.25);\
        \n  box-shadow: inset -1.5em 0 0.75em rgba(0, 0, 0, 0.25);\
        \n  box-shadow: inset -1.5em 0 1.5em -0.75em rgba(0, 0, 0, 0.25);\
        \n  box-shadow: inset -1.5em 0 0.75em rgba(0, 0, 0, 0.25);\
        \n  box-shadow: inset -1.5em 0 1.5em- 0.75em rgba(0, 0, 0, 0.25);\
        \n  box-shadow: inset -1.5em 0 0.75em rgba(0, 0, 0, 0.25);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_829.hrx"
#[test]
#[ignore] // wrong result
fn issue_829() {
    assert_eq!(
        rsass(
            ".foo {\
            \n    @media (foo: bar), (bar: baz) {\
            \n        foo: bar;\
            \n\
            \n        @media (foo: bar) {\
            \n            bar: baz;\
            \n        }\
            \n\
            \n        .bar {\
            \n            baz: bam;\
            \n        }\
            \n    }\
            \n }\
            \n\
            \n"
        )
        .unwrap(),
        "@media (foo: bar), (bar: baz) {\
        \n  .foo {\
        \n    foo: bar;\
        \n  }\
        \n}\
        \n@media (foo: bar) and (foo: bar), (bar: baz) and (foo: bar) {\
        \n  .foo {\
        \n    bar: baz;\
        \n  }\
        \n}\
        \n@media (foo: bar), (bar: baz) {\
        \n  .foo .bar {\
        \n    baz: bam;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_83.hrx"

// Ignoring "issue_83", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_845.hrx"

// From "sass-spec/spec/libsass-closed-issues/issue_857.hrx"
#[test]
fn issue_857() {
    assert_eq!(
        rsass(
            "$list: \"item-1\" \"item-2\" \"item-3\";\
            \n\
            \n#hello {\
            \n  @if length($list) % 2 == 0 {\
            \n    color: blue;\
            \n  }\
            \n\
            \n  @else {\
            \n    color: red;\
            \n  }\
            \n}"
        )
        .unwrap(),
        "#hello {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_859.hrx"
#[test]
fn issue_859() {
    assert_eq!(
        rsass(
            "@media screen {\
            \n  .two {\
            \n    @at-root .one {\
            \n      background: blue;\
            \n      .three {\
            \n        color: red;\
            \n      }\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@media screen {\
        \n  .one {\
        \n    background: blue;\
        \n  }\
        \n  .one .three {\
        \n    color: red;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_86.hrx"
#[test]
fn issue_86() {
    assert_eq!(
        rsass(
            ".color-functions {\r\
            \n    $color: red;\r\
            \n    hue: hue($color);\r\
            \n    hue-type: type-of(hue($color));\r\
            \n    hue-unit: unit(hue($color));\r\
            \n    hue-comparable: comparable(hue($color), hue($color));\r\
            \n\ttest-1: comparable(lightness(red), 1%);\r\
            \n\ttest-2: comparable(saturation(red), 1%);\r\
            \n}"
        )
        .unwrap(),
        ".color-functions {\
        \n  hue: 0deg;\
        \n  hue-type: number;\
        \n  hue-unit: \"deg\";\
        \n  hue-comparable: true;\
        \n  test-1: true;\
        \n  test-2: true;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_864.hrx"
#[test]
fn issue_864() {
    assert_eq!(
        rsass("div { color: desaturate(#999, 50%); }").unwrap(),
        "div {\
        \n  color: #999999;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_87.hrx"
#[test]
fn issue_87() {
    assert_eq!(
        rsass(
            "$bar: \"bar\";\r\
            \n$foobar: \"foo#{$bar}\";\r\
            \n#{$bar} {\r\
            \n  #{$bar}: #{$bar};\r\
            \n  #{$bar}: $bar;\r\
            \n}\r\
            \n#{$foobar} {\r\
            \n  #{$foobar}: #{$foobar};\r\
            \n  #{$foobar}: $foobar;\r\
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

// From "sass-spec/spec/libsass-closed-issues/issue_870.hrx"
#[test]
fn issue_870() {
    assert_eq!(
        rsass(
            "$quoted-strings-csv: \"alpha\", \"beta\", \'gamma\', \'delta\';\
            \n$quoted-strings-ssv: \"alpha\" \"beta\" \'gamma\' \'delta\';\
            \n\
            \n.csv {\
            \n  output: $quoted-strings-csv;\
            \n  output: #{$quoted-strings-csv};\
            \n  output: \"[#{$quoted-strings-csv}]\";\
            \n  output: \"#{$quoted-strings-csv}\";\
            \n  output: \"[\"#{$quoted-strings-csv}\"]\";\
            \n  output: \'#{$quoted-strings-csv}\';\
            \n  output: \"[\'#{$quoted-strings-csv}\']\";\
            \n}\
            \n\
            \n.ssv {\
            \n  output: $quoted-strings-ssv;\
            \n  output: #{$quoted-strings-ssv};\
            \n  output: \"[#{$quoted-strings-ssv}]\";\
            \n  output: \"#{$quoted-strings-ssv}\";\
            \n  output: \"[\"#{$quoted-strings-ssv}\"]\";\
            \n  output: \'#{$quoted-strings-ssv}\';\
            \n  output: \"[\'#{$quoted-strings-ssv}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".csv {\
        \n  output: \"alpha\", \"beta\", \"gamma\", \"delta\";\
        \n  output: alpha, beta, gamma, delta;\
        \n  output: \"[alpha, beta, gamma, delta]\";\
        \n  output: \"alpha, beta, gamma, delta\";\
        \n  output: \"[\" alpha, beta, gamma, delta \"]\";\
        \n  output: \"alpha, beta, gamma, delta\";\
        \n  output: \"[\'alpha, beta, gamma, delta\']\";\
        \n}\
        \n.ssv {\
        \n  output: \"alpha\" \"beta\" \"gamma\" \"delta\";\
        \n  output: alpha beta gamma delta;\
        \n  output: \"[alpha beta gamma delta]\";\
        \n  output: \"alpha beta gamma delta\";\
        \n  output: \"[\" alpha beta gamma delta \"]\";\
        \n  output: \"alpha beta gamma delta\";\
        \n  output: \"[\'alpha beta gamma delta\']\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_871.hrx"

// Ignoring "issue_871", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_873.hrx"
#[test]
fn issue_873() {
    assert_eq!(
        rsass(
            "$quoted: \"notification\";\
            \n$unquoted: notification;\
            \n\
            \n@function func($var) {\
            \n  @return $var;\
            \n}\
            \n\
            \nfoo {\
            \n  foo: func(notification);\
            \n  foo: #{notification};\
            \n  foo: #{$quoted};\
            \n  foo: $quoted;\
            \n  foo: #{$unquoted};\
            \n  foo: $unquoted;\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  foo: notification;\
        \n  foo: notification;\
        \n  foo: notification;\
        \n  foo: \"notification\";\
        \n  foo: notification;\
        \n  foo: notification;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_877.hrx"
#[test]
fn issue_877() {
    assert_eq!(
        rsass(
            "@function _test1() {\
            \n  @return \'hello\';\
            \n}\
            \n\
            \n@function -test2() {\
            \n  @return \'hello\';\
            \n}\
            \n\
            \n@function test() {\
            \n  @return \'world\';\
            \n}\
            \n\
            \n@mixin _test1() {\
            \n  mixin: true;\
            \n}\
            \n\
            \n@mixin -test2() {\
            \n  mixin: true;\
            \n}\
            \n\
            \n@mixin test() {\
            \n  mixin: true;\
            \n}\
            \n\
            \n$-test1: true;\
            \n$_test2: true;\
            \n$test: true;\
            \n\
            \n.test {\
            \n  function: function-exists(\'_test1\');\
            \n  function: function-exists(\'-test1\');\
            \n  function: function-exists(\'_test2\');\
            \n  function: function-exists(\'-test2\');\
            \n  function: function-exists(\'test1\');\
            \n  function: function-exists(\'test2\');\
            \n  function: function-exists(\'test\');\
            \n  mixin: mixin-exists(\'_test1\');\
            \n  mixin: mixin-exists(\'-test1\');\
            \n  mixin: mixin-exists(\'_test2\');\
            \n  mixin: mixin-exists(\'-test2\');\
            \n  mixin: mixin-exists(\'test1\');\
            \n  mixin: mixin-exists(\'test2\');\
            \n  mixin: mixin-exists(\'test\');\
            \n  variable: variable-exists(\'_test1\');\
            \n  variable: variable-exists(\'-test1\');\
            \n  variable: variable-exists(\'_test2\');\
            \n  variable: variable-exists(\'-test2\');\
            \n  variable: variable-exists(\'test1\');\
            \n  variable: variable-exists(\'test2\');\
            \n  variable: variable-exists(\'test\');\
            \n  global-variable: global-variable-exists(\'_test1\');\
            \n  global-variable: global-variable-exists(\'-test1\');\
            \n  global-variable: global-variable-exists(\'_test2\');\
            \n  global-variable: global-variable-exists(\'-test2\');\
            \n  global-variable: global-variable-exists(\'test1\');\
            \n  global-variable: global-variable-exists(\'test2\');\
            \n  global-variable: global-variable-exists(\'test\');\
            \n}\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  function: true;\
        \n  function: true;\
        \n  function: true;\
        \n  function: true;\
        \n  function: false;\
        \n  function: false;\
        \n  function: true;\
        \n  mixin: true;\
        \n  mixin: true;\
        \n  mixin: true;\
        \n  mixin: true;\
        \n  mixin: false;\
        \n  mixin: false;\
        \n  mixin: true;\
        \n  variable: true;\
        \n  variable: true;\
        \n  variable: true;\
        \n  variable: true;\
        \n  variable: false;\
        \n  variable: false;\
        \n  variable: true;\
        \n  global-variable: true;\
        \n  global-variable: true;\
        \n  global-variable: true;\
        \n  global-variable: true;\
        \n  global-variable: false;\
        \n  global-variable: false;\
        \n  global-variable: true;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_883.hrx"
#[test]
fn issue_883() {
    assert_eq!(
        rsass(
            "div {\
            \n  @foo {\
            \n    font: a;\
            \n  }\
            \n  @bar {\
            \n    color: b;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@foo {\
        \n  div {\
        \n    font: a;\
        \n  }\
        \n}\
        \n@bar {\
        \n  div {\
        \n    color: b;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_884.hrx"
#[test]
fn issue_884() {
    assert_eq!(
        rsass(
            "@function foo() {\
            \n  @return 2;\
            \n}\
            \n\
            \n$foo: false;\
            \n@if foo() % 2 == 0 {\
            \n  $foo: true;\
            \n}\
            \n\
            \na {\
            \n  foo: $foo;\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  foo: true;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_890.hrx"
#[test]
fn issue_890() {
    assert_eq!(
        rsass(
            ".foo {\
            \n  border: {\
            \n    right: 10px solid /*here is a comment*/;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo {\
        \n  border-right: 10px solid;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_893.hrx"
#[test]
#[ignore] // wrong result
fn issue_893() {
    assert_eq!(
        rsass(
            "$gutter: 20px;\
            \n\
            \n.row {\
            \n  margin: 0 -$gutter;\
            \n}"
        )
        .unwrap(),
        ".row {\
        \n  margin: -20px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_894.hrx"
#[test]
fn issue_894() {
    assert_eq!(
        rsass(
            "a {/**/}\
            \nb {content: \'something so I have a non-empty expected output\'}"
        )
        .unwrap(),
        "a {\
        \n  /**/\
        \n}\
        \nb {\
        \n  content: \"something so I have a non-empty expected output\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_91.hrx"
#[test]
fn issue_91() {
    assert_eq!(
        rsass(
            "@mixin simple-media-query($max-width, $min-width) {\r\
            \n      @media only screen and (max-width: $max-width) and (min-width: $min-width) {\r\
            \n        @content;\r\
            \n      }\r\
            \n}\r\
            \n\r\
            \n@mixin test($value) {\r\
            \n    border-color: $value;\r\
            \n}\r\
            \n\r\
            \nbody \r\
            \n{\r\
            \n    @include test(\"#ccc\");\r\
            \n    @include simple-media-query(900px, 400px) {\r\
            \n        border-color: black;\r\
            \n    }\r\
            \n}"
        )
        .unwrap(),
        "body {\
        \n  border-color: \"#ccc\";\
        \n}\
        \n@media only screen and (max-width: 900px) and (min-width: 400px) {\
        \n  body {\
        \n    border-color: black;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_930.hrx"
#[test]
fn issue_930() {
    assert_eq!(
        rsass(
            ".foo {\
            \n  &.bar {\
            \n    color: #F00;\
            \n  }\
            \n}\
            \n\
            \n$class: \'baz\';\
            \n.foo {\
            \n  &.#{$class} {\
            \n    color: #F00;\
            \n  }\
            \n}\
            \n\
            \n$n: 1;\
            \n.foo {\
            \n  &:nth-child(#{$n}) {\
            \n    color: #F00;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        ".foo.bar {\
        \n  color: #F00;\
        \n}\
        \n.foo.baz {\
        \n  color: #F00;\
        \n}\
        \n.foo:nth-child(1) {\
        \n  color: #F00;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_931.hrx"
#[test]
#[ignore] // wrong result
fn issue_931() {
    assert_eq!(
        rsass(
            "@mixin img-opacity($trans) {\
            \n          filter : alpha(opacity=($trans * 100));\
            \n      -ms-filter : \"progid:DXImageTransform.Microsoft.Alpha(Opacity=#{$trans * 100})\";\
            \n    -moz-opacity : $trans;\
            \n  -khtml-opacity : $trans;\
            \n         opacity : $trans;\
            \n}\
            \n\
            \nimg {\
            \n  @include img-opacity(.5);\
            \n}"
        )
        .unwrap(),
        "img {\
        \n  filter: alpha(opacity=50);\
        \n  -ms-filter: \"progid:DXImageTransform.Microsoft.Alpha(Opacity=50)\";\
        \n  -moz-opacity: 0.5;\
        \n  -khtml-opacity: 0.5;\
        \n  opacity: 0.5;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_941.hrx"
#[test]
#[ignore] // wrong result
fn issue_941() {
    assert_eq!(
        rsass(
            ".one, /* 1 */\
            \n.two /* 2 */ { /* 3 */\
            \n\tcolor: #F00; /* 4 */\
            \n}\
            \n"
        )
        .unwrap(),
        ".one,\
        \n.two {\
        \n  /* 3 */\
        \n  color: #F00;\
        \n  /* 4 */\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_942.hrx"
#[test]
#[ignore] // wrong result
fn issue_942() {
    assert_eq!(
        rsass(
            "$v: \".foo \\\
            \n.bar\";\
            \n\
            \n#{$v} {\
            \n\tcolor: #F00;\
            \n}\
            \n\
            \ndiv {\
            \n\tcontent: \"foo\\\
            \nbar\";\
            \n}"
        )
        .unwrap(),
        ".foo .bar {\
        \n  color: #F00;\
        \n}\
        \ndiv {\
        \n  content: \"foobar\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_943.hrx"
#[test]
#[ignore] // wrong result
fn issue_943() {
    assert_eq!(
        rsass(
            "%dog {\
            \n    @media (min-width: 10px) {\
            \n        &:hover {\
            \n            display: none;\
            \n        }\
            \n    }\
            \n}\
            \n\
            \n.puppy {\
            \n    @extend %dog;\
            \n    background-color: red;\
            \n}"
        )
        .unwrap(),
        "@media (min-width: 10px) {\
        \n  .puppy:hover {\
        \n    display: none;\
        \n  }\
        \n}\
        \n.puppy {\
        \n  background-color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_945.hrx"

// Ignoring "issue_945", error tests are not supported yet.

// From "sass-spec/spec/libsass-closed-issues/issue_947.hrx"
#[test]
fn issue_947() {
    assert_eq!(
        rsass(
            "@keyframes test {\
            \n  $var: 10%;\
            \n  #{$var} {\
            \n    color: red;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@keyframes test {\
        \n  10% {\
        \n    color: red;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_948.hrx"
#[test]
fn issue_948() {
    assert_eq!(
        rsass("foo { bar: 10 * 5#{px}; }").unwrap(),
        "foo {\
        \n  bar: 50 px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_950.hrx"
#[test]
#[ignore] // wrong result
fn issue_950() {
    assert_eq!(
        rsass(
            ".selector1{ foo: bar; }\
            \n.selector2{ zapf: dings; }\
            \n\
            \n.selector3{ @extend .selector1, .selector2; }"
        )
        .unwrap(),
        ".selector1, .selector3 {\
        \n  foo: bar;\
        \n}\
        \n.selector2, .selector3 {\
        \n  zapf: dings;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_976.hrx"
#[test]
fn issue_976() {
    assert_eq!(
        rsass(
            ".debug {\
            \n  @debug-this {\
            \n    foo: bar;\
            \n  }\
            \n}"
        )
        .unwrap(),
        "@debug-this {\
        \n  .debug {\
        \n    foo: bar;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_978.hrx"
#[test]
fn issue_978() {
    assert_eq!(
        rsass(
            ".foo {\
            \n  [baz=\"#{&}\"] {\
            \n    foo: bar;\
            \n  }\
            \n}"
        )
        .unwrap(),
        ".foo [baz=\".foo\"] {\
        \n  foo: bar;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_980.hrx"
#[test]
fn issue_980() {
    assert_eq!(
        rsass(
            "@function foo($value, $default: 13, $args...) {\
            \n  $res: $value + $default;\
            \n  @if length($args) != 0 {\
            \n    $res: $res + nth($args, 1);\
            \n  }\
            \n  @return $res;\
            \n}\
            \n\
            \n.test {\
            \n  value: foo(3); // expected: 16\
            \n  value: foo(3, 4); // expected: 7\
            \n  value: foo(3, 4, 5, 6); // expected: 12\
            \n}\
            \n"
        )
        .unwrap(),
        ".test {\
        \n  value: 16;\
        \n  value: 7;\
        \n  value: 12;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_988.hrx"
#[test]
fn issue_988() {
    assert_eq!(
        rsass(
            "@function str-replace($string, $search, $replace: \'\') {\
            \n  $index: str-index($string, $search);\
            \n  @if $index {\
            \n    @return str-slice($string, 1, $index - 1) + $replace +\
            \n      str-replace(str-slice($string, $index + str-length($search)), $search, $replace);\
            \n  }\
            \n  @return $string;\
            \n}\
            \n\
            \n$string: \'Foo Bar Baz Qux\';\
            \n.foo {\
            \n  content: str-replace($string, \' \', \'-\');\
            \n}"
        )
        .unwrap(),
        ".foo {\
        \n  content: \"Foo-Bar-Baz-Qux\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/libsass-closed-issues/issue_992.hrx"
#[test]
fn issue_992() {
    assert_eq!(
        rsass(
            "$color: \'red\';\
            \n\
            \n.-text-#{$color}- {\
            \n  color: $color;\
            \n}"
        )
        .unwrap(),
        ".-text-red- {\
        \n  color: \"red\";\
        \n}\
        \n"
    );
}
