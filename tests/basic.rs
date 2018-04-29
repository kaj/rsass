//! These are from the "basic" directory in the sass specification.
//! See <https://github.com/sass/sass-spec> for source material.
//! I add one a test function for one specification at a time and then
//! try to implement that functionality without breaking those already
//! added.
extern crate rsass;
use rsass::{compile_scss, compile_scss_file, OutputStyle};

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
    check(
        b"a {\n  \
            color: blue;\n\
            }",
        "a {\n  \
         color: blue;\n\
         }\n",
    )
}

#[test]
fn t02_simple_nesting() {
    check(
        b"div {\n  \
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
         }\n",
    )
}

#[test]
fn t03_simple_variable() {
    check(
        b"$color: red;\n\
            \n\
            a {\n  \
            color: $color;\n\
            }",
        "a {\n  \
         color: red;\n\
         }\n",
    )
}

#[test]
fn t04_basic_variables() {
    check(
        b"$color: \"black\";\n\
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
         ",
    )
}

#[test]
fn t05_empty_levels() {
    check(
        b"div {\n  span {\n    color: red;\n    background: blue;\n  }\n}\n\
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
         empty1 empty2 div empty3 span {\n  blah: blah;\n  blah: blah;\n}\n",
    )
}

#[test]
fn t06b_simpler_comments() {
    check(
        b"/* top level comment\n   \
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
         /* Final preserved comment */\n",
    )
}

#[test]
fn t06c_simple_line_comment() {
    check(
        b"span {\n  \
            color: red; // line comment goes away\n\
            }\n",
        "span {\n  \
         color: red;\n\
         }\n",
    )
}

#[test]
fn t06d_simple_local_variable() {
    check(
        b"$red: #ff0000;\n\
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
         }\n",
    )
}

#[test]
fn t06_nesting_and_comments() {
    check(
        b"$blah: bloo blee;\n\
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
         div empty_with_comment span {\n  c: d;\n}\n",
    )
}

#[test]
fn t07_nested_simple_selector_groups() {
    check(
        b"a, b {\n  color: red;\n  background: blue;\n}\n\n\
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
         wow: we are far inside;\n  but: it still works;\n}\n",
    )
}

#[test]
fn t08_selector_combinators() {
    check(
        b"a   +   b  >  c {\n  \
            d e {\n    color: blue;\n    background: white;\n  }\n  \
            color: red;\n  background: gray;\n}",
        "a + b > c {\n  color: red;\n  background: gray;\n}\n\
         a + b > c d e {\n  color: blue;\n  background: white;\n}\n",
    )
}

#[test]
fn t09_selector_groups_and_combinators() {
    check(
        b"a + b, c {\n  blah: blah;\n  bleh: bleh;\n  \
            d e, f ~ g + h, > i {\n    bloo: bloo;\n    blee: blee;\n  }\n}",
        "a + b, c {\n  blah: blah;\n  bleh: bleh;\n}\n\
         a + b d e, a + b f ~ g + h, a + b > i, c d e, c f ~ g + h, c > i \
         {\n  bloo: bloo;\n  blee: blee;\n}\n",
    )
}

#[test]
fn t10_classes_and_ids() {
    check(
        b"a + b, .class {\n  blah: blah;\n  bleh: bleh;\n  \
            d #id, f ~ g.other + h, > i#grar \
            {\n    bloo: bloo;\n    blee: blee;\n  }\n}",
        "a + b, .class {\n  blah: blah;\n  bleh: bleh;\n}\n\
         a + b d #id, a + b f ~ g.other + h, a + b > i#grar, \
         .class d #id, .class f ~ g.other + h, .class > i#grar \
         {\n  bloo: bloo;\n  blee: blee;\n}\n",
    )
}

#[test]
fn t11_attribute_selectors() {
    check(
        b"[hey  =  'ho'], a > b {\n  blah: blah;\n  \
            c, [hoo *=    \"ha\" ] {\n    bloo: bloo;\n  }\n}",
        "[hey='ho'], a > b {\n  blah: blah;\n}\n\
         [hey='ho'] c, [hey='ho'] [hoo*=\"ha\"], a > b c, \
         a > b [hoo*=\"ha\"] {\n  bloo: bloo;\n}\n",
    )
}

#[test]
fn t12_pseudo_classes_and_elements() {
    check(
        b"a b {\n  color: red;\n  \
            :first-child, :nth-of-type(  -2n+1 ) {\n    \
            .foo#bar:nth-child(even) {\n      hoo: goo;\n    }\n    \
            blah: bloo;\n    \
            ::after {\n      content: \"glux\";\n      \
            color: green;\n    }\n    \
            :not(.foo) {\n      hoo: boo;\n    }\n    a { b: c; }\n  }\n}",
        "a b {\n  color: red;\n}\n\
         a b :first-child, a b :nth-of-type(-2n+1) {\n  blah: bloo;\n}\n\
         a b :first-child .foo#bar:nth-child(even), \
         a b :nth-of-type(-2n+1) .foo#bar:nth-child(even) {\n  hoo: goo;\n}\n\
         a b :first-child ::after, a b :nth-of-type(-2n+1) ::after {\n  \
         content: \"glux\";\n  color: green;\n}\n\
         a b :first-child :not(.foo), a b :nth-of-type(-2n+1) :not(.foo) \
         {\n  hoo: boo;\n}\n\
         a b :first-child a, a b :nth-of-type(-2n+1) a {\n  b: c;\n}\n",
    )
}

