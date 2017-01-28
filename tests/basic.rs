//! These are from the "basic" directory in the sass specification.
//! See https://github.com/sass/sass-spec for source material.
//! I add one a test function for one specification at a time and then
//! try to implement that functionality without breaking those already
//! added.
extern crate rsass;
use rsass::{OutputStyle, compile_scss, compile_scss_file};

#[test]
fn t00_empty() {
    check(b"\n", "")
}

#[test]
fn txx_empty_rule() {
    check(b"foo{}", "")
}

#[test]
fn t01_simple_css() {
    check(b"a {\n  \
            color: blue;\n\
            }",
          "a {\n  \
            color: blue;\n\
            }\n")
}

#[test]
fn t02_simple_nesting() {
    check(b"div {\n  \
            color: black;\n\
            img {\n    \
            border: 0px;\n  \
            }\n\
            } // gone!",
          "div {\n  \
            color: black;\n\
            }\n\
            div img {\n  \
            border: 0px;\n\
            }\n")
}

#[test]
fn t03_simple_variable() {
    check(b"$color: red;\n\
            \n\
            a {\n  \
            color: $color;\n\
            }",
          "a {\n  \
            color: red;\n\
            }\n")
}

#[test]
fn t04_basic_variables() {
    check(b"$color: \"black\";\n\
            $color: red;\n\
            $background: \"blue\";\n\
            \n\
            a {\n  \
            color: $color;\n  \
            background: $background;\n\
            }\n\
            \n\
            $y: before;\n\
            \n\
            $x: 1 2 $y;\n\
            \n\
            foo {\n  \
            a: $x;\n\
            }\n\
            \n\
            $y: after;\n\
            \n\
            foo {\n  \
            a: $x;\n\
            }",
          "a {\n  \
            color: red;\n  \
            background: \"blue\";\n\
            }\n\
            \n\
            foo {\n  \
            a: 1 2 before;\n\
            }\n\
            \n\
            foo {\n  \
            a: 1 2 before;\n\
            }\n\
            ")
}

#[test]
fn t05_empty_levels() {
    check(b"div {\n  span {\n    color: red;\n    background: blue;\n  }\n}\n\
            \n\
            div {\n  color: gray;\n  empty {\n    \
            span {\n      color: red;\n      background: blue;\n    }\n  \
            }\n}\n\
            \n\
            empty1 {\n  empty2 {\n    div {\n      blah: blah;\n    }\n  }\n}\n\
            \n\
            empty1 {\n  empty2 {\n    div {\n      \
            bloo: blee;\n      empty3 {\n        \
            span {\n          blah: blah;\n          blah: blah;\n        \
            }\n      }\n    }\n  }\n}\n",
          "div span {\n  color: red;\n  background: blue;\n}\n\
            \n\
            div {\n  color: gray;\n}\n\
            div empty span {\n  color: red;\n  background: blue;\n}\n\
            \n\
            empty1 empty2 div {\n  blah: blah;\n}\n\
            \n\
            empty1 empty2 div {\n  bloo: blee;\n}\n\
            empty1 empty2 div empty3 span {\n  blah: blah;\n  blah: blah;\n}\n")
}

#[test]
fn t06b_simpler_comments() {
    check(b"/* top level comment\n   \
            should be preserved */\n\
            div {\n  \
            /* another comment that should be preserved */\n  \
            color: red;\n  \
            /* Yet another preserved comment. */\n\
            }\n\
            /* Final preserved comment */\n",
          "/* top level comment\n   \
            should be preserved */\n\
            div {\n  \
            /* another comment that should be preserved */\n  \
            color: red;\n  \
            /* Yet another preserved comment. */\n\
            }\n\n\
            /* Final preserved comment */\n")
}

#[test]
fn t06c_simple_line_comment() {
    check(b"span {\n  \
            color: red; // line comment goes away\n\
            }\n",
          "span {\n  \
            color: red;\n\
            }\n")
}

#[test]
fn t06d_simple_local_variable() {
    check(b"$red: #ff0000;\n\
            span {\n  \
            $red: #ff3040;
            color: $red; // local value\n\
            }\n\
            div {\n\
            color: $red; // global value\n\
            }\n",
          "span {\n  \
            color: #ff3040;\n\
            }\n\n\
            div {\n  \
            color: #ff0000;\n\
            }\n")
}

