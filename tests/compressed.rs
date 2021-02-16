//! These are from the `output_styles/compressed/basic` directory in the
//! sass specification.
use rsass::compile_scss;
use rsass::output::{Format, Style};

#[test]
fn t01_simple_css() {
    check(
        b"a {\n  \
            color: blue;\n\
            }",
        "a{color:blue}\n",
    )
}

#[test]
fn t02_simple_nesting() {
    check(
        b"div {\n  img {\n    border: 0px;\n  }\n}",
        "div img{border:0px}\n",
    )
}

#[test]
fn t03_simple_variable() {
    check(
        b"$color: red;\n\na {\n  color: $color;\n}",
        "a{color:red}\n",
    )
}

#[test]
fn t04_basic_variables() {
    check(
        b"$color: \"black\";\n$color: red;\n$background: \"blue\";\n\n\
            a {\n  color: $color;\n  background: $background;\n}\n\n\
            $y: before;\n\n\
            $x: 1 2 $y;\n\n\
            foo {\n  a: $x;\n}\n\n\
            $y: after;\n\n\
            foo {\n  a: $x;\n}",
        "a{color:red;background:\"blue\"}foo{a:1 2 before}\
         foo{a:1 2 before}\n",
    )
}

#[test]
fn t05_empty_levels() {
    check(
        b"div {\n  \
            span {\n    color: red;\n    background: blue;\n  }\n}\n\n\
            div {\n  color: gray;\n\
            empty {\n    \
            span {\n      color: red;\n      background: blue;\n    }\n  \
            }\n}\n\n\
            empty1 {\n  empty2 {\n    \
            div {\n      blah: blah;\n    }\n  }\n}\n\n\
            empty1 {\n  empty2 {\n    div {\n      bloo: blee;\n      \
            empty3 {\n        \
            span {\n          blah: blah;\n          blah: blah;\n        \
            }\n      }\n    }\n  }\n}\n",
        "div span{color:red;background:blue}div{color:gray}\
         div empty span{color:red;background:blue}\
         empty1 empty2 div{blah:blah}empty1 empty2 div{bloo:blee}\
         empty1 empty2 div empty3 span{blah:blah;blah:blah}\n",
    )
}

#[test]
fn t06_nesting_and_comments() {
    // No comments preserved in compressed output!
    check(
        b"$blah: bloo blee;\n$blip: \"a 'red' and \\\"blue\\\" value\";\n\n\
            /* top level comment -- should be preserved */\n\
            div {\n  /* another comment that should be preserved */\n  \
            color: red;\n  background: blue;\n  $blux: hux; // gone!\n  \
            span {\n    font-weight: bold;\n    \
            a {\n      \
            text-decoration: none; /* where will this comment go? */\n      \
            color: green;\n      \
            /* what about this comment? */ border: 1px $blah red;\n    \
            }\n    \
            /* yet another comment that should be preserved */\n    \
            display: inline-block;\n  }  // gone!\n  \
            /* the next selector should be indented two spaces */\n  \
            empty {\n    \
            not_empty {\n      blah: blah; // gone!\n      bloo: bloo;\n    \
            }\n  }\n  \
            p {\n    padding: 10px 8%;\n    -webkit-box-sizing: $blux;\n  }\n  \
            margin: 10px 5px;\n  h1 {\n    color: $blip;\n  }\n}\n\
            /* last comment, top level again --\n   \
            compare the indentation! */\n\n\
            div {\n\n\
            f: g;\n  \
            empty {\n    span {\n      a: b;\n    }\n  }\n  \
            empty_with_comment {\n    /* hey now */\n    \
            span {\n      c: d;\n    }\n  }\n}",
        "div{color:red;background:blue;margin:10px 5px}\
         div span{font-weight:bold;display:inline-block}\
         div span a{text-decoration:none;color:green;\
         border:1px bloo blee red}\
         div empty not_empty{blah:blah;bloo:bloo}\
         div p{padding:10px 8%;-webkit-box-sizing:hux}\
         div h1{color:\"a 'red' and \\\"blue\\\" value\"}\
         div{f:g}div empty span{a:b}div empty_with_comment span{c:d}\n",
    )
}

#[test]
fn t07_nested_simple_selector_groups() {
    check(
        b"a, b {\n  color: red;\n  background: blue;\n}\n\n\
            c, d {\n  color: gray;\n  \
            e, f {\n    background: blue;\n    padding: 10px 5px;\n  }\n  \
            g, h {\n    blah: blah;\n    bloo: bloo;\n  }\n  \
            i, j {\n    foo: goo;\n    k, l {\n      \
            m, n, o {\n        wow: we are far inside;\n        \
            but: it still works;\n      }\n      \
            hoo: boo;\n    }\n  }\n}",
        "a,b{color:red;background:blue}c,d{color:gray}\
         c e,c f,d e,d f{background:blue;padding:10px 5px}\
         c g,c h,d g,d h{blah:blah;bloo:bloo}c i,c j,d i,d j{foo:goo}\
         c i k,c i l,c j k,c j l,d i k,d i l,d j k,d j l{hoo:boo}\
         c i k m,c i k n,c i k o,c i l m,c i l n,c i l o,c j k m,c j k n,\
         c j k o,c j l m,c j l n,c j l o,d i k m,d i k n,d i k o,d i l m,\
         d i l n,d i l o,d j k m,d j k n,d j k o,d j l m,d j l n,d j l o\
         {wow:we are far inside;but:it still works}\n",
    )
}