#[test]
fn t13_back_references() {
    check(
        b"hey, ho {\n  \
            & > boo, foo &.goo {\n    bloo: bloo;\n  }\n  \
            blah: blah;\n\
            }",
        "hey, ho {\n  blah: blah;\n}\n\
         hey > boo, foo hey.goo, ho > boo, foo ho.goo {\n  bloo: bloo;\n}\n",
    )
}

#[test]
fn t14_imports() {
    let path = "tests/basic/14_imports/input.scss";
    assert_eq!(
        compile_scss_file(path.as_ref(), OutputStyle::Expanded)
            .and_then(|s| Ok(String::from_utf8(s)?))
            .unwrap(),
        "div span {\n  moo: goo;\n}\n\n\
         foo {\n  blah: blah;\n}\n\
         foo goo {\n  blee: blee;\n  hello: world;\n}\n\
         foo goo hoo {\n  mux: scooba-dee-doo;\n  \
         flux: gooboo boo;\n}\n\
         foo goo hoo d {\n  inside: d now;\n}\n\
         foo blux {\n  hey: another thing;\n  \
         ho: will this work;\n}\n"
    )
}

#[test]
fn t15_arithmetic_and_lists_abcd() {
    check(
        b"div {\n  a: 1 + 2;\n  b: 3 + 3/4;\n  c: 1/2 + 1/2;\n  \
            /* shouldn't eval the following \"300\" */\n  \
            d: 300;\n}",
        "div {\n  a: 3;\n  b: 3.75;\n  c: 1;\n  \
         /* shouldn't eval the following \"300\" */\n  \
         d: 300;\n}\n",
    )
}

#[test]
fn t15_arithmetic_and_lists_efg() {
    check(
        b"div {\n  e: 1 + (5/10 2 3);\n  f: 1 + ((2+(3 4) 5) 6);\n  \
            g: 1 + ((1+(14/7 8) 9) 6);\n}",
        "div {\n  e: 15/10 2 3;\n  f: 123 4 5 6;\n  g: 1114/7 8 9 6;\n}\n",
    )
}
#[test]
fn t15_arithmetic_and_lists_hijklmt_div_or_not() {
    check(
        b"$stuff: 1 2 3;\n\
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
         t: 120.754/5 6/7;\n}\n",
    )
}

#[test]
fn t15_arithmetic_and_lists_nopqrs() {
    check(
        b"$stuff: 1 2 3;\n\
            div {\n  n: 1 2 3, $stuff 4 5 (6, 7 8 9);\n  \
            o: 3px + 3px + 3px;\n  p: 4 + 1px;\n  q: (20pt / 10pt);\n  \
            r: 16em * 4;\n  s: (5em / 2);\n}",
        "div {\n  n: 1 2 3, 1 2 3 4 5 6, 7 8 9;\n  \
         o: 9px;\n  p: 5px;\n  q: 2;\n  r: 64em;\n  s: 2.5em;\n}\n",
    )
}

#[test]
fn t15_arithmetic_and_lists() {
    check(
        b"$stuff: 1 2 3;\n\n\
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
         t: 120.754/5 6/7;\n}\n",
    )
}

#[test]
fn t16_hex_arithmetic() {
    check(
        b"div {\n  p01: #AbC;\n  p02: #AAbbCC;\n  p03: #AbC + hello;\n  \
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
         p20: #0b0a0b;\n  p21: white;\n}\n",
    )
}

#[test]
fn t17_basic_mixins() {
    check(
        b"@mixin foo($x, $y) {\n  hugabug: $y $x;\n}\n\n\
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
         div {\n  blah: blah from a variable blah;\n}\n",
    )
}

#[test]
fn t18_mixin_scope() {
    check(
        b"$x: global x;\n$y: global y;\n\n\
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
         b: global y changed by foo;\n}\n",
    )
}

#[test]
fn t19_full_mixin_craziness() {
    check(
        b"$x: global-x;\n$y: global-y;\n$z: global-z;\n\n\
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
         a: global-x;\n  b: changed global y;\n}\n",
    )
}

#[test]
fn t20_scoped_variables() {
    check(
        b"@mixin foo() {\n  /* begin foo */\n  /* assigning to $x */\n  \
            $x: inside foo;\n  x: $x;\n  /* end foo */\n}\n\n\
            outer {\n  /* assigning to $x */\n  \
            $x: inside outer scope;\n  blah: blah;\n  \
            inner {\n    @include foo();\n    x: $x;\n  }\n}",
        "outer {\n  /* assigning to $x */\n  blah: blah;\n}\n\
         outer inner {\n  /* begin foo */\n  /* assigning to $x */\n  \
         x: inside foo;\n  /* end foo */\n  x: inside outer scope;\n}\n",
    )
}

#[test]
fn t21_one_builtin_function() {
    check(
        b"div {\n  color: rgb(255, $blue: 0, $green: 255);\n  \
            background: rgb(123, 45, 6);\n}\n",
        "div {\n  color: yellow;\n  background: #7b2d06;\n}\n",
    )
}

#[test]
fn t22_colors_with_alpha() {
    check(
        b"$x: rgb(0, 255, 255);\n\n\
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
         goo: rgba(64, 0, 191, 0.75);\n  boo: #edcba9;\n}\n",
    )
}

#[test]
fn t23_basic_value_interpolation_4_0() {
    check(
        b"div {\n  a: hello#{world};\n  a: hello #{world};\n  \
            b: 12#{3};\n  b: type-of(12#{3});\n  b: #{12 + 111};\n  \
            b: type-of(#{12 + 111});\n}",
        "div {\n  a: helloworld;\n  a: hello world;\n  \
         b: 12 3;\n  b: list;\n  b: 123;\n  b: string;\n}\n",
    )
}