#[test]
fn t06_nesting_and_comments() {
    check(b"$blah: bloo blee;\n\
            $blip: \"a 'red' and \\\"blue\\\" value\";\n\
            /* top level comment -- should be preserved */\n\
            div {\n  /* another comment that should be preserved */\n  \
            color: red;\n  background: blue;\n  $blux: hux; // gone!\n  \
            span {\n    font-weight: bold;\n    \
            a {\n      text-decoration: none; \
            /* where will this comment go? */\n      color: green;\n      \
            /* what about this comment? */ border: 1px $blah red;\n    }\n    \
            /* yet another comment that should be preserved */\n    \
            display: inline-block;\n  }  // gone!\n  \
            /* the next selector should be indented two spaces */\n  \
            empty {\n    \
            not_empty {\n      blah: blah; // gone!\n      bloo: bloo;\n    \
            }\n  }\n  \
            p {\n    padding: 10px 8%;\n    -webkit-box-sizing: $blux;\n  }\n  \
            margin: 10px 5px;  \
            h1 {\n    color: $blip;\n  }\n}\n\
            /* last comment, top level again --\n   \
            compare the indentation! */\n\n\
            div {\n  f: g;\n  empty {\n    span {\n      a: b;\n    }\n  }\n  \
            empty_with_comment {\n    /* hey now */\n    \
            span {\n      c: d;\n    }\n  }\n}",
          "/* top level comment -- should be preserved */\n\
           div {\n  /* another comment that should be preserved */\n  \
           color: red;\n  background: blue;\n  \
           /* the next selector should be indented two spaces */\n  \
           margin: 10px 5px;\n}\n\
           div span {\n  font-weight: bold;\n  \
           /* yet another comment that should be preserved */\n  \
           display: inline-block;\n}\n\
           div span a {\n  text-decoration: none;\n  \
           /* where will this comment go? */\n  color: green;\n  \
           /* what about this comment? */\n  border: 1px bloo blee red;\n}\n\
           div empty not_empty {\n  blah: blah;\n  bloo: bloo;\n}\n\
           div p {\n  padding: 10px 8%;\n  -webkit-box-sizing: hux;\n}\n\
           div h1 {\n  color: \"a 'red' and \\\"blue\\\" value\";\n}\n\n\
           /* last comment, top level again --\n   \
           compare the indentation! */\n\
           div {\n  f: g;\n}\n\
           div empty span {\n  a: b;\n}\n\
           div empty_with_comment {\n  /* hey now */\n}\n\
           div empty_with_comment span {\n  c: d;\n}\n")
}

#[test]
fn t07_nested_simple_selector_groups() {
    check(b"a, b {\n  color: red;\n  background: blue;\n}\n\n\
            c, d {\n  \
            color: gray;\n  \
            e, f {\n    background: blue;\n    padding: 10px 5px;\n  }\n  \
            g, h {\n    blah: blah;\n    bloo: bloo;\n  }\n  \
            i, j {\n    \
            foo: goo;\n    \
            k, l {\n      \
            m, n, o {\n        \
            wow: we are far inside;\n        but: it still works;\n      \
            }\n      \
            hoo: boo;\n    }\n  }\n}",
          "a, b {\n  color: red;\n  background: blue;\n}\n\n\
            c, d {\n  color: gray;\n}\n\
            c e, c f, d e, d f {\n  \
            background: blue;\n  padding: 10px 5px;\n}\n\
            c g, c h, d g, d h {\n  blah: blah;\n  bloo: bloo;\n}\n\
            c i, c j, d i, d j {\n  foo: goo;\n}\n\
            c i k, c i l, c j k, c j l, d i k, d i l, d j k, d j l {\n  \
            hoo: boo;\n}\n\
            c i k m, c i k n, c i k o, c i l m, c i l n, c i l o, c j k m, \
            c j k n, c j k o, c j l m, c j l n, c j l o, d i k m, d i k n, \
            d i k o, d i l m, d i l n, d i l o, d j k m, d j k n, d j k o, \
            d j l m, d j l n, d j l o {\n  \
            wow: we are far inside;\n  but: it still works;\n}\n")
}

#[test]
fn t08_selector_combinators() {
    check(b"a   +   b  >  c {\n  \
            d e {\n    color: blue;\n    background: white;\n  }\n  \
            color: red;\n  background: gray;\n}",
          "a + b > c {\n  color: red;\n  background: gray;\n}\n\
            a + b > c d e {\n  color: blue;\n  background: white;\n}\n")
}

#[test]
fn t09_selector_groups_and_combinators() {
    check(b"a + b, c {\n  blah: blah;\n  bleh: bleh;\n  \
            d e, f ~ g + h, > i {\n    bloo: bloo;\n    blee: blee;\n  }\n}",
          "a + b, c {\n  blah: blah;\n  bleh: bleh;\n}\n\
            a + b d e, a + b f ~ g + h, a + b > i, c d e, c f ~ g + h, c > i \
            {\n  bloo: bloo;\n  blee: blee;\n}\n")
}