#[test]
fn t08_selector_combinators() {
    check(
        b"a   +   b  >  c {\n  \
            d e {\n    color: blue;\n    background: white;\n  }\n  \
            color: red;\n  background: gray;\n}",
        "a+b>c{color:red;background:gray}\
         a+b>c d e{color:blue;background:white}\n",
    )
}

#[test]
fn t10_classes_and_ids() {
    check(
        b"a + b, .class {\n  blah: blah;\n  bleh: bleh;\n  \
            d #id, f ~ g.other + h, > i#grar {\n    bloo: bloo;\n    \
            blee: blee;\n  }\n}",
        "a+b,.class{blah:blah;bleh:bleh}\
         a+b d #id,a+b f ~ g.other+h,a+b>i#grar,.class d #id,\
         .class f ~ g.other+h,.class>i#grar{bloo:bloo;blee:blee}\n",
    )
}

#[test]
fn t15_arithmetic_and_lists() {
    check(
        b"$stuff: 1 2 3;\n\n\
            $three: 3;\n\n\
            div {\n  a: 1 + 2;\n  b: 3 + 3/4;\n  c: 1/2 + 1/2;\n  \
            /* shouldn't eval the following \"300\" */\n  d: 300;\n  \
            /* increasingly jacked-up edge cases that combine arithmetic \
            with lists */\n  e: 1 + (5/10 2 3);\n  \
            f: 1 + ((2+(3 4) 5) 6);\n  g: 1 + ((1+(14/7 8) 9) 6);\n  \
            /* shouldn't perform the following division */\n  \
            h: 15 / 3 / 5;\n  \
            /* should perform the following division now */\n  \
            i: (15 / 3 / 5);\n  /* this too */\n  j: (15 / 3) / 5;\n  \
            /* and this */\n  k: 15 / $three;\n  l: 15 / 5 / $three;\n  \
            m: 1/2, $stuff url(\"www.foo.com/blah.png\") blah blah;\n  \
            n: 1 2 3, $stuff 4 5 (6, 7 8 9);\n  o: 3px + 3px + 3px;\n  \
            p: 4 + 1px;\n  q: (20pt / 10pt);\n  r: 16em * 4;\n  \
            s: (5em / 2);\n  t: 1 + (2 + (3/4 + (4/5 6/7)));\n}",
        "div{a:3;b:3.75;c:1;d:300;e:15/10 2 3;f:123 4 5 6;g:1114/7 8 9 6;\
         h:15/3/5;i:1;j:1;k:5;l:1;\
         m:1/2,1 2 3 url(\"www.foo.com/blah.png\") blah blah;\
         n:1 2 3,1 2 3 4 5 6,7 8 9;o:9px;p:5px;q:2;r:64em;s:2.5em;\
         t:12.754/5 6/7}\n",
    )
}

#[test]
fn t19_full_mixin_craziness() {
    check(
        b"$x: global-x;\n$y: global-y;\n$z: global-z;\n\n\
            @mixin foo($x, $y) {\n  /* begin foo */\n  \
            margin: $x $y;\n  blip {\n    hey: now;\n  }\n  \
            /* end foo */\n}\n\n\
            @mixin foogoo($x, $y, $z) {\n  margin: $x $y $z;\n}\n\n\
            @mixin hux($y) {\n  /* begin hux */\n  color: $y;\n  \
            @include foo(called-from-hux, $y: $y);\n  /* end hux */\n}\n\n\
            div {\n  @include foo(1, 2);\n  @include foo(1, 3);\n  \
            @include foogoo(1, 2, $z: zee);\n  \
            @include foogoo(1, $y /* blah */ : kwd-y, $z: kwd-z);\n}\n\n\
            div {\n  @include hux($y: $y);\n}\n\n\
            $y: different-global-y;\n\n\
            div {\n  @include hux(calling-hux-again);\n}\n\n\
            @mixin bung() {\n  blah: original-bung;\n}\n\n\
            div {\n  @include bung();\n}\n\n\
            @mixin bung() {\n  blah: redefined-bung;\n}\n\n\
            div {\n  @include bung();\n}\n\n\
            div {\n  /* calls to nullary mixins may omit the empty argument \
            list */\n  @include bung;\n}\n\n\
            div {\n  @include foo($x: kwdarg1, $y: kwdarg2);\n}\n\n\
            @mixin ruleset() {\n  hoo {\n    color: boo;\n  }\n}\n\n\
            @include ruleset();\n\n\
            $da: default argument;\n\n\
            @mixin default_args($x, $y: $da) {\n  blah: $x $y;\n}\n\
            $da: some other default;\n\n\
            div {\n  @include default_args(boogoo);\n}\n\n\
            @mixin original() {\n  value: original;\n}\n\n\
            div {\n  @include original();\n}\n\n\
            @mixin original() {\n  value: no longer original;\n}\n\n\
            div {\n  @include original();\n}\n\n\
            @mixin set-x($x) {\n  $x: changed local x;\n  arg: $x;\n  \
            $y: changed global y !global;\n  blarg: $y;\n}\n\n\
            div {\n  @include set-x(blah);\n  a: $x;\n  b: $y;\n}\n",
        "div{margin:1 2;margin:1 3;margin:1 2 zee;margin:1 kwd-y kwd-z}\
         div blip{hey:now}div blip{hey:now}div{color:global-y;\
         margin:called-from-hux global-y}div blip{hey:now}\
         div{color:calling-hux-again;margin:called-from-hux \
         calling-hux-again}div blip{hey:now}div{blah:original-bung}\
         div{blah:redefined-bung}div{blah:redefined-bung}\
         div{margin:kwdarg1 kwdarg2}div blip{hey:now}hoo{color:boo}\
         div{blah:boogoo some other default}div{value:original}\
         div{value:no longer original}\
         div{arg:changed local x;blarg:changed global y;a:global-x;\
         b:changed global y}\n",
    )
}