#[test]
fn t24_namespace_properties() {
    check(
        b"div {\n  a: {\n    \
            p1: q;\n    b: {\n      p2: q;\n    }\n    p3: q;\n  }\n}\n\n\
            foo {\n  bar: baz {\n    bip: bop;\n    \
            bing: type-of(\"hello\");\n    bang: 1 + 2;\n    bung: bap;\n    \
            bong: bup {\n      x: x;\n      y: y;\n      z: z;\n    \
            }\n  }\n}\n",
        "div {\n  a-p1: q;\n  a-b-p2: q;\n  a-p3: q;\n}\n\n\
         foo {\n  bar: baz;\n  bar-bip: bop;\n  bar-bing: string;\n  \
         bar-bang: 3;\n  bar-bung: bap;\n  bar-bong: bup;\n  \
         bar-bong-x: x;\n  bar-bong-y: y;\n  bar-bong-z: z;\n}\n",
    )
}

#[test]
fn t25_basic_string_interpolation() {
    check(
        b"div {\n  blah: \"hello #{2+2} world #{unit(23px)} #{'bloo\\n'} \
            blah\";\n}",
        "div {\n  blah: \"hello 4 world px bloon blah\";\n}\n",
    )
}

#[test]
fn t26_selector_interpolation() {
    check(
        b"$x: oo, ba;\n$y: az, hu;\n\n\
            f#{$x}r {\n  p: 1;\n  b#{$y}x {\n    \
            q: 2;\n    mumble#{length($x) + length($y)} {\n      r: 3;\n    \
            }\n  }\n}",
        "foo, bar {\n  p: 1;\n}\n\
         foo baz, foo hux, bar baz, bar hux {\n  q: 2;\n}\n\
         foo baz mumble4, foo hux mumble4, bar baz mumble4, \
         bar hux mumble4 {\n  r: 3;\n}\n",
    )
}

#[test]
fn t27_media_queries() {
    check(
        b"a b c {\n  blee: blee;\n  d e f {\n    blah: blah;\n    \
            bloo: bloo;\n  }\n  g h, i j {\n    \
            @media print and (foo: 1 2 3), (bar: 3px hux(muz)), not screen \
            {\n      hey: ho;\n      k l m {\n        hee: fee;\n      }\n    \
            }\n  }\n  blah: blah;\n}\n\n\n",
        "a b c {\n  blee: blee;\n  blah: blah;\n}\n\
         a b c d e f {\n  blah: blah;\n  bloo: bloo;\n}\n\
         @media print and (foo: 1 2 3), (bar: 3px hux(muz)), not screen \
         {\n  a b c g h, a b c i j {\n    hey: ho;\n  }\n  \
         a b c g h k l m, a b c i j k l m {\n    hee: fee;\n  }\n}\n",
    )
}

#[test]
fn t28_url() {
    check(
        b"$x: pop;\n$y: 123;\n\n\n\n\
            div {\n  foo: url(bloo/blah.css);\n  \
            bar: url(http://foo/bar/hux.css);\n  foo: url(fudge#{$x}.css);\n  \
            bar: url(\"http://fudge#{$x}/styles.css\");\n  \
            hux: url(http://box_#{$y}////fudge#{$x}.css);\n  \
            @each $i in (1 2 3 4 5) {\n    \
            hux: url(http://box_#{$y}////fudge#{$x}.css);\n    \
            foo: url(http://blah.com/bar-#{$i}.css);\n    \
            bar: url(http://fonts.googleapis.com/css?family=Karla:400,700,\
            400italic|Anonymous+Pro:400,700,400italic);\n  }\n  \
            gloo: url(\"hey#{1+2}.css\");\n  \
            floo: url(hadoop-#{$y+321}.css);\n}\n",
        "div {\n  foo: url(bloo/blah.css);\n  \
         bar: url(http://foo/bar/hux.css);\n  foo: url(fudgepop.css);\n  \
         bar: url(\"http://fudgepop/styles.css\");\n  \
         hux: url(http://box_123////fudgepop.css);\n  \
         hux: url(http://box_123////fudgepop.css);\n  \
         foo: url(http://blah.com/bar-1.css);\n  \
         bar: url(http://fonts.googleapis.com/css?family=Karla:400,700,\
         400italic|Anonymous+Pro:400,700,400italic);\n  \
         hux: url(http://box_123////fudgepop.css);\n  \
         foo: url(http://blah.com/bar-2.css);\n  \
         bar: url(http://fonts.googleapis.com/css?family=Karla:400,700,\
         400italic|Anonymous+Pro:400,700,400italic);\n  \
         hux: url(http://box_123////fudgepop.css);\n  \
         foo: url(http://blah.com/bar-3.css);\n  \
         bar: url(http://fonts.googleapis.com/css?family=Karla:400,700,\
         400italic|Anonymous+Pro:400,700,400italic);\n  \
         hux: url(http://box_123////fudgepop.css);\n  \
         foo: url(http://blah.com/bar-4.css);\n  \
         bar: url(http://fonts.googleapis.com/css?family=Karla:400,700,\
         400italic|Anonymous+Pro:400,700,400italic);\n  \
         hux: url(http://box_123////fudgepop.css);\n  \
         foo: url(http://blah.com/bar-5.css);\n  \
         bar: url(http://fonts.googleapis.com/css?family=Karla:400,700,\
         400italic|Anonymous+Pro:400,700,400italic);\n  \
         gloo: url(\"hey3.css\");\n  floo: url(hadoop-444.css);\n}\n",
    )
}

#[test]
fn t29_if() {
    check(
        b"$x: a, b, 1+2;\n\n\
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
         div {\n  background: red;\n}\n",
    )
}