#[test]
fn t10_classes_and_ids() {
    check(b"a + b, .class {\n  blah: blah;\n  bleh: bleh;\n  \
            d #id, f ~ g.other + h, > i#grar \
            {\n    bloo: bloo;\n    blee: blee;\n  }\n}",
          "a + b, .class {\n  blah: blah;\n  bleh: bleh;\n}\n\
            a + b d #id, a + b f ~ g.other + h, a + b > i#grar, \
            .class d #id, .class f ~ g.other + h, .class > i#grar \
            {\n  bloo: bloo;\n  blee: blee;\n}\n")
}

#[test]
fn t11_attribute_selectors() {
    check(b"[hey  =  'ho'], a > b {\n  blah: blah;\n  \
            c, [hoo *=    \"ha\" ] {\n    bloo: bloo;\n  }\n}",
          "[hey='ho'], a > b {\n  blah: blah;\n}\n\
            [hey='ho'] c, [hey='ho'] [hoo*=\"ha\"], a > b c, \
            a > b [hoo*=\"ha\"] {\n  bloo: bloo;\n}\n")
}

// Note: There does not seem to exist a test 12 in the spec.

#[test]
fn t13_back_references() {
    check(b"hey, ho {\n  \
            & > boo, foo &.goo {\n    bloo: bloo;\n  }\n  \
            blah: blah;\n\
            }",
          "hey, ho {\n  blah: blah;\n}\n\
            hey > boo, foo hey.goo, ho > boo, foo ho.goo {\n  bloo: bloo;\n}\n")
}

#[test]
fn t14_imports() {
    assert_eq!(compile_scss_file("tests/basic/14_imports/input.scss".as_ref(),
                                 OutputStyle::Normal)
                   .and_then(|s| {
                       String::from_utf8(s)
                           .map_err(|e| format!("Non-utf8 output: {}", e))
                   }),
               Ok("div span {\n  moo: goo;\n}\n\n\
                   foo {\n  blah: blah;\n}\n\
                   foo goo {\n  blee: blee;\n  hello: world;\n}\n\
                   foo goo hoo {\n  mux: scooba-dee-doo;\n  \
                   flux: gooboo boo;\n}\n\
                   foo goo hoo d {\n  inside: d now;\n}\n\
                   foo blux {\n  hey: another thing;\n  \
                   ho: will this work;\n}\n"
                   .into()))
}

#[test]
fn t15_arithmetic_and_lists_abcd() {
    check(b"div {\n  a: 1 + 2;\n  b: 3 + 3/4;\n  c: 1/2 + 1/2;\n  \
            /* shouldn't eval the following \"300\" */\n  \
            d: 300;\n}",
          "div {\n  a: 3;\n  b: 3.75;\n  c: 1;\n  \
            /* shouldn't eval the following \"300\" */\n  \
            d: 300;\n}\n")
}

#[test]
fn t15_arithmetic_and_lists_efg() {
    check(b"div {\n  e: 1 + (5/10 2 3);\n  f: 1 + ((2+(3 4) 5) 6);\n  \
            g: 1 + ((1+(14/7 8) 9) 6);\n}",
          "div {\n  e: 15/10 2 3;\n  f: 123 4 5 6;\n  g: 1114/7 8 9 6;\n}\n")
}
#[test]
fn t15_arithmetic_and_lists_hijklmt_div_or_not() {
    check(b"$stuff: 1 2 3;\n\
            $three: 3;\n\
            div {\n  \
            /* shouldn't perform the following division */\n  \
            h: 15 / 3 / 5;\n  \
            /* should perform the following division now */\n  \
            i: (15 / 3 / 5);\n  /* this too */\n  j: (15 / 3) / 5;\n  \
            /* and this */\n  k: 15 / $three;\n  l: 15 / 5 / $three;\n  \
            m: 1/2, $stuff url(\"www.foo.com/blah.png\") blah blah;\n  \
            t: 1 + (2 + (3/4 + (4/5 6/7)));\n}",
          "div {\n  /* shouldn't perform the following division */\n  \
            h: 15 / 3 / 5;\n  \
            /* should perform the following division now */\n  \
            i: 1;\n  /* this too */\n  j: 1;\n  /* and this */\n  k: 5;\n  \
            l: 1;\n  \
            m: 1/2, 1 2 3 url(\"www.foo.com/blah.png\") blah blah;\n  \
            t: 120.754/5 6/7;\n}\n")
}

#[test]
fn t15_arithmetic_and_lists_nopqrs() {
    check(b"$stuff: 1 2 3;\n\
            div {\n  n: 1 2 3, $stuff 4 5 (6, 7 8 9);\n  \
            o: 3px + 3px + 3px;\n  p: 4 + 1px;\n  q: (20pt / 10pt);\n  \
            r: 16em * 4;\n  s: (5em / 2);\n}",
          "div {\n  n: 1 2 3, 1 2 3 4 5 6, 7 8 9;\n  \
            o: 9px;\n  p: 5px;\n  q: 2;\n  r: 64em;\n  s: 2.5em;\n}\n")
}