#[test]
fn t22_colors_with_alpha() {
    // Note: Expected output differs from libsass in that rass skipps initial zero in alpha, e.g. `rgba(1,22,3,.5)` rather than `rgba(1,22,3,0.5)`
    check(
        b"$x: rgb(0, 255, 255);\n\n\
            div {\n  color: rgb(255, $blue: 0, $green: 255);\n  \
            background: rgb(123, 45, 6);\n  flah: rgba(0, 0, 0, 1) + #111;\n  \
            grah: rgba(#f0e, $alpha: .5);\n  blah: rgba(1,2,3,.6);\n  \n  \
            floo: $x;\n  bloo: rgba($x, 0.7);\n  groo: $x;\n\n\
            $x: rgb(123, 45, 6);\n  \n  \
            hoo: red($x);\n  moo: green($x);\n  poo: blue($x);\n  \n  \
            goo: mix(rgba(255, 0, 0, 0.5), #00f);\n  \n  \
            boo: invert(#123456);\n}\n",
        "div{color:#ff0;background:#7b2d06;flah:#111;\
           grah:rgba(255,0,238,.5);blah:rgba(1,2,3,.6);floo:aqua;\
           bloo:rgba(0,255,255,.7);groo:aqua;hoo:123;moo:45;poo:6;\
           goo:rgba(64,0,191,.75);boo:#edcba9}
",
    )
}

#[test]
fn t27_media_queries() {
    check(
        b"a b c {\n  blee: blee;\n  \
            d e f {\n    blah: blah;\n    bloo: bloo;\n  }\n  \
            g h, i j {\n    \
            @media print and (foo: 1 2 3), (bar: 3px hux(muz)), \
            not screen {\n      hey: ho;\n      \
            k l m {\n        hee: fee;\n      }\n    }\n  }\n\
            blah: blah;\n}\n",
        "a b c{blee:blee;blah:blah}a b c d e f{blah:blah;bloo:bloo}\
         @media print and (foo: 1 2 3),(bar: 3px hux(muz)),not screen{\
         a b c g h,a b c i j{hey:ho}a b c g h k l m,a b c i j k l m{hee:fee}\
         }\n",
    )
}

#[test]
fn t49_interpolants_in_css_imports() {
    check(
        b"$google-protocol: \"http\"; // choose http or https\n\
            $google-webfont: \"Open+Sans:400italic,700italic,400,700|Oswald\"\
            ; // pull string after ?family= from step 3\n\n\
            @import url(\"#{$google-protocol}://fonts.googleapis.com/css?\
            family=#{$google-webfont}\");",
        "@import url(\"http://fonts.googleapis.com/css?family=\
           Open+Sans:400italic,700italic,400,700|Oswald\")
",
    )
}

#[test]
fn t50_wrapped_pseudo_selectors() {
    check(
        b"div {\n  \
            :-moz-any(ol p.blah, ul, menu, dir) \
            :-moz-any(ol span + h1, ul, menu, dir) ul {\n    \
            list-style-type: square;\n  }\n  \
            :-moz-any(ol span + h1, ul, menu, dir) ul {\n    \
            list-style-type: square;\n  }\n  \
            :foo(p div) {\n    hi: hi;\n  }\n  \
            :foo(ol) {\n    hi: hi;\n  }\n}\n",
        "div :-moz-any(ol p.blah,ul,menu,dir) \
         :-moz-any(ol span+h1,ul,menu,dir) ul{list-style-type:square}\
         div :-moz-any(ol span+h1,ul,menu,dir) ul{list-style-type:square}\
         div :foo(p div){hi:hi}div :foo(ol){hi:hi}\n",
    )
}

fn check(input: &[u8], expected: &str) {
    let format = Format {
        style: Style::Compressed,
        precision: 5,
    };
    assert_eq!(
        String::from_utf8(compile_scss(input, format).unwrap()).unwrap(),
        expected
    );
}