#[test]
fn t30_if_in_function() {
    check(
        b"$x: true;\n\n\
            @function foobar() {\n  \
            @if $x {\n    $x: false !global;\n    @return foo;\n  }\n  \
            @else {\n    $x: true !global;\n    @return bar;\n  }\n}\n\n\
            div {\n  content: foobar();\n  content: foobar();\n  \
            content: foobar();\n  content: foobar();\n  \
            $x: false !global;\n  content: foobar();\n}\n",
        "div {\n  content: foo;\n  content: bar;\n  content: foo;\n  \
         content: bar;\n  content: bar;\n}\n",
    )
}

#[test]
fn t31_if_in_mixin() {
    check(
        b"$x: true;\n\n\
            @mixin foobar() {\n  \
            @if $x {\n    $x: false !global;\n    content: foo;\n  }\n  \
            @else {\n    $x: true !global;\n    content: bar;\n  }\n}\n\n\
            div {\n  @include foobar();\n  @include foobar();\n  \
            @include foobar();\n  $x: true !global;\n  \
            @include foobar();\n}\n",
        "div {\n  content: foo;\n  content: bar;\n  content: foo;\n  \
         content: foo;\n}\n",
    )
}

#[test]
fn t32_percentages() {
    check(
        b"div {\n  width: 10% + 20%;\n  height: 10% - 20%;\n  \
            width: 10% + 10;\n  width: 10 + 10%;\n  height: 10% - 10;\n  \
            height: 10 - 10%;\n  blah: (20% / 4%);\n  flah: 12 * 75%;\n  \
            grah: 75% * 12;\n  // hwah: (24 / 8%);\n  nyah: (35% / 7);\n}",
        "div {\n  width: 30%;\n  height: -10%;\n  \
         width: 20%;\n  width: 20%;\n  height: 0%;\n  \
         height: 0%;\n  blah: 5;\n  flah: 900%;\n  \
         grah: 900%;\n  nyah: 5%;\n}\n",
    )
}

#[test]
fn t33_ambigous_imports() {
    let path = "tests/basic/33_ambiguous_imports/input.scss";
    assert_eq!(
        compile_scss_file(path.as_ref(), OutputStyle::Expanded)
            .and_then(|s| Ok(String::from_utf8(s)?))
            .unwrap(),
        "main {\n  color: red;\n}\n\n\
         dir {\n  color: blue;\n}\n\n\
         fudge {\n  color: brown;\n}\n"
    )
}

#[test]
fn t35_varargs_false() {
    check(
        b"@mixin foo($args...) {\n  @each $arg in $args {\n    \
            @if $arg {\n      thing: $arg;\n    }\n  }\n}\n\n\
            div {\n  @include foo(a, b, false);\n}\n",
        "div {\n  thing: a;\n  thing: b;\n}\n",
    )
}

#[test]
fn t36_extra_commas_in_selectors() {
    check(
        b"div,, , span, ,, {\n  color: red;\n}",
        "div, span {\n  color: red;\n}\n",
    )
}

#[test]
fn t37_url_expressions() {
    check(
        b"$x: true;\n$file-1x: \"budge.png\";\n\n\
            @function fudge($str) {\n  @return \"assets/fudge/\" + $str;\n}\n\n\
            div {\n  blah: url(foo + bar);\n  blah: url(fn(\"s\"));\n  \
            blah: url(if(true, \"red.png\", \"blue.png\"));\n  \
            blah: url(hello-#{world}.png);\n  \
            blah: url(if($x, fudge(\"#{$file-1x}\"), \"#{$file-1x}\"));\n}",
        "div {\n  blah: url(foobar);\n  blah: url(fn(\"s\"));\n  \
         blah: url(\"red.png\");\n  blah: url(hello-world.png);\n  \
         blah: url(\"assets/fudge/budge.png\");\n}\n",
    )
}

#[test]
fn t38_expressions_in_at_directives() {
    // Note: This actually checks that expressions are _not_
    // evalutated in at directives!
    check(
        b"$x: 1;\n$y: 2;\n\n\
            @foo $x $y, hux {\n  \
            bar {\n    whatever: whatever;\n  }\n}\n",
        "@foo $x $y, hux {\n  bar {\n    whatever: whatever;\n  }\n}\n",
    )
}

#[test]
fn t39_dash_match_attribute_selector() {
    check(
        b"div[class|=\"blah\"] {\n  color: blue;\n}\n",
        "div[class|=\"blah\"] {\n  color: blue;\n}\n",
    )
}

#[test]
fn t40_pseudo_class_identifier_starting_with_n() {
    check(
        b"div:lang(nb) {\n  color: blue;\n}",
        "div:lang(nb) {\n  color: blue;\n}\n",
    )
}

#[test]
fn t41_slashy_urls() {
    check(
        b"div {\n  blah: url(//some/absolute/path);\n  \
            blee: url(/*looks-like-a*/comment);\n}",
        "div {\n  blah: url(//some/absolute/path);\n  \
         blee: url(/*looks-like-a*/comment);\n}\n",
    )
}

#[test]
fn t42_css_imports() {
    check(
        b"div {\n  color: red;\n}\n\n\
            @import \"hux\\ bux.css\";\n\
            @import \"foo.css\";\n\n\
            span {\n  color: blue;\n}\n\n\
            @import \"bar.css\";",
        "@import url(hux bux.css);\n\
         @import url(foo.css);\n\
         @import url(bar.css);\n\
         div {\n  color: red;\n}\n\n\
         span {\n  color: blue;\n}\n",
    )
}