#[test]
fn t15_arithmetic_and_lists() {
    check(b"$stuff: 1 2 3;\n\n\
            $three: 3;\n\n\
            div {\n  \
            a: 1 + 2;\n  b: 3 + 3/4;\n  c: 1/2 + 1/2;\n  \
            /* shouldn't eval the following \"300\" */\n  d: 300;\n\
            /* increasingly jacked-up edge cases that combine \
            arithmetic with lists */\n  \
            e: 1 + (5/10 2 3);\n  f: 1 + ((2+(3 4) 5) 6);\n  \
            g: 1 + ((1+(14/7 8) 9) 6);\n  \
            /* shouldn't perform the following division */\n  \
            h: 15 / 3 / 5;\n  \
            /* should perform the following division now */\n  \
            i: (15 / 3 / 5);\n  \
            /* this too */\n  j: (15 / 3) / 5;\n  \
            /* and this */\n  k: 15 / $three;\n  l: 15 / 5 / $three;\n  \
            m: 1/2, $stuff url(\"www.foo.com/blah.png\") blah blah;\n  \
            n: 1 2 3, $stuff 4 5 (6, 7 8 9);\n  \
            o: 3px + 3px + 3px;\n  p: 4 + 1px;\n  q: (20pt / 10pt);\n  \
            r: 16em * 4;\n  s: (5em / 2);\n  \
            t: 1 + (2 + (3/4 + (4/5 6/7)));\n}",
          "div {\n  a: 3;\n  b: 3.75;\n  c: 1;\n  \
            /* shouldn't eval the following \"300\" */\n  d: 300;\n  \
            /* increasingly jacked-up edge cases that combine \
            arithmetic with lists */\n  \
            e: 15/10 2 3;\n  f: 123 4 5 6;\n  g: 1114/7 8 9 6;\n  \
            /* shouldn't perform the following division */\n  \
            h: 15 / 3 / 5;\n  \
            /* should perform the following division now */\n  i: 1;\n  \
            /* this too */\n  j: 1;\n  /* and this */\n  k: 5;\n  l: 1;\n  \
            m: 1/2, 1 2 3 url(\"www.foo.com/blah.png\") blah blah;\n  \
            n: 1 2 3, 1 2 3 4 5 6, 7 8 9;\n  \
            o: 9px;\n  p: 5px;\n  q: 2;\n  r: 64em;\n  s: 2.5em;\n  \
            t: 120.754/5 6/7;\n}\n")
}

#[test]
fn t16_hex_arithmetic() {
    check(b"div {\n  p01: #AbC;\n  p02: #AAbbCC;\n  p03: #AbC + hello;\n  \
            p04: #AbC + 1; // add 1 to each triplet\n  \
            p05: #AbC + #001; // triplet-wise addition\n  \
            p06: #0000ff + 1; // add 1 to each triplet; \
            ignore overflow because it doesn't correspond to a color name\n  \
            p07: #0000ff + #000001; // convert overflow to name of color \
            (blue)\n  \
            p08: #00ffff + #000101; // aqua\n  \
            p09: #000000;\n  p10: #000000 - 1; // black\n  \
            p11: #000000 - #000001; // black\n  \
            p12: #ffff00 + #010100; // yellow\n  \
            p13: (#101010 / 7);\n  p14: #000 + 0;\n  p15a: 10 - #a2B;\n  \
            p15b: 10 - #aa22BB;\n  p16: #000 - #001;\n  p17: #f0F + #101;\n  \
            p18: 10 #a2B + 1;\n  p19a: (10 / #a2B);\n  \
            p19b: (10 / #aa22BB);\n  p20: rgb(10,10,10) + #010001;\n  \
            p21: #010000 + rgb(255, 255, 255);\n}",
          "div {\n  p01: #AbC;\n  p02: #AAbbCC;\n  p03: #AbChello;\n  \
            p04: #abbccd;\n  p05: #aabbdd;\n  p06: #0101ff;\n  p07: blue;\n  \
            p08: cyan;\n  p09: #000000;\n  p10: black;\n  p11: black;\n  \
            p12: yellow;\n  p13: #020202;\n  p14: black;\n  p15a: 10-#a2B;\n  \
            p15b: 10-#aa22BB;\n  p16: black;\n  p17: magenta;\n  \
            p18: 10 #ab23bc;\n  p19a: 10/#a2B;\n  p19b: 10/#aa22BB;\n  \
            p20: #0b0a0b;\n  p21: white;\n}\n")
}