#[test]
fn t43_str_length() {
    check(
        "div {\n  foo: str-length(\"protégé\");\n  \
         foo: str-length(protégé);\n  foo: str-length(\"\");\n  \
         foo: str-length(\"hello there\");\n  \
         foo: str-length(\"Façade\");\n  foo: str-length(\"Tromsø\");\n  \
         foo: str-length(\"Ãlso\");\n}"
            .as_bytes(),
        "div {\n  foo: 7;\n  foo: 7;\n  foo: 0;\n  foo: 11;\n  foo: 6;\n  \
         foo: 6;\n  foo: 4;\n}\n",
    )
}

#[test]
fn t44_bem_selectors() {
    check(
        b"div {\n\n  &_foo {\n    blah: blah;\n  }\n  \
            &--modifier {\n    blach: blah;\n  }\n  \
            &hux {\n    blah: blah;\n  }\n  \
            &div.foo#bar[hux] {\n    blah: blah;\n  }\n\n}",
        "div_foo {\n  blah: blah;\n}\n\
         div--modifier {\n  blach: blah;\n}\n\
         divhux {\n  blah: blah;\n}\n\
         divdiv.foo#bar[hux] {\n  blah: blah;\n}\n",
    )
}

#[test]
fn t45_str_insert() {
    check(
        "div {\n  bar: str-insert(\"abcd\", \"X\", 1);\n  \
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
         ɱʭʬѪ҈ݓ\";\n}\n",
    )
}
#[test]
fn t46_str_index() {
    check(
        "div {\n\n  bar: a str-index(\"abcde\", \"bc\");\n  \
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
         bar: a 1;\n  bar: a 1;\n}\n",
    )
}

#[test]
fn t48_case_conversion() {
    check(
        b"div {\n\n  bar: to-upper-case(\"blah\");\n  \
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
         bar: blah;\n  bar: blah;\n  bar: blah;\n  bar: \"\";\n}\n",
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
        "div :-moz-any(ol p.blah, ul, menu, dir) \
         :-moz-any(ol span + h1, ul, menu, dir) ul {\n  \
         list-style-type: square;\n}\n\
         div :-moz-any(ol span + h1, ul, menu, dir) ul {\n  \
         list-style-type: square;\n}\n\
         div :foo(p div) {\n  hi: hi;\n}\n\
         div :foo(ol) {\n  hi: hi;\n}\n",
    )
}

#[test]
fn t51_trailing_commas_in_list() {
    check(
        b"$mylist: (alpha, beta, gamma, );\n\
            $my-single-item-list: (alpha,);\n\
            .test { \n  out1: length($mylist);\n  \
            blah: type-of(nth($mylist,3));\n  \
            out: length($my-single-item-list); \n}",
        ".test {\n  out1: 3;\n  blah: string;\n  out: 1;\n}\n",
    )
}

#[test]
fn t52a_each_loop() {
    check(
        b"@each $my_cool-var in a, b, c {\n  \
            div {\n    color: $my-cool_var;\n  }\n}",
        "div {\n  color: a;\n}\n\n\
         div {\n  color: b;\n}\n\n\
         div {\n  color: c;\n}\n",
    )
}

#[test]
fn t52b_for_loop() {
    check(
        b"@for $my_cool_var from 1 to 4 {\n  \
            div {\n    color: $my-cool-var;\n  }\n}\n",
        "div {\n  color: 1;\n}\n\n\
         div {\n  color: 2;\n}\n\n\
         div {\n  color: 3;\n}\n",
    )
}
#[test]
fn t52b_for_loop_through() {
    check(
        b"@for $my_cool_var from 1 through 4 {\n  \
            div {\n    color: $my-cool-var;\n  }\n}\n",
        "div {\n  color: 1;\n}\n\n\
         div {\n  color: 2;\n}\n\n\
         div {\n  color: 3;\n}\n\n\
         div {\n  color: 4;\n}\n",
    )
}

#[test]
fn t52_interchangeable_hyphens_underscores() {
    check(
        b"$my-cool-var: \"hello\";\n\
            @mixin my-cool-mixin($yada-yada) {\n  blah: blah;\n  \
            hi: $yada_yada;\n}\n\
            @function my-cool-function($cool_arg) {\n  \
            @return $cool-arg;\n}\n\n\
            div {\n  @include my_cool-mixin($yada_yada: \"hi\");\n  \
            @include my_cool-mixin($my_cool-var);\n  \
            foo: my-cool_function($cool-arg: \"boop\");\n  \
            foo: my-cool_function($my-cool_var);\n  bar: $my_cool_var;\n}\n\n\
            @each $my_cool_var in a, b, c {\n  \
            div {\n    color: $my-cool-var;\n  }\n}\n\n\
            @for $my_cool_var from 1 to 10 {\n  \
            div {\n    color: $my-cool-var;\n  }\n}\n\n\n\
            @function blah_blah() {\n  @return blah;\n}\n\n\
            div {\n  foo: blah-blah();\n}",
        "div {\n  blah: blah;\n  hi: \"hi\";\n  blah: blah;\n  \
         hi: \"hello\";\n  foo: \"boop\";\n  foo: \"hello\";\n  \
         bar: \"hello\";\n}\n\n\
         div {\n  color: a;\n}\n\ndiv {\n  color: b;\n}\n\n\
         div {\n  color: c;\n}\n\ndiv {\n  color: 1;\n}\n\n\
         div {\n  color: 2;\n}\n\ndiv {\n  color: 3;\n}\n\n\
         div {\n  color: 4;\n}\n\ndiv {\n  color: 5;\n}\n\n\
         div {\n  color: 6;\n}\n\ndiv {\n  color: 7;\n}\n\n\
         div {\n  color: 8;\n}\n\ndiv {\n  color: 9;\n}\n\n\
         div {\n  foo: blah;\n}\n",
    )
}

#[test]
fn t53_escaped_quotes() {
    check(
        "[data-icon='test-1']:before {\n    content:'\\\\';\n}\n\n\
         [data-icon='test-2']:before {\n    content:'\\'';\n}\n\n\
         [data-icon='test-3']:before {\n    content:\"\\\"\";\n}\n\n\
         [data-icon='test-4']:before {\n    content:'\\\\';\n}\n\n\
         [data-icon='test-5']:before {\n    content:'\\'';\n}\n\n\
         [data-icon='test-6']:before {\n    content:\"\\\"\";\n}\n\n\
         $open-quote:    «;\n$close-quote:   »;\n\n\
         $open-quote: \\201C;\n$close-quote: \\201D;\n\n\
         .\\E9motion { \nblah: hi; }\n.\\E9 dition { \nblah: hi; }\n.\
         \\0000E9dition { \nblah: hi; }\n"
            .as_bytes(),
        "[data-icon='test-1']:before {\n  content: '\\\\';\n}\n\n\
         [data-icon='test-2']:before {\n  content: '\\'';\n}\n\n\
         [data-icon='test-3']:before {\n  content: \"\\\"\";\n}\n\n\
         [data-icon='test-4']:before {\n  content: '\\\\';\n}\n\n\
         [data-icon='test-5']:before {\n  content: '\\'';\n}\n\n\
         [data-icon='test-6']:before {\n  content: \"\\\"\";\n}\n\n\
         .\\E9motion {\n  blah: hi;\n}\n\n.\\E9 dition {\n  blah: hi;\n}\n\n\
         .\\0000E9dition {\n  blah: hi;\n}\n",
    )
}

#[test]
fn t54_adjacent_identifiers_with_hyphens() {
    check(
        b"input {\n    outline: 5px auto -webkit-focus-ring-color;\n    \
            foo: random -hello-this-is-dog;\n    \
            bar: rando -two-or-more -things-that-start -with-hyphens;\n    \
            baz: foo - bar;\n}",
        "input {\n  outline: 5px auto -webkit-focus-ring-color;\n  \
         foo: random -hello-this-is-dog;\n  \
         bar: rando -two-or-more -things-that-start -with-hyphens;\n  \
         baz: foo-bar;\n}\n",
    )
}

#[test]
fn t55_variable_exists() {
    check(
        b"@function exists($name) {\n  @return variable-exists($name);\n}\n\n\
            @function f() {\n  $foo: hi;\n  @return g();\n}\n\n\
            @function g() {\n  @return variable-exists(foo);\n}\n\n\
            div {\n  foo: variable-exists(x);\n  \
            foo: variable-exists(\"x\");\n\n  \
            span {\n    $x: false;\n\n    foo: variable-exists(x);\n    \
            foo: variable-exists(\"x\");\n    foo: variable-exists(y);\n    \
            foo: variable-exists(\"y\");\n    foo: exists(x);\n    \
            foo: exists(\"x\");\n\n\
            p {\n      foo: variable-exists(x);\n      \
            foo: variable-exists(\"x\");\n      foo: exists(x);\n      \
            foo: exists(\"x\");\n      foo: variable-exists(y);\n      \
            foo: variable-exists(\"y\");\n      foo: f();\n      \
            $y: blah;\n    }\n  }\n}\n",
        "div {\n  foo: false;\n  foo: false;\n}\n\
         div span {\n  foo: true;\n  foo: true;\n  foo: false;\n  \
         foo: false;\n  foo: false;\n  foo: false;\n}\n\
         div span p {\n  foo: true;\n  foo: true;\n  foo: false;\n  \
         foo: false;\n  foo: false;\n  foo: false;\n  foo: false;\n}\n",
    )
}

#[test]
fn t56_global_variable_exists() {
    check(
        b"@function exists($name) {\n  \
            @return global-variable-exists($name);\n}\n\n\
            @function f() {\n  $foo: hi;\n  @return g();\n}\n\n\
            @function g() {\n  @return global-variable-exists(foo);\n}\n\n\
            $z: hi;\n\n\
            div {\n  foo: global-variable-exists(x);    \n  \
            foo: global-variable-exists(\"x\");    \n  \
            foo: global-variable-exists(z);\n  \
            foo: global-variable-exists(\"z\");    \n\n  \
            span {\n    $x: false;\n\n    \
            foo: global-variable-exists(x);\n    \
            foo: global-variable-exists(\"x\");    \n    \
            foo: global-variable-exists(y);\n    \
            foo: global-variable-exists(\"y\");    \n\n    \
            foo: global-variable-exists(z);\n    \
            foo: global-variable-exists(\"z\");    \n\n    \
            p {\n      foo: global-variable-exists(x);\n      \
            foo: global-variable-exists(\"x\");    \n      \
            foo: exists(x);\n      foo: exists(\"x\");    \n      \
            foo: global-variable-exists(z);\n      \
            foo: global-variable-exists(\"z\");    \n      \
            foo: global-variable-exists(y);\n      \
            foo: global-variable-exists(\"y\");   \n      \
            foo: f();\n      $y: blah;\n      //TODO: check for shadowing\n    \
            }\n  }\n\n}\n",
        "div {\n  foo: false;\n  foo: false;\n  foo: true;\n  foo: true;\n}\n\
         div span {\n  foo: false;\n  foo: false;\n  foo: false;\n  \
         foo: false;\n  foo: true;\n  foo: true;\n}\n\
         div span p {\n  foo: false;\n  foo: false;\n  foo: false;\n  \
         foo: false;\n  foo: true;\n  foo: true;\n  foo: false;\n  \
         foo: false;\n  foo: false;\n}\n",
    )
}