#[test]
fn t17_basic_mixins() {
    check(b"@mixin foo($x, $y) {\n  hugabug: $y $x;\n}\n\n\
            @mixin bar($a, $b: flug) {\n  flugablug: $a $b glug;\n}\n\n\
            @mixin hux() {\n  \
            no: parameters here;\n  \
            div, span {\n    \
            some: nested stuff;\n    \
            foo, bar {\n      more: stuff so forth;\n      blah: blah;\n    \
            }\n  }\n  /* end of hux */\n}\n\n\
            a {\n  \
            hey: ho;\n  @include foo(second, third);\n  \
            @include foo($y: kwd-y, $x: kwd-x);\n  goo: boo hoo;\n  \
            @include hux;\n  @include bar(pug);\n  @include bar(pug, mug);\n\
            }\n\n\
            $x: from a variable;\n\n\
            div {\n  blah: blah $x blah;\n}",
          "a {\n  hey: ho;\n  hugabug: third second;\n  \
            hugabug: kwd-y kwd-x;\n  goo: boo hoo;\n  no: parameters here;\n  \
            /* end of hux */\n  \
            flugablug: pug flug glug;\n  flugablug: pug mug glug;\n}\n\
            a div, a span {\n  some: nested stuff;\n}\n\
            a div foo, a div bar, a span foo, a span bar {\n  \
            more: stuff so forth;\n  blah: blah;\n}\n\n\
            div {\n  blah: blah from a variable blah;\n}\n")
}

#[test]
fn t18_mixin_scope() {
    check(b"$x: global x;\n$y: global y;\n\n\
            @mixin foo($x) {\n  \
            f-a: $x;\n  f-b: $y;\n  $x: local x changed by foo;\n  \
            $y: global y changed by foo !global;\n  $z: new local z;\n  \
            f-a: $x;\n  f-b: $y;\n  f-c: $z;\n}\n\n\
            div {\n  \
            a: $x;\n  b: $y;\n  @include foo(arg);\n  a: $x;\n  b: $y;\n}\n",
          "div {\n  \
            a: global x;\n  b: global y;\n  f-a: arg;\n  f-b: global y;\n  \
            f-a: local x changed by foo;\n  f-b: global y changed by foo;\n  \
            f-c: new local z;\n  a: global x;\n  \
            b: global y changed by foo;\n}\n")
}

#[test]
fn t19_full_mixin_craziness() {
    check(b"$x: global-x;\n$y: global-y;\n$z: global-z;\n\n\
            @mixin foo($x, $y) {\n  /* begin foo */\n  margin: $x $y;\n  \
            blip {\n    hey: now;\n  }\n  /* end foo */\n}\n\n\
            @mixin foogoo($x, $y, $z) {\n  margin: $x $y $z;\n}\n\n\
            @mixin hux($y) {\n  /* begin hux */\n  \
            color: $y;\n  @include foo(called-from-hux, $y: $y);\n  \
            /* end hux */\n}\n\n\
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
            div {\n  \
            /* calls to nullary mixins may omit the empty argument list */\n  \
            @include bung;\n}\n\n\
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
          "div {\n  /* begin foo */\n  margin: 1 2;\n  /* end foo */\n  \
            /* begin foo */\n  margin: 1 3;\n  /* end foo */\n  \
            margin: 1 2 zee;\n  margin: 1 kwd-y kwd-z;\n}\n\
            div blip {\n  hey: now;\n}\n\
            div blip {\n  hey: now;\n}\n\n\
            div {\n  /* begin hux */\n  color: global-y;\n  \
            /* begin foo */\n  margin: called-from-hux global-y;\n  \
            /* end foo */\n  /* end hux */\n}\n\
            div blip {\n  hey: now;\n}\n\n\
            div {\n  /* begin hux */\n  color: calling-hux-again;\n  \
            /* begin foo */\n  margin: called-from-hux calling-hux-again;\n  \
            /* end foo */\n  /* end hux */\n}\n\
            div blip {\n  hey: now;\n}\n\n\
            div {\n  blah: original-bung;\n}\n\n\
            div {\n  blah: redefined-bung;\n}\n\n\
            div {\n  \
            /* calls to nullary mixins may omit the empty argument list */\n  \
            blah: redefined-bung;\n}\n\n\
            div {\n  /* begin foo */\n  margin: kwdarg1 kwdarg2;\n  \
            /* end foo */\n}\n\
            div blip {\n  hey: now;\n}\n\n\
            hoo {\n  color: boo;\n}\n\n\
            div {\n  blah: boogoo some other default;\n}\n\n\
            div {\n  value: original;\n}\n\n\
            div {\n  value: no longer original;\n}\n\n\
            div {\n  arg: changed local x;\n  blarg: changed global y;\n  \
            a: global-x;\n  b: changed global y;\n}\n")
}