#[test]
fn t57_function_exists() {
    check(
        b"@function exists($name) {\n  @return function-exists($name);\n}\
            \n\n\
            @function f() {\n  $foo: hi;\n  @return g();\n}\n\n\
            @function g() {\n  @return function-exists(foo);\n}\n\n\
            @function h() {\n  @return function-exists(lighten);\n}\n\n\
            div {\n  foo: function-exists(lighten); \n  \
            foo: function-exists(\"lighten\"); \n  \
            foo: function-exists(exists);\n  \
            foo: function-exists(\"exists\"); \n  \
            foo: function-exists(f);\n  foo: function-exists(\"f\"); \n  \
            foo: function-exists(g);\n  foo: function-exists(\"g\"); \n  \
            foo: function-exists(nope);\n  foo: function-exists(\"nope\"); \n  \
            foo: g();\n  foo: f();\n  foo: h();\n\n\n  \
            span {\n    foo: function-exists(lighten); \n    \
            foo: function-exists(\"lighten\"); \n    \
            foo: function-exists(exists);\n    \
            foo: function-exists(\"exists\"); \n    \
            foo: function-exists(f);\n    foo: function-exists(\"f\"); \n    \
            foo: function-exists(g);\n    foo: function-exists(\"g\"); \n    \
            foo: function-exists(nope);\n    \
            foo: function-exists(\"nope\"); \n    \
            foo: g();\n    foo: f();\n    foo: h();\n    \
            p {\n      foo: function-exists(lighten); \n      \
            foo: function-exists(\"lighten\"); \n      \
            foo: function-exists(exists);\n      \
            foo: function-exists(\"exists\"); \n      \
            foo: function-exists(f);\n      \
            foo: function-exists(\"f\"); \n      \
            foo: function-exists(g);\n      \
            foo: function-exists(\"g\"); \n      \
            foo: function-exists(nope);\n      \
            foo: function-exists(\"nope\"); \n      \
            foo: g();\n      foo: f();\n      foo: h();\n    }\n  }\n\n}\n",
        "div {\n  foo: true;\n  foo: true;\n  foo: true;\n  foo: true;\n  \
         foo: true;\n  foo: true;\n  foo: true;\n  foo: true;\n  \
         foo: false;\n  foo: false;\n  foo: false;\n  foo: false;\n  \
         foo: true;\n}\n\
         div span {\n  foo: true;\n  foo: true;\n  foo: true;\n  \
         foo: true;\n  foo: true;\n  foo: true;\n  foo: true;\n  \
         foo: true;\n  foo: false;\n  foo: false;\n  foo: false;\n  \
         foo: false;\n  foo: true;\n}\n\
         div span p {\n  foo: true;\n  foo: true;\n  foo: true;\n  \
         foo: true;\n  foo: true;\n  foo: true;\n  foo: true;\n  \
         foo: true;\n  foo: false;\n  foo: false;\n  foo: false;\n  \
         foo: false;\n  foo: true;\n}\n",
    )
}

#[test]
fn t58_mixin_exists() {
    check(
        b"@function exists($name) {\n  @return mixin-exists($name);\n}\n\n\
            @function f() {\n  $foo: hi;\n  @return g();\n}\n\n\
            @function g() {\n  @return mixin-exists(foo);\n}\n\n\
            @function h() {\n  @return mixin-exists(lighten);\n}\n\n\
            @mixin red-text { color: red; }\n\
            @mixin blue-text { color: red; }\n\
            @mixin green-text { color: red; }\n\n\
            div {\n  foo: mixin-exists(red-text); \n  \
            foo: mixin-exists(\"red-text\"); \n  \
            foo: mixin-exists(blue-text); \n  \
            foo: mixin-exists(\"blue-text\"); \n  \
            foo: mixin-exists(green-text);   \n  \
            foo: mixin-exists(\"green-text\");\n  foo: mixin-exists(nope);\n  \
            foo: mixin-exists(\"nope\");\n  \
            foo: g();\n  foo: f();\n  foo: h();\n\n\n  \
            span {\n    foo: mixin-exists(red-text); \n    \
            foo: mixin-exists(\"red-text\"); \n    \
            foo: mixin-exists(blue-text); \n    \
            foo: mixin-exists(\"blue-text\"); \n    \
            foo: mixin-exists(green-text);   \n    \
            foo: mixin-exists(\"green-text\"); \n    \
            foo: mixin-exists(nope);\n    \
            foo: mixin-exists(\"nope\");\n    \
            foo: g();\n    foo: f();\n    foo: h();\n    \
            p {\n      foo: mixin-exists(red-text); \n      \
            foo: mixin-exists(\"red-text\"); \n      \
            foo: mixin-exists(blue-text); \n      \
            foo: mixin-exists(\"blue-text\"); \n      \
            foo: mixin-exists(green-text);   \n      \
            foo: mixin-exists(\"green-text\"); \n      \
            foo: mixin-exists(nope);\n      \
            foo: mixin-exists(\"nope\");\n      \
            foo: g();\n      foo: f();\n      foo: h();\n    }\n  }\n\n}",
        "div {\n  foo: true;\n  foo: true;\n  foo: true;\n  foo: true;\n  \
         foo: true;\n  foo: true;\n  foo: false;\n  foo: false;\n  \
         foo: false;\n  foo: false;\n  foo: false;\n}\n\
         div span {\n  foo: true;\n  foo: true;\n  foo: true;\n  \
         foo: true;\n  foo: true;\n  foo: true;\n  foo: false;\n  \
         foo: false;\n  foo: false;\n  foo: false;\n  foo: false;\n}\n\
         div span p {\n  foo: true;\n  foo: true;\n  foo: true;\n  \
         foo: true;\n  foo: true;\n  foo: true;\n  foo: false;\n  \
         foo: false;\n  foo: false;\n  foo: false;\n  foo: false;\n}\n",
    )
}