#[test]
fn t20_scoped_variables() {
    check(b"@mixin foo() {\n  /* begin foo */\n  /* assigning to $x */\n  \
            $x: inside foo;\n  x: $x;\n  /* end foo */\n}\n\n\
            outer {\n  /* assigning to $x */\n  \
            $x: inside outer scope;\n  blah: blah;\n  \
            inner {\n    @include foo();\n    x: $x;\n  }\n}",
          "outer {\n  /* assigning to $x */\n  blah: blah;\n}\n\
            outer inner {\n  /* begin foo */\n  /* assigning to $x */\n  \
            x: inside foo;\n  /* end foo */\n  x: inside outer scope;\n}\n")
}

#[test]
fn t21_one_builtin_function() {
    check(b"div {\n  color: rgb(255, $blue: 0, $green: 255);\n  \
            background: rgb(123, 45, 6);\n}\n",
          "div {\n  color: yellow;\n  background: #7b2d06;\n}\n")
}

#[test]
fn t22_colors_with_alpha() {
    check(b"$x: rgb(0, 255, 255);\n\n\
            div {\n  color: rgb(255, $blue: 0, $green: 255);\n  \
            background: rgb(123, 45, 6);\n  \
            flah: rgba(0, 0, 0, 1) + #111;\n  \
            grah: rgba(#f0e, $alpha: .5);\n  blah: rgba(1,2,3,.6);\n\n  \
            floo: $x;\n  bloo: rgba($x, 0.7);\n  groo: $x;\n\n  \
            $x: rgb(123, 45, 6);\n\n  \
            hoo: red($x);\n  moo: green($x);\n  poo: blue($x);\n\n  \
            goo: mix(rgba(255, 0, 0, 0.5), #00f);\n\n\
            boo: invert(#123456);\n}\n",
          "div {\n  color: yellow;\n  background: #7b2d06;\n  \
            flah: #111111;\n  grah: rgba(255, 0, 238, 0.5);\n  \
            blah: rgba(1, 2, 3, 0.6);\n  floo: cyan;\n  \
            bloo: rgba(0, 255, 255, 0.7);\n  groo: cyan;\n  \
            hoo: 123;\n  moo: 45;\n  poo: 6;\n  \
            goo: rgba(64, 0, 191, 0.75);\n  boo: #edcba9;\n}\n")
}

// TODO Implement tests 23 - 28 ...

#[test]
fn t29_if() {
    check(b"$x: a, b, 1+2;\n\n\
            @if type-of(nth($x, 3)) == number {\n  \
            div {\n    background: gray;\n  }\n}\n\n\
            @if type-of(nth($x, 2)) == number {\n  \
            div {\n    background: gray;\n  }\n}\n\
            @else if type-of(nth($x, 2)) == string {\n  \
            div {\n    background: blue;\n  }\n}\n\n\
            @if type-of(nth($x, 2)) == number {\n  \
            div {\n    background: gray;\n  }\n}\n\
            @else if type-of(nth($x, 2)) == color {\n  \
            div {\n    background: blue;\n  }\n}\n\
            @else {\n  div {\n    background: red;\n  }\n}",
          "div {\n  background: gray;\n}\n\n\
           div {\n  background: blue;\n}\n\n\
           div {\n  background: red;\n}\n")
}

// TODO Implement tests 30 ...

#[test]
fn t31_if_in_mixin() {
    check(b"$x: true;\n\n\
            @mixin foobar() {\n  \
            @if $x {\n    $x: false !global;\n    content: foo;\n  }\n  \
            @else {\n    $x: true !global;\n    content: bar;\n  }\n}\n\n\
            div {\n  @include foobar();\n  @include foobar();\n  \
            @include foobar();\n  $x: true !global;\n  \
            @include foobar();\n}\n",
          "div {\n  content: foo;\n  content: bar;\n  content: foo;\n  \
           content: foo;\n}\n")
}

#[test]
fn t32_percentages() {
    check(b"div {\n  width: 10% + 20%;\n  height: 10% - 20%;\n  \
            width: 10% + 10;\n  width: 10 + 10%;\n  height: 10% - 10;\n  \
            height: 10 - 10%;\n  blah: (20% / 4%);\n  flah: 12 * 75%;\n  \
            grah: 75% * 12;\n  // hwah: (24 / 8%);\n  nyah: (35% / 7);\n}",
          "div {\n  width: 30%;\n  height: -10%;\n  \
            width: 20%;\n  width: 20%;\n  height: 0%;\n  \
            height: 0%;\n  blah: 5;\n  flah: 900%;\n  \
            grah: 900%;\n  nyah: 5%;\n}\n")
}

// TODO Implement tests 33 - 35 ...

#[test]
fn t36_extra_commas_in_selectors() {
    check(b"div,, , span, ,, {\n  color: red;\n}",
          "div, span {\n  color: red;\n}\n")
}

#[test]
fn t39_dash_match_attribute_selector() {
    check(b"div[class|=\"blah\"] {\n  color: blue;\n}\n",
          "div[class|=\"blah\"] {\n  color: blue;\n}\n")
}