#[test]
fn t59_if_expression() {
    check(
        b"$x: 0;\n$if-false: whatever;\n\n\
            div {\n  \
            foo: if($if-true: hey, $if-false: ho, $condition: true);\n  \
            foo: if($if-true: hey, $if-false: ho, $condition: false);\n  \
            foo: if($x != 0, if($x, true, false), unquote(\"x is zero\"));\n  \
            foo: if(false, 1/0, $if-false: $if-false);\n}",
        "div {\n  \
         foo: hey;\n  foo: ho;\n  foo: x is zero;\n  foo: whatever;\n}\n",
    )
}

#[test]
fn t60_call_3_5() {
    check(
        b"@function foobar() {\n  @return foobar;\n}\n\n\
            @function fudge($str) {\n  @return \"assets/fudge/\" + $str;\n}\n\n\
            body {\n  display: call(foobar); \n  \
            display: call(min, 1,3,5,7);\n  display: call(min, 5);\n  \
            display: call(max, 10,3,5,7);\n  color: fudge(\"blah\");\n}",
        "body {\n  display: foobar;\n  display: 1;\n  display: 5;\n  \
         display: 10;\n  color: \"assets/fudge/blah\";\n}\n",
    )
}

/// No proper spec-test for str-slice, this is from
/// `spec/libsass-closed-issues/issue_760/input.scss`
#[test]
fn ti815_str_slice() {
    check(
        b"foo {\n  foo: str-slice(\"bar\", 1, 2);\n  \
            bar: str-slice(\"bar\", 3);\n}\n",
        "foo {\n  foo: \"ba\";\n  bar: \"r\";\n}\n",
    )
}

/// From `spec/libsass-closed-issues/issue_574`
#[test]
fn ti574_map_trailing_comma() {
    check(
        b"$flow: left;\n\n\
          $map: (\n  margin-#{$flow}: 3em,\n  foo: bar,\n);\n\n\
          .test {\n  margin-left: map-get($map, margin-left);\n}\n",
        ".test {\n  margin-left: 3em;\n}\n",
    )
}

/// From `spec/core_functions/invert/weight-parameter`
#[test]
fn weight_parameter() {
    check(
        b".invert-with-weight {\n  zero-percent: invert(#edc, 0%);\n  \
          ten-percent: invert(#edc, 10%);\n  \
          keyword: invert(#edc, $weight: 10%);\n  \
          one-hundred-percent: invert(#edc, 100%);\n}\n",
        ".invert-with-weight {\n  zero-percent: #eeddcc;\n  \
         ten-percent: #d8cabd;\n  keyword: #d8cabd;\n  \
         one-hundred-percent: #112233;\n}\n",
    )
}

/// From `spec/core_functions/rgba/success`
#[test]
fn rgba_success() {
    check(
        b"a {\n  calc-1: rgba(calc(1), 2, 3, 0.4);\n  \
          calc-2: rgba(1, calc(2), 3, 0.4);\n  \
          calc-3: rgba(1, 2, calc(3), 0.4);\n  \
          calc-4: rgba(1, 2, 3, calc(0.4));\n\n  \
          var-1: rgba(var(--foo), 2, 3, 0.4);\n  \
          var-2: rgba(1, var(--foo), 3, 0.4);\n  \
          var-3: rgba(1, 2, var(--foo), 0.4);\n  \
          var-4: rgba(1, 2, 3, var(0.4));\n\n  \
          calc-2-args: rgba(blue, calc(0.4));\n  \
          var-2-args-alpha: rgba(blue, var(0.4));\n  \
          var-2-args-color: rgba(var(--foo), 0.4);\n  \
          var-2-args-both: rgba(var(--foo), var(0.4));\n}\n",
        "a {\n  calc-1: rgba(calc(1), 2, 3, 0.4);\n  \
         calc-2: rgba(1, calc(2), 3, 0.4);\n  \
         calc-3: rgba(1, 2, calc(3), 0.4);\n  \
         calc-4: rgba(1, 2, 3, calc(0.4));\n  \
         var-1: rgba(var(--foo), 2, 3, 0.4);\n  \
         var-2: rgba(1, var(--foo), 3, 0.4);\n  \
         var-3: rgba(1, 2, var(--foo), 0.4);\n  \
         var-4: rgba(1, 2, 3, var(0.4));\n  \
         calc-2-args: rgba(0, 0, 255, calc(0.4));\n  \
         var-2-args-alpha: rgba(0, 0, 255, var(0.4));\n  \
         var-2-args-color: rgba(var(--foo), 0.4);\n  \
         var-2-args-both: rgba(var(--foo), var(0.4));\n}\n",
    )
}

/// From `spec/libsass-closed-issues/issue_1133/normal`
#[test]
fn each_binds_multiple() {
    check(
        b"@function foo($map) {\n    @return $map;\n}\n\n\
          a {\n    $map: foo((this: is, my: map));\n    \
          @each $k, $v in $map {\n        #{$k}: $v;\n    }\n}\n\n\
          b {\n    $map: call(\"foo\", (this: is, my: map));\n    \
          @each $k, $v in $map {\n        #{$k}: $v;\n    }\n}\n",
        "a {\n  this: is;\n  my: map;\n}\n\n\
         b {\n  this: is;\n  my: map;\n}\n",
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