#[test]
fn t43_str_length() {
    check("div {\n  foo: str-length(\"protégé\");\n  \
           foo: str-length(protégé);\n  foo: str-length(\"\");\n  \
           foo: str-length(\"hello there\");\n  \
           foo: str-length(\"Façade\");\n  foo: str-length(\"Tromsø\");\n  \
           foo: str-length(\"Ãlso\");\n}"
              .as_bytes(),
          "div {\n  foo: 7;\n  foo: 7;\n  foo: 0;\n  foo: 11;\n  foo: 6;\n  \
           foo: 6;\n  foo: 4;\n}\n")
}

#[test]
fn t45_str_insert() {
    check("div {\n  bar: str-insert(\"abcd\", \"X\", 1);\n  \
           bar: str-insert(\"abcd\", 'X', 1);\n  \
           bar: str-insert(\"abcd\", 'X\\'fjd\\'sk', 1);\n  \
           bar: str-insert(\"abcd\", \"e\", 3);\n  \
           bar: str-insert(\"abcd\", \"e\", 18);\n  \
           bar: str-insert(\"abcd\", \"e\", -2);\n  \
           bar: str-insert(\"abcd\", \"e\", -18);\n  \
           bar: str-insert(\"abcd\", \"e\", 0);\n  \
           bar: str-insert(\"abcd\", e, 0);\n  \
           bar: str-insert(abcd, \"e\", 0);\n  \
           bar: str-insert(abcd, e, 0);\n  \
           bar: str-insert(\"Déjà vu\", \"abcd\", 0);\n  \
           bar: str-insert(\"Déjà vu\", \"abcd\", 2);\n  \
           bar: str-insert(\"Déjà vu\", \"abcd\", -3);\n  \
           bar: str-insert(\"Déjà vu\", \"abcd\", 18);\n\n  \
           bar: str-insert(\"øáéíóúüñ¿éàŤǅǂɊɱʭʬѪ҈ݓ\", \
           \" ABCD \", 0);\n  \
           bar: str-insert(\"øáéíóúüñ¿éàŤǅǂɊɱʭʬѪ҈ݓ\", \
           \" ABCD \", 2);\n  \
           bar: str-insert(\"øáéíóúüñ¿éàŤǅǂɊɱʭʬѪ҈ݓ\", \
           \" ABCD \", 5);\n  \
           bar: str-insert(\"øáéíóúüñ¿éàŤǅǂɊɱʭʬѪ҈ݓ\", \
           \" ABCD \", 9);\n  \
           bar: str-insert(\"øáéíóúüñ¿éàŤǅǂɊɱʭʬѪ҈ݓ\", \
           \" ABCD \", 28);\n  \
           bar: str-insert(\"øáéíóúüñ¿éàŤǅǂɊɱʭʬѪ҈ݓ\", \
           \" ABCD \", -3);\n  \
           bar: str-insert(\"øáéíóúüñ¿éàŤǅǂɊɱʭʬѪ҈ݓ\", \
           \" ABCD \", -28);\n\n}"
              .as_bytes(),
          "@charset \"UTF-8\";\n\
           div {\n  bar: \"Xabcd\";\n  bar: \"Xabcd\";\n  \
           bar: \"X'fjd'skabcd\";\n  bar: \"abecd\";\n  bar: \"abcde\";\n  \
           bar: \"abced\";\n  bar: \"eabcd\";\n  bar: \"eabcd\";\n  \
           bar: \"eabcd\";\n  bar: eabcd;\n  bar: eabcd;\n  \
           bar: \"abcdDéjà vu\";\n  \
           bar: \"Dabcdéjà vu\";\n  \
           bar: \"Déjà abcdvu\";\n  \
           bar: \"Déjà vuabcd\";\n  \
           bar: \" ABCD øáéíóúüñ¿éàŤǅǂɊ\
           ɱʭʬѪ҈ݓ\";\n  \
           bar: \"ø ABCD áéíóúüñ¿éàŤǅǂɊ\
           ɱʭʬѪ҈ݓ\";\n  \
           bar: \"øáéí ABCD óúüñ¿éàŤǅǂɊ\
           ɱʭʬѪ҈ݓ\";\n  \
           bar: \"øáéíóúüñ ABCD ¿éàŤǅǂɊ\
           ɱʭʬѪ҈ݓ\";\n  \
           bar: \"øáéíóúüñ¿éàŤǅǂɊ\
           ɱʭʬѪ҈ݓ ABCD \";\n  \
           bar: \"øáéíóúüñ¿éàŤǅǂ\
           ɊɱʭʬѪ ABCD ҈ݓ\";\n  \
           bar: \" ABCD øáéíóúüñ¿éàŤǅǂɊ\
           ɱʭʬѪ҈ݓ\";\n}\n")
}
#[test]
fn t46_str_index() {
    check("div {\n\n  bar: a str-index(\"abcde\", \"bc\");\n  \
           bar: a str-index(\"abcde\", \"a\");\n  \
           bar: a str-index(\"abcde\", \"e\");\n  \
           bar: a str-index(\"abcde\", \"xyz\");\n  \
           bar: a str-index(\"\", \"abc\");\n  \
           bar: a str-index(\"abcde\", \"abcdefg\");\n  \
           bar: a str-index(\"øáéíóúüñ¿éàŤǅǂɊɱʭʬѪ҈ݓ\", \
           \"Ɋ\"); // 15\n  \
           bar: a str-index(\"øáéíóúüñ¿éàŤǅǂɊɱʭʬѪ҈ݓ\", \
           \"úüñ\"); // 6\n  \
           bar: a str-index(\"øáéíóúüñ¿éàŤǅǂɊɱʭʬѪ҈ݓ\", \
           \"abcde\");\n  \
           bar: a str-index(\"abcde\", \"\");\n  \
           bar: a str-index(\"\", \"\");\n\n}"
              .as_bytes(),
          "div {\n  bar: a 2;\n  bar: a 1;\n  bar: a 5;\n  bar: a;\n  \
           bar: a;\n  bar: a;\n  bar: a 15;\n  bar: a 6;\n  bar: a;\n  \
           bar: a 1;\n  bar: a 1;\n}\n")
}

#[test]
fn t48_case_conversion() {
    check(b"div {\n\n  bar: to-upper-case(\"blah\");\n  \
            bar: to-upper-case(\"BLAH\");\n  bar: to-upper-case(\"bLaH\");\n  \
            bar: to-upper-case(\"1232178942\");\n  \
            bar: to-upper-case(blah);\n  bar: to-upper-case(BLAH);\n  \
            bar: to-upper-case(bLaH);\n  bar: to-upper-case(\"\");\n\n  \
            bar: to-lower-case(\"blah\");\n  bar: to-lower-case(\"BLAH\");\n  \
            bar: to-lower-case(\"bLaH\");\n  \
            bar: to-lower-case(\"1232178942\");\n  \
            bar: to-lower-case(blah);\n  bar: to-lower-case(BLAH);\n  \
            bar: to-lower-case(bLaH);\n  bar: to-lower-case(\"\");\n\n}\n",
          "div {\n  bar: \"BLAH\";\n  bar: \"BLAH\";\n  bar: \"BLAH\";\n  \
           bar: \"1232178942\";\n  \
           bar: BLAH;\n  bar: BLAH;\n  bar: BLAH;\n  bar: \"\";\n  \
           bar: \"blah\";\n  bar: \"blah\";\n  bar: \"blah\";\n  \
           bar: \"1232178942\";\n  \
           bar: blah;\n  bar: blah;\n  bar: blah;\n  bar: \"\";\n}\n")
}

#[test]
fn t54_adjacent_identifiers_with_hyphens() {
    check(b"input {\n    outline: 5px auto -webkit-focus-ring-color;\n    \
            foo: random -hello-this-is-dog;\n    \
            bar: rando -two-or-more -things-that-start -with-hyphens;\n    \
            baz: foo - bar;\n}",
          "input {\n  outline: 5px auto -webkit-focus-ring-color;\n  \
           foo: random -hello-this-is-dog;\n  \
           bar: rando -two-or-more -things-that-start -with-hyphens;\n  \
           baz: foo-bar;\n}\n")
}

#[test]
fn t59_if_expression() {
    check(b"$x: 0;\n$if-false: whatever;\n\n\
            div {\n  \
            foo: if($if-true: hey, $if-false: ho, $condition: true);\n  \
            foo: if($if-true: hey, $if-false: ho, $condition: false);\n  \
            foo: if($x != 0, if($x, true, false), unquote(\"x is zero\"));\n  \
            foo: if(false, 1/0, $if-false: $if-false);\n}",
          "div {\n  \
           foo: hey;\n  foo: ho;\n  foo: x is zero;\n  foo: whatever;\n}\n")

}

/// No proper spec-test for str-slice, this is from
/// spec/libsass-closed-issues/issue_760/input.scss
#[test]
fn ti815_str_slice() {
    check(b"foo {\n  foo: str-slice(\"bar\", 1, 2);\n  \
            bar: str-slice(\"bar\", 3);\n}\n",
          "foo {\n  foo: \"ba\";\n  bar: \"r\";\n}\n")
}

fn check(input: &[u8], expected: &str) {
    assert_eq!(compile_scss(input, OutputStyle::Normal).and_then(|s| {
                   String::from_utf8(s)
                       .map_err(|e| format!("Non-utf8 output: {}", e))
               }),
               Ok(expected.into()));
}
