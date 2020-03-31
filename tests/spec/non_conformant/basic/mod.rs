//! Tests auto-converted from "sass-spec/spec/non_conformant/basic"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/non_conformant/basic/00_empty.hrx"
#[test]
fn t00_empty() {
    assert_eq!(
        rsass(
            "\
            \n"
        )
        .unwrap(),
        ""
    );
}

// From "sass-spec/spec/non_conformant/basic/01_simple_css.hrx"
#[test]
fn t01_simple_css() {
    assert_eq!(
        rsass(
            "a {\
            \n  color: blue;\
            \n}"
        )
        .unwrap(),
        "a {\
        \n  color: blue;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/02_simple_nesting.hrx"
#[test]
fn t02_simple_nesting() {
    assert_eq!(
        rsass(
            "div {\
            \n  img {\
            \n    border: 0px;\
            \n  }\
            \n}"
        )
        .unwrap(),
        "div img {\
        \n  border: 0px;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/03_simple_variable.hrx"
#[test]
fn t03_simple_variable() {
    assert_eq!(
        rsass(
            "$color: red;\
            \n\
            \na {\
            \n  color: $color;\
            \n}"
        )
        .unwrap(),
        "a {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/04_basic_variables.hrx"
#[test]
fn t04_basic_variables() {
    assert_eq!(
        rsass(
            "$color: \"black\";\
            \n$color: red;\
            \n$background: \"blue\";\
            \n\
            \na {\
            \n  color: $color;\
            \n  background: $background;\
            \n}\
            \n\
            \n$y: before;\
            \n\
            \n$x: 1 2 $y;\
            \n\
            \nfoo {\
            \n  a: $x;\
            \n}\
            \n\
            \n$y: after;\
            \n\
            \nfoo {\
            \n  a: $x;\
            \n}"
        )
        .unwrap(),
        "a {\
        \n  color: red;\
        \n  background: \"blue\";\
        \n}\
        \nfoo {\
        \n  a: 1 2 before;\
        \n}\
        \nfoo {\
        \n  a: 1 2 before;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/05_empty_levels.hrx"
#[test]
fn t05_empty_levels() {
    assert_eq!(
        rsass(
            "div {\
            \n  span {\
            \n    color: red;\
            \n    background: blue;\
            \n  }\
            \n}\
            \n\
            \ndiv {\
            \n  color: gray;\
            \n  empty {\
            \n    span {\
            \n      color: red;\
            \n      background: blue;\
            \n    }\
            \n  }\
            \n}\
            \n\
            \nempty1 {\
            \n  empty2 {\
            \n    div {\
            \n      blah: blah;\
            \n    }\
            \n  }\
            \n}\
            \n\
            \nempty1 {\
            \n  empty2 {\
            \n    div {\
            \n      bloo: blee;\
            \n      empty3 {\
            \n        span {\
            \n          blah: blah;\
            \n          blah: blah;\
            \n        }\
            \n      }\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "div span {\
        \n  color: red;\
        \n  background: blue;\
        \n}\
        \ndiv {\
        \n  color: gray;\
        \n}\
        \ndiv empty span {\
        \n  color: red;\
        \n  background: blue;\
        \n}\
        \nempty1 empty2 div {\
        \n  blah: blah;\
        \n}\
        \nempty1 empty2 div {\
        \n  bloo: blee;\
        \n}\
        \nempty1 empty2 div empty3 span {\
        \n  blah: blah;\
        \n  blah: blah;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/06_nesting_and_comments.hrx"
#[test]
fn t06_nesting_and_comments() {
    assert_eq!(
        rsass(
            "$blah: bloo blee;\
            \n$blip: \"a \'red\' and \\\"blue\\\" value\";\
            \n\
            \n/* top level comment -- should be preserved */\
            \ndiv {\
            \n  /* another comment that should be preserved */\
            \n  color: red;\
            \n  background: blue;\
            \n  $blux: hux; // gone!\
            \n  span {\
            \n    font-weight: bold;\
            \n    a {\
            \n      text-decoration: none; /* where will this comment go? */\
            \n      color: green;\
            \n      /* what about this comment? */ border: 1px $blah red;\
            \n    }\
            \n    /* yet another comment that should be preserved */\
            \n    display: inline-block;\
            \n  }  // gone!\
            \n  /* the next selector should be indented two spaces */\
            \n  empty {\
            \n    not_empty {\
            \n      blah: blah; // gone!\
            \n      bloo: bloo;\
            \n    }\
            \n  }\
            \n  p {\
            \n    padding: 10px 8%;\
            \n    -webkit-box-sizing: $blux;\
            \n  }\
            \n  margin: 10px 5px;\
            \n  h1 {\
            \n    color: $blip;\
            \n  }\
            \n}\
            \n/* last comment, top level again --\
            \n   compare the indentation! */\
            \n   \
            \ndiv {\
            \n  f: g;\
            \n  empty {\
            \n    span {\
            \n      a: b;\
            \n    }\
            \n  }\
            \n  empty_with_comment {\
            \n    /* hey now */\
            \n    span {\
            \n      c: d;\
            \n    }\
            \n  }\
            \n}"
        )
        .unwrap(),
        "/* top level comment -- should be preserved */\
        \ndiv {\
        \n  /* another comment that should be preserved */\
        \n  color: red;\
        \n  background: blue;\
        \n  /* the next selector should be indented two spaces */\
        \n  margin: 10px 5px;\
        \n}\
        \ndiv span {\
        \n  font-weight: bold;\
        \n  /* yet another comment that should be preserved */\
        \n  display: inline-block;\
        \n}\
        \ndiv span a {\
        \n  text-decoration: none;\
        \n  /* where will this comment go? */\
        \n  color: green;\
        \n  /* what about this comment? */\
        \n  border: 1px bloo blee red;\
        \n}\
        \ndiv empty not_empty {\
        \n  blah: blah;\
        \n  bloo: bloo;\
        \n}\
        \ndiv p {\
        \n  padding: 10px 8%;\
        \n  -webkit-box-sizing: hux;\
        \n}\
        \ndiv h1 {\
        \n  color: \"a \'red\' and \\\"blue\\\" value\";\
        \n}\
        \n/* last comment, top level again --\
        \n   compare the indentation! */\
        \ndiv {\
        \n  f: g;\
        \n}\
        \ndiv empty span {\
        \n  a: b;\
        \n}\
        \ndiv empty_with_comment {\
        \n  /* hey now */\
        \n}\
        \ndiv empty_with_comment span {\
        \n  c: d;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/07_nested_simple_selector_groups.hrx"
#[test]
fn t07_nested_simple_selector_groups() {
    assert_eq!(
        rsass(
            "a, b {\
            \n  color: red;\
            \n  background: blue;\
            \n}\
            \n\
            \nc, d {\
            \n  color: gray;\
            \n  e, f {\
            \n    background: blue;\
            \n    padding: 10px 5px;\
            \n  }\
            \n  g, h {\
            \n    blah: blah;\
            \n    bloo: bloo;\
            \n  }\
            \n  i, j {\
            \n    foo: goo;\
            \n    k, l {\
            \n      m, n, o {\
            \n        wow: we are far inside;\
            \n        but: it still works;\
            \n      }\
            \n      hoo: boo;\
            \n    }\
            \n  }\
            \n}"
        )
        .unwrap(),
        "a, b {\
        \n  color: red;\
        \n  background: blue;\
        \n}\
        \nc, d {\
        \n  color: gray;\
        \n}\
        \nc e, c f, d e, d f {\
        \n  background: blue;\
        \n  padding: 10px 5px;\
        \n}\
        \nc g, c h, d g, d h {\
        \n  blah: blah;\
        \n  bloo: bloo;\
        \n}\
        \nc i, c j, d i, d j {\
        \n  foo: goo;\
        \n}\
        \nc i k, c i l, c j k, c j l, d i k, d i l, d j k, d j l {\
        \n  hoo: boo;\
        \n}\
        \nc i k m, c i k n, c i k o, c i l m, c i l n, c i l o, c j k m, c j k n, c j k o, c j l m, c j l n, c j l o, d i k m, d i k n, d i k o, d i l m, d i l n, d i l o, d j k m, d j k n, d j k o, d j l m, d j l n, d j l o {\
        \n  wow: we are far inside;\
        \n  but: it still works;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/08_selector_combinators.hrx"
#[test]
fn t08_selector_combinators() {
    assert_eq!(
        rsass(
            "a   +   b  >  c {\
            \n  d e {\
            \n    color: blue;\
            \n    background: white;\
            \n  }\
            \n  color: red;\
            \n  background: gray;\
            \n}"
        )
        .unwrap(),
        "a + b > c {\
        \n  color: red;\
        \n  background: gray;\
        \n}\
        \na + b > c d e {\
        \n  color: blue;\
        \n  background: white;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/09_selector_groups_and_combinators.hrx"
#[test]
fn t09_selector_groups_and_combinators() {
    assert_eq!(
        rsass(
            "a + b, c {\
            \n  blah: blah;\
            \n  bleh: bleh;\
            \n  d e, f ~ g + h, > i {\
            \n    bloo: bloo;\
            \n    blee: blee;\
            \n  }\
            \n}"
        )
        .unwrap(),
        "a + b, c {\
        \n  blah: blah;\
        \n  bleh: bleh;\
        \n}\
        \na + b d e, a + b f ~ g + h, a + b > i, c d e, c f ~ g + h, c > i {\
        \n  bloo: bloo;\
        \n  blee: blee;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/10_classes_and_ids.hrx"
#[test]
fn t10_classes_and_ids() {
    assert_eq!(
        rsass(
            "a + b, .class {\
            \n  blah: blah;\
            \n  bleh: bleh;\
            \n  d #id, f ~ g.other + h, > i#grar {\
            \n    bloo: bloo;\
            \n    blee: blee;\
            \n  }\
            \n}"
        )
        .unwrap(),
        "a + b, .class {\
        \n  blah: blah;\
        \n  bleh: bleh;\
        \n}\
        \na + b d #id, a + b f ~ g.other + h, a + b > i#grar, .class d #id, .class f ~ g.other + h, .class > i#grar {\
        \n  bloo: bloo;\
        \n  blee: blee;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/11_attribute_selectors.hrx"
#[test]
fn t11_attribute_selectors() {
    assert_eq!(
        rsass(
            "[hey  =  \'ho\'], a > b {\
            \n  blah: blah;\
            \n  c, [hoo *=    \"ha\" ] {\
            \n    bloo: bloo;\
            \n  }\
            \n}"
        )
        .unwrap(),
        "[hey=ho], a > b {\
        \n  blah: blah;\
        \n}\
        \n[hey=ho] c, [hey=ho] [hoo*=ha], a > b c, a > b [hoo*=ha] {\
        \n  bloo: bloo;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/12_pseudo_classes_and_elements.hrx"
#[test]
fn t12_pseudo_classes_and_elements() {
    assert_eq!(
        rsass(
            "a b {\
            \n  color: red;\
            \n  :first-child, :nth-of-type(  -2n+1 ) {\
            \n    .foo#bar:nth-child(even) {\
            \n      hoo: goo;\
            \n    }\
            \n    blah: bloo;\
            \n    ::after {\
            \n      content: \"glux\";\
            \n      color: green;\
            \n    }\
            \n    :not(.foo) {\
            \n      hoo: boo;\
            \n    }\
            \n    a { b: c; }\
            \n  }\
            \n}"
        )
        .unwrap(),
        "a b {\
        \n  color: red;\
        \n}\
        \na b :first-child, a b :nth-of-type(-2n+1) {\
        \n  blah: bloo;\
        \n}\
        \na b :first-child .foo#bar:nth-child(even), a b :nth-of-type(-2n+1) .foo#bar:nth-child(even) {\
        \n  hoo: goo;\
        \n}\
        \na b :first-child ::after, a b :nth-of-type(-2n+1) ::after {\
        \n  content: \"glux\";\
        \n  color: green;\
        \n}\
        \na b :first-child :not(.foo), a b :nth-of-type(-2n+1) :not(.foo) {\
        \n  hoo: boo;\
        \n}\
        \na b :first-child a, a b :nth-of-type(-2n+1) a {\
        \n  b: c;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/13_back_references.hrx"
#[test]
fn t13_back_references() {
    assert_eq!(
        rsass(
            "hey, ho {\
            \n  & > boo, foo &.goo {\
            \n    bloo: bloo;\
            \n  }\
            \n  blah: blah;\
            \n}"
        )
        .unwrap(),
        "hey, ho {\
        \n  blah: blah;\
        \n}\
        \nhey > boo, foo hey.goo, ho > boo, foo ho.goo {\
        \n  bloo: bloo;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/14_imports.hrx"
#[test]
#[ignore] // wrong result
fn t14_imports() {
    assert_eq!(
        rsass(
            "@import \"a.scss\";\
            \n\
            \nfoo {\
            \n  blah: blah;\
            \n  goo {\
            \n    blee: blee;\
            \n    @import \"../14_imports/b.scss\";\
            \n    hello: world;\
            \n  }\
            \n  @import \"sub/c.scss\";\
            \n}"
        )
        .unwrap(),
        "div span {\
        \n  moo: goo;\
        \n}\
        \nfoo {\
        \n  blah: blah;\
        \n}\
        \nfoo goo {\
        \n  blee: blee;\
        \n  hello: world;\
        \n}\
        \nfoo goo hoo {\
        \n  mux: scooba-dee-doo;\
        \n  flux: gooboo boo;\
        \n}\
        \nfoo goo hoo d {\
        \n  inside: d now;\
        \n}\
        \nfoo blux {\
        \n  hey: another thing;\
        \n  ho: will this work;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/15_arithmetic_and_lists.hrx"
#[test]
#[ignore] // wrong result
fn t15_arithmetic_and_lists() {
    assert_eq!(
        rsass(
            "$stuff: 1 2 3;\
            \n\
            \n$three: 3;\
            \n\
            \ndiv {\
            \n  a: 1 + 2;\
            \n  b: 3 + 3/4;\
            \n  c: 1/2 + 1/2;\
            \n  /* shouldn\'t eval the following \"300\" */\
            \n  d: 300;\
            \n  /* increasingly jacked-up edge cases that combine arithmetic with lists */\
            \n  e: 1 + (5/10 2 3);\
            \n  f: 1 + ((2+(3 4) 5) 6);\
            \n  g: 1 + ((1+(14/7 8) 9) 6);\
            \n  /* shouldn\'t perform the following division */\
            \n  h: 15 / 3 / 5;\
            \n  /* should perform the following division now */\
            \n  i: (15 / 3 / 5);\
            \n  /* this too */\
            \n  j: (15 / 3) / 5;\
            \n  /* and this */\
            \n  k: 15 / $three;\
            \n  l: 15 / 5 / $three;\
            \n  m: 1/2, $stuff url(\"www.foo.com/blah.png\") blah blah;\
            \n  n: 1 2 3, $stuff 4 5 (6, 7 8 9);\
            \n  o: 3px + 3px + 3px;\
            \n  p: 4 + 1px;\
            \n  q: (20pt / 10pt);\
            \n  r: 16em * 4;\
            \n  s: (5em / 2);\
            \n  t: 1 + (2 + (3/4 + (4/5 6/7)));\
            \n\
            \n  // Arithmetic operations in Sass should never fail, since we implicitly use\
            \n  // floating-point for all numbers.\
            \n  one-over-zero: (1 / 0);\
            \n  zero-over-zero: (0 / 0);\
            \n  one-mod-zero: 1 % 0;\
            \n  zero-mod-zero: 0 % 0;\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  a: 3;\
        \n  b: 3.75;\
        \n  c: 1;\
        \n  /* shouldn\'t eval the following \"300\" */\
        \n  d: 300;\
        \n  /* increasingly jacked-up edge cases that combine arithmetic with lists */\
        \n  e: 15/10 2 3;\
        \n  f: 123 4 5 6;\
        \n  g: 1114/7 8 9 6;\
        \n  /* shouldn\'t perform the following division */\
        \n  h: 15/3/5;\
        \n  /* should perform the following division now */\
        \n  i: 1;\
        \n  /* this too */\
        \n  j: 1;\
        \n  /* and this */\
        \n  k: 5;\
        \n  l: 1;\
        \n  m: 1/2, 1 2 3 url(\"www.foo.com/blah.png\") blah blah;\
        \n  n: 1 2 3, 1 2 3 4 5 6, 7 8 9;\
        \n  o: 9px;\
        \n  p: 5px;\
        \n  q: 2;\
        \n  r: 64em;\
        \n  s: 2.5em;\
        \n  t: 120.754/5 6/7;\
        \n  one-over-zero: Infinity;\
        \n  zero-over-zero: NaN;\
        \n  one-mod-zero: NaN;\
        \n  zero-mod-zero: NaN;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/17_basic_mixins.hrx"
#[test]
fn t17_basic_mixins() {
    assert_eq!(
        rsass(
            "@mixin foo($x, $y) {\
            \n  hugabug: $y $x;\
            \n}\
            \n\
            \n@mixin bar($a, $b: flug) {\
            \n  flugablug: $a $b glug;\
            \n}\
            \n\
            \n@mixin hux() {\
            \n  no: parameters here;\
            \n  div, span {\
            \n    some: nested stuff;\
            \n    foo, bar {\
            \n      more: stuff so forth;\
            \n      blah: blah;\
            \n    }\
            \n  }\
            \n  /* end of hux */\
            \n}\
            \n\
            \na {\
            \n  hey: ho;\
            \n  @include foo(second, third);\
            \n  @include foo($y: kwd-y, $x: kwd-x);\
            \n  goo: boo hoo;\
            \n  @include hux;\
            \n  @include bar(pug);\
            \n  @include bar(pug, mug);\
            \n}\
            \n\
            \n\
            \n$x: from a variable;\
            \n\
            \ndiv {\
            \n  blah: blah $x blah;\
            \n}"
        )
        .unwrap(),
        "a {\
        \n  hey: ho;\
        \n  hugabug: third second;\
        \n  hugabug: kwd-y kwd-x;\
        \n  goo: boo hoo;\
        \n  no: parameters here;\
        \n  /* end of hux */\
        \n  flugablug: pug flug glug;\
        \n  flugablug: pug mug glug;\
        \n}\
        \na div, a span {\
        \n  some: nested stuff;\
        \n}\
        \na div foo, a div bar, a span foo, a span bar {\
        \n  more: stuff so forth;\
        \n  blah: blah;\
        \n}\
        \ndiv {\
        \n  blah: blah from a variable blah;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/18_mixin_scope.hrx"
#[test]
fn t18_mixin_scope() {
    assert_eq!(
        rsass(
            "$x: global x;\
            \n$y: global y;\
            \n\
            \n@mixin foo($x) {\
            \n  f-a: $x;\
            \n  f-b: $y;\
            \n  $x: local x changed by foo;\
            \n  $y: global y changed by foo !global;\
            \n  $z: new local z;\
            \n  f-a: $x;\
            \n  f-b: $y;\
            \n  f-c: $z;\
            \n}\
            \n\
            \ndiv {\
            \n  a: $x;\
            \n  b: $y;\
            \n  @include foo(arg);\
            \n  a: $x;\
            \n  b: $y;\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  a: global x;\
        \n  b: global y;\
        \n  f-a: arg;\
        \n  f-b: global y;\
        \n  f-a: local x changed by foo;\
        \n  f-b: global y changed by foo;\
        \n  f-c: new local z;\
        \n  a: global x;\
        \n  b: global y changed by foo;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/19_full_mixin_craziness.hrx"
#[test]
fn t19_full_mixin_craziness() {
    assert_eq!(
        rsass(
            "$x: global-x;\
            \n$y: global-y;\
            \n$z: global-z;\
            \n\
            \n@mixin foo($x, $y) {\
            \n  /* begin foo */\
            \n  margin: $x $y;\
            \n  blip {\
            \n    hey: now;\
            \n  }\
            \n  /* end foo */\
            \n}\
            \n\
            \n@mixin foogoo($x, $y, $z) {\
            \n  margin: $x $y $z;\
            \n}\
            \n\
            \n@mixin hux($y) {\
            \n  /* begin hux */\
            \n  color: $y;\
            \n  @include foo(called-from-hux, $y: $y);\
            \n  /* end hux */\
            \n}\
            \n\
            \ndiv {\
            \n  @include foo(1, 2);\
            \n  @include foo(1, 3);\
            \n  @include foogoo(1, 2, $z: zee);\
            \n  @include foogoo(1, $y /* blah */ : kwd-y, $z: kwd-z);\
            \n}\
            \n\
            \ndiv {\
            \n  @include hux($y: $y);\
            \n}\
            \n\
            \n$y: different-global-y;\
            \n\
            \ndiv {\
            \n  @include hux(calling-hux-again);\
            \n}\
            \n\
            \n@mixin bung() {\
            \n  blah: original-bung;\
            \n}\
            \n\
            \ndiv {\
            \n  @include bung();\
            \n}\
            \n\
            \n@mixin bung() {\
            \n  blah: redefined-bung;\
            \n}\
            \n\
            \ndiv {\
            \n  @include bung();\
            \n}\
            \n\
            \ndiv {\
            \n  /* calls to nullary mixins may omit the empty argument list */\
            \n  @include bung;\
            \n}\
            \n\
            \ndiv {\
            \n  @include foo($x: kwdarg1, $y: kwdarg2);\
            \n}\
            \n\
            \n@mixin ruleset() {\
            \n  hoo {\
            \n    color: boo;\
            \n  }\
            \n}\
            \n\
            \n@include ruleset();\
            \n\
            \n$da: default argument;\
            \n\
            \n@mixin default_args($x, $y: $da) {\
            \n  blah: $x $y;\
            \n}\
            \n$da: some other default;\
            \n\
            \ndiv {\
            \n  @include default_args(boogoo);\
            \n}\
            \n\
            \n@mixin original() {\
            \n  value: original;\
            \n}\
            \n\
            \ndiv {\
            \n  @include original();\
            \n}\
            \n\
            \n@mixin original() {\
            \n  value: no longer original;\
            \n}\
            \n\
            \ndiv {\
            \n  @include original();\
            \n}\
            \n\
            \n@mixin set-x($x) {\
            \n  $x: changed local x;\
            \n  arg: $x;\
            \n  $y: changed global y !global;\
            \n  blarg: $y;\
            \n}\
            \n\
            \ndiv {\
            \n  @include set-x(blah);\
            \n  a: $x;\
            \n  b: $y;\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  /* begin foo */\
        \n  margin: 1 2;\
        \n  /* end foo */\
        \n  /* begin foo */\
        \n  margin: 1 3;\
        \n  /* end foo */\
        \n  margin: 1 2 zee;\
        \n  margin: 1 kwd-y kwd-z;\
        \n}\
        \ndiv blip {\
        \n  hey: now;\
        \n}\
        \ndiv blip {\
        \n  hey: now;\
        \n}\
        \ndiv {\
        \n  /* begin hux */\
        \n  color: global-y;\
        \n  /* begin foo */\
        \n  margin: called-from-hux global-y;\
        \n  /* end foo */\
        \n  /* end hux */\
        \n}\
        \ndiv blip {\
        \n  hey: now;\
        \n}\
        \ndiv {\
        \n  /* begin hux */\
        \n  color: calling-hux-again;\
        \n  /* begin foo */\
        \n  margin: called-from-hux calling-hux-again;\
        \n  /* end foo */\
        \n  /* end hux */\
        \n}\
        \ndiv blip {\
        \n  hey: now;\
        \n}\
        \ndiv {\
        \n  blah: original-bung;\
        \n}\
        \ndiv {\
        \n  blah: redefined-bung;\
        \n}\
        \ndiv {\
        \n  /* calls to nullary mixins may omit the empty argument list */\
        \n  blah: redefined-bung;\
        \n}\
        \ndiv {\
        \n  /* begin foo */\
        \n  margin: kwdarg1 kwdarg2;\
        \n  /* end foo */\
        \n}\
        \ndiv blip {\
        \n  hey: now;\
        \n}\
        \nhoo {\
        \n  color: boo;\
        \n}\
        \ndiv {\
        \n  blah: boogoo some other default;\
        \n}\
        \ndiv {\
        \n  value: original;\
        \n}\
        \ndiv {\
        \n  value: no longer original;\
        \n}\
        \ndiv {\
        \n  arg: changed local x;\
        \n  blarg: changed global y;\
        \n  a: global-x;\
        \n  b: changed global y;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/20_scoped_variables.hrx"
#[test]
fn t20_scoped_variables() {
    assert_eq!(
        rsass(
            "@mixin foo() {\
            \n  /* begin foo */\
            \n  /* assigning to $x */\
            \n  $x: inside foo;\
            \n  x: $x;\
            \n  /* end foo */\
            \n}\
            \n\
            \nouter {\
            \n  /* assigning to $x */\
            \n  $x: inside outer scope;\
            \n  blah: blah;\
            \n  inner {\
            \n    @include foo();\
            \n    x: $x;\
            \n  }\
            \n}"
        )
        .unwrap(),
        "outer {\
        \n  /* assigning to $x */\
        \n  blah: blah;\
        \n}\
        \nouter inner {\
        \n  /* begin foo */\
        \n  /* assigning to $x */\
        \n  x: inside foo;\
        \n  /* end foo */\
        \n  x: inside outer scope;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/21_one_builtin_function.hrx"
#[test]
fn t21_one_builtin_function() {
    assert_eq!(
        rsass(
            "div {\
            \n  color: rgb(255, $blue: 0, $green: 255);\
            \n  background: rgb(123, 45, 6);\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  color: yellow;\
        \n  background: #7b2d06;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/22_colors_with_alpha.hrx"
#[test]
fn t22_colors_with_alpha() {
    assert_eq!(
        rsass(
            "$x: rgb(0, 255, 255);\
            \n\
            \ndiv {\
            \n  color: rgb(255, $blue: 0, $green: 255);\
            \n  background: rgb(123, 45, 6);\
            \n  grah: rgba(#f0e, $alpha: .5);\
            \n  blah: rgba(1,2,3,.6);\
            \n  \
            \n  floo: $x;\
            \n  bloo: rgba($x, 0.7);\
            \n  groo: $x;\
            \n  \
            \n  $x: rgb(123, 45, 6);\
            \n  \
            \n  hoo: red($x);\
            \n  moo: green($x);\
            \n  poo: blue($x);\
            \n  \
            \n  goo: mix(rgba(255, 0, 0, 0.5), #00f);\
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
        \n  blah: rgba(1, 2, 3, 0.6);\
        \n  floo: aqua;\
        \n  bloo: rgba(0, 255, 255, 0.7);\
        \n  groo: aqua;\
        \n  hoo: 123;\
        \n  moo: 45;\
        \n  poo: 6;\
        \n  goo: rgba(64, 0, 191, 0.75);\
        \n  boo: #edcba9;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/23_basic_value_interpolation.hrx"
#[test]
fn t23_basic_value_interpolation() {
    assert_eq!(
        rsass(
            "div {\
            \n  a: hello#{world};\
            \n  a: hello #{world};\
            \n  b: 12#{3};\
            \n  b: type-of(12#{3});\
            \n  b: #{12 + 111};\
            \n  b: type-of(#{12 + 111});\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  a: helloworld;\
        \n  a: hello world;\
        \n  b: 12 3;\
        \n  b: list;\
        \n  b: 123;\
        \n  b: string;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/24_namespace_properties.hrx"
#[test]
fn t24_namespace_properties() {
    assert_eq!(
        rsass(
            "div {\
            \n  a: {\
            \n    p1: q;\
            \n    b: {\
            \n      p2: q;\
            \n    }\
            \n    p3: q;\
            \n  }\
            \n}\
            \n\
            \nfoo {\
            \n  bar: baz {\
            \n    bip: bop;\
            \n    bing: type-of(\"hello\");\
            \n    bang: 1 + 2;\
            \n    bung: bap;\
            \n    bong: bup {\
            \n      x: x;\
            \n      y: y;\
            \n      z: z;\
            \n    }\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  a-p1: q;\
        \n  a-b-p2: q;\
        \n  a-p3: q;\
        \n}\
        \nfoo {\
        \n  bar: baz;\
        \n  bar-bip: bop;\
        \n  bar-bing: string;\
        \n  bar-bang: 3;\
        \n  bar-bung: bap;\
        \n  bar-bong: bup;\
        \n  bar-bong-x: x;\
        \n  bar-bong-y: y;\
        \n  bar-bong-z: z;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/25_basic_string_interpolation.hrx"
#[test]
fn t25_basic_string_interpolation() {
    assert_eq!(
        rsass(
            "div {\
            \n  blah: \"hello #{2+2} world #{unit(23px)} #{\'bloo\\n\'} blah\";\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  blah: \"hello 4 world px bloon blah\";\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/26_selector_interpolation.hrx"
#[test]
fn t26_selector_interpolation() {
    assert_eq!(
        rsass(
            "$x: oo, ba;\
            \n$y: az, hu;\
            \n\
            \nf#{$x}r {\
            \n  p: 1;\
            \n  b#{$y}x {\
            \n    q: 2;\
            \n    mumble#{length($x) + length($y)} {\
            \n      r: 3;\
            \n    }\
            \n  }\
            \n}"
        )
        .unwrap(),
        "foo, bar {\
        \n  p: 1;\
        \n}\
        \nfoo baz, foo hux, bar baz, bar hux {\
        \n  q: 2;\
        \n}\
        \nfoo baz mumble4, foo hux mumble4, bar baz mumble4, bar hux mumble4 {\
        \n  r: 3;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/27_media_queries.hrx"
#[test]
fn t27_media_queries() {
    assert_eq!(
        rsass(
            "a b c {\
            \n  blee: blee;\
            \n  d e f {\
            \n    blah: blah;\
            \n    bloo: bloo;\
            \n  }\
            \n  g h, i j {\
            \n    @media print and (foo: 1 2 3), (bar: 3px hux(muz)), not screen {\
            \n      hey: ho;\
            \n      k l m {\
            \n        hee: fee;\
            \n      }\
            \n    }\
            \n  }\
            \n  blah: blah;\
            \n}\
            \n\
            \n\
            \n"
        )
        .unwrap(),
        "a b c {\
        \n  blee: blee;\
        \n  blah: blah;\
        \n}\
        \na b c d e f {\
        \n  blah: blah;\
        \n  bloo: bloo;\
        \n}\
        \n@media print and (foo: 1 2 3), (bar: 3px hux(muz)), not screen {\
        \n  a b c g h, a b c i j {\
        \n    hey: ho;\
        \n  }\
        \n  a b c g h k l m, a b c i j k l m {\
        \n    hee: fee;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/28_url.hrx"
#[test]
fn t28_url() {
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
            \n    bar: url(http://fonts.googleapis.com/css?family=Karla:400,700,400italic|Anonymous+Pro:400,700,400italic);\
            \n  }\
            \n  gloo: url(\"hey#{1+2}.css\");\
            \n  floo: url(hadoop-#{$y+321}.css);\
            \n}\
            \n"
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
        \n  bar: url(http://fonts.googleapis.com/css?family=Karla:400,700,400italic|Anonymous+Pro:400,700,400italic);\
        \n  hux: url(http://box_123////fudgepop.css);\
        \n  foo: url(http://blah.com/bar-2.css);\
        \n  bar: url(http://fonts.googleapis.com/css?family=Karla:400,700,400italic|Anonymous+Pro:400,700,400italic);\
        \n  hux: url(http://box_123////fudgepop.css);\
        \n  foo: url(http://blah.com/bar-3.css);\
        \n  bar: url(http://fonts.googleapis.com/css?family=Karla:400,700,400italic|Anonymous+Pro:400,700,400italic);\
        \n  hux: url(http://box_123////fudgepop.css);\
        \n  foo: url(http://blah.com/bar-4.css);\
        \n  bar: url(http://fonts.googleapis.com/css?family=Karla:400,700,400italic|Anonymous+Pro:400,700,400italic);\
        \n  hux: url(http://box_123////fudgepop.css);\
        \n  foo: url(http://blah.com/bar-5.css);\
        \n  bar: url(http://fonts.googleapis.com/css?family=Karla:400,700,400italic|Anonymous+Pro:400,700,400italic);\
        \n  gloo: url(\"hey3.css\");\
        \n  floo: url(hadoop-444.css);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/29_if.hrx"
#[test]
fn t29_if() {
    assert_eq!(
        rsass(
            "$x: a, b, 1+2;\
            \n\
            \n@if type-of(nth($x, 3)) == number {\
            \n  div {\
            \n    background: gray;\
            \n  }\
            \n}\
            \n\
            \n@if type-of(nth($x, 2)) == number {\
            \n  div {\
            \n    background: gray;\
            \n  }\
            \n}\
            \n@else if type-of(nth($x, 2)) == string {\
            \n  div {\
            \n    background: blue;\
            \n  }\
            \n}\
            \n\
            \n@if type-of(nth($x, 2)) == number {\
            \n  div {\
            \n    background: gray;\
            \n  }\
            \n}\
            \n@else if type-of(nth($x, 2)) == color {\
            \n  div {\
            \n    background: blue;\
            \n  }\
            \n}\
            \n@else {\
            \n  div {\
            \n    background: red;\
            \n  }\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  background: gray;\
        \n}\
        \ndiv {\
        \n  background: blue;\
        \n}\
        \ndiv {\
        \n  background: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/30_if_in_function.hrx"
#[test]
fn t30_if_in_function() {
    assert_eq!(
        rsass(
            "$x: true;\
            \n\
            \n@function foobar() {\
            \n  @if $x {\
            \n    $x: false !global;\
            \n    @return foo;\
            \n  }\
            \n  @else {\
            \n    $x: true !global;\
            \n    @return bar;\
            \n  }\
            \n}\
            \n\
            \ndiv {\
            \n  content: foobar();\
            \n  content: foobar();\
            \n  content: foobar();\
            \n  content: foobar();\
            \n  $x: false !global;\
            \n  content: foobar();\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  content: foo;\
        \n  content: bar;\
        \n  content: foo;\
        \n  content: bar;\
        \n  content: bar;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/31_if_in_mixin.hrx"
#[test]
fn t31_if_in_mixin() {
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

// From "sass-spec/spec/non_conformant/basic/32_percentages.hrx"
#[test]
fn t32_percentages() {
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

// From "sass-spec/spec/non_conformant/basic/33_ambiguous_imports.hrx"
#[test]
#[ignore] // wrong result
fn t33_ambiguous_imports() {
    assert_eq!(
        rsass(
            "main {\
            \n  color: red;\
            \n}\
            \n\
            \n@import \"dir\";"
        )
        .unwrap(),
        "main {\
        \n  color: red;\
        \n}\
        \ndir {\
        \n  color: blue;\
        \n}\
        \nfudge {\
        \n  color: brown;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/35_varargs_false.hrx"
#[test]
fn t35_varargs_false() {
    assert_eq!(
        rsass(
            "@mixin foo($args...) {\
            \n  @each $arg in $args {\
            \n    @if $arg {\
            \n      thing: $arg;\
            \n    }\
            \n  }\
            \n}\
            \n\
            \ndiv {\
            \n  @include foo(a, b, false);\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  thing: a;\
        \n  thing: b;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/36_extra_commas_in_selectors.hrx"
#[test]
fn t36_extra_commas_in_selectors() {
    assert_eq!(
        rsass(
            "div,, , span, ,, {\
            \n  color: red;\
            \n}"
        )
        .unwrap(),
        "div, span {\
        \n  color: red;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/37_url_expressions.hrx"
#[test]
fn t37_url_expressions() {
    assert_eq!(
        rsass(
            "$x: true;\
            \n$file-1x: \"budge.png\";\
            \n\
            \n@function fudge($str) {\
            \n  @return \"assets/fudge/\" + $str;\
            \n}\
            \n\
            \ndiv {\
            \n  blah: url(foo + bar);\
            \n  blah: url(fn(\"s\"));\
            \n  blah: url(if(true, \"red.png\", \"blue.png\"));\
            \n  blah: url(hello-#{world}.png);\
            \n  blah: url(if($x, fudge(\"#{$file-1x}\"), \"#{$file-1x}\"));\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  blah: url(foobar);\
        \n  blah: url(fn(\"s\"));\
        \n  blah: url(\"red.png\");\
        \n  blah: url(hello-world.png);\
        \n  blah: url(\"assets/fudge/budge.png\");\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/38_expressions_in_at_directives.hrx"
#[test]
fn t38_expressions_in_at_directives() {
    assert_eq!(
        rsass(
            "$x: 1;\
            \n$y: 2;\
            \n\
            \n@foo $x $y, hux {\
            \n  bar {\
            \n    whatever: whatever;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "@foo $x $y, hux {\
        \n  bar {\
        \n    whatever: whatever;\
        \n  }\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/39_dash_match_attribute_selector.hrx"
#[test]
fn t39_dash_match_attribute_selector() {
    assert_eq!(
        rsass(
            "div[class|=\"blah\"] {\
            \n  color: blue;\
            \n}"
        )
        .unwrap(),
        "div[class|=blah] {\
        \n  color: blue;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/40_pseudo_class_identifier_starting_with_n.hrx"
#[test]
fn t40_pseudo_class_identifier_starting_with_n() {
    assert_eq!(
        rsass(
            "div:lang(nb) {\
            \n  color: blue;\
            \n}"
        )
        .unwrap(),
        "div:lang(nb) {\
        \n  color: blue;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/41_slashy_urls.hrx"
#[test]
fn t41_slashy_urls() {
    assert_eq!(
        rsass(
            "div {\
            \n  blah: url(//some/absolute/path);\
            \n  blee: url(/*looks-like-a*/comment);\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  blah: url(//some/absolute/path);\
        \n  blee: url(/*looks-like-a*/comment);\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/42_css_imports.hrx"
#[test]
#[ignore] // wrong result
fn t42_css_imports() {
    assert_eq!(
        rsass(
            "div {\
            \n  color: red;\
            \n}\
            \n\
            \n@import \"hux\\ bux.css\";\
            \n@import \"foo.css\";\
            \n\
            \nspan {\
            \n  color: blue;\
            \n}\
            \n\
            \n@import \"bar.css\";"
        )
        .unwrap(),
        "@import \"hux\\ bux.css\";\
        \n@import \"foo.css\";\
        \n@import \"bar.css\";\
        \ndiv {\
        \n  color: red;\
        \n}\
        \nspan {\
        \n  color: blue;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/44_bem_selectors.hrx"
#[test]
fn t44_bem_selectors() {
    assert_eq!(
        rsass(
            "div {\
            \n\
            \n  &_foo {\
            \n    blah: blah;\
            \n  }\
            \n  &--modifier {\
            \n    blach: blah;\
            \n  }\
            \n  &hux {\
            \n    blah: blah;\
            \n  }\
            \n  &div.foo#bar[hux] {\
            \n    blah: blah;\
            \n  }\
            \n\
            \n}"
        )
        .unwrap(),
        "div_foo {\
        \n  blah: blah;\
        \n}\
        \ndiv--modifier {\
        \n  blach: blah;\
        \n}\
        \ndivhux {\
        \n  blah: blah;\
        \n}\
        \ndivdiv.foo#bar[hux] {\
        \n  blah: blah;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/49_interpolants_in_css_imports.hrx"
#[test]
fn t49_interpolants_in_css_imports() {
    assert_eq!(
        rsass(
            "$google-protocol: \"http\"; // choose http or https\
            \n$google-webfont: \"Open+Sans:400italic,700italic,400,700|Oswald\"; // pull string after ?family= from step 3\
            \n\
            \n@import url(\"#{$google-protocol}://fonts.googleapis.com/css?family=#{$google-webfont}\");"
        )
        .unwrap(),
        "@import url(\"http://fonts.googleapis.com/css?family=Open+Sans:400italic,700italic,400,700|Oswald\");\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/50_wrapped_pseudo_selectors.hrx"
#[test]
fn t50_wrapped_pseudo_selectors() {
    assert_eq!(
        rsass(
            "div {\
            \n  :-moz-any(ol p.blah, ul, menu, dir) :-moz-any(ol span + h1, ul, menu, dir) ul {\
            \n    list-style-type: square;\
            \n  }\
            \n  :-moz-any(ol span + h1, ul, menu, dir) ul {\
            \n    list-style-type: square;\
            \n  }\
            \n  :foo(p div) {\
            \n    hi: hi;\
            \n  }\
            \n  :foo(ol) {\
            \n    hi: hi;\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "div :-moz-any(ol p.blah, ul, menu, dir) :-moz-any(ol span + h1, ul, menu, dir) ul {\
        \n  list-style-type: square;\
        \n}\
        \ndiv :-moz-any(ol span + h1, ul, menu, dir) ul {\
        \n  list-style-type: square;\
        \n}\
        \ndiv :foo(p div) {\
        \n  hi: hi;\
        \n}\
        \ndiv :foo(ol) {\
        \n  hi: hi;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/51_trailing_commas_in_list.hrx"
#[test]
fn t51_trailing_commas_in_list() {
    assert_eq!(
        rsass(
            "$mylist: (alpha, beta, gamma, );\
            \n$my-single-item-list: (alpha,);\
            \n.test { \
            \n  out1: length($mylist);\
            \n  blah: type-of(nth($mylist,3));\
            \n  out: length($my-single-item-list); \
            \n}"
        )
        .unwrap(),
        ".test {\
        \n  out1: 3;\
        \n  blah: string;\
        \n  out: 1;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/52_interchangeable_hyphens_underscores.hrx"
#[test]
fn t52_interchangeable_hyphens_underscores() {
    assert_eq!(
        rsass(
            "$my-cool-var: \"hello\";\
            \n@mixin my-cool-mixin($yada-yada) {\
            \n  blah: blah;\
            \n  hi: $yada_yada;\
            \n}\
            \n@function my-cool-function($cool_arg) {\
            \n  @return $cool-arg;\
            \n}\
            \n\
            \ndiv {\
            \n  @include my_cool-mixin($yada_yada: \"hi\");\
            \n  @include my_cool-mixin($my_cool-var);\
            \n  foo: my-cool_function($cool-arg: \"boop\");\
            \n  foo: my-cool_function($my-cool_var);\
            \n  bar: $my_cool_var;\
            \n}\
            \n\
            \n@each $my_cool_var in a, b, c {\
            \n  div {\
            \n    color: $my-cool-var;\
            \n  }\
            \n}\
            \n\
            \n@for $my_cool_var from 1 to 10 {\
            \n  div {\
            \n    color: $my-cool-var;\
            \n  }\
            \n}\
            \n\
            \n\
            \n@function blah_blah() {\
            \n  @return blah;\
            \n}\
            \n\
            \ndiv {\
            \n  foo: blah-blah();\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  blah: blah;\
        \n  hi: \"hi\";\
        \n  blah: blah;\
        \n  hi: \"hello\";\
        \n  foo: \"boop\";\
        \n  foo: \"hello\";\
        \n  bar: \"hello\";\
        \n}\
        \ndiv {\
        \n  color: a;\
        \n}\
        \ndiv {\
        \n  color: b;\
        \n}\
        \ndiv {\
        \n  color: c;\
        \n}\
        \ndiv {\
        \n  color: 1;\
        \n}\
        \ndiv {\
        \n  color: 2;\
        \n}\
        \ndiv {\
        \n  color: 3;\
        \n}\
        \ndiv {\
        \n  color: 4;\
        \n}\
        \ndiv {\
        \n  color: 5;\
        \n}\
        \ndiv {\
        \n  color: 6;\
        \n}\
        \ndiv {\
        \n  color: 7;\
        \n}\
        \ndiv {\
        \n  color: 8;\
        \n}\
        \ndiv {\
        \n  color: 9;\
        \n}\
        \ndiv {\
        \n  foo: blah;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/53_escaped_quotes"
#[test]
#[ignore] // wrong result
fn t53_escaped_quotes() {
    assert_eq!(
        rsass(
            "[data-icon=\'test-1\']:before {\
            \n    content:\'\\\\\';\
            \n}\
            \n\
            \n[data-icon=\'test-2\']:before {\
            \n    content:\'\\\'\';\
            \n}\
            \n\
            \n[data-icon=\'test-3\']:before {\
            \n    content:\"\\\"\";\
            \n}\
            \n\
            \n[data-icon=\'test-4\']:before {\
            \n    content:\'\\\\\';\
            \n}\
            \n\
            \n[data-icon=\'test-5\']:before {\
            \n    content:\'\\\'\';\
            \n}\
            \n\
            \n[data-icon=\'test-6\']:before {\
            \n    content:\"\\\"\";\
            \n}\
            \n\
            \n$open-quote:    «;\
            \n$close-quote:   »;\
            \n\
            \n$open-quote: \\201C;\
            \n$close-quote: \\201D;\
            \n\
            \n.\\E9motion { \
            \nblah: hi; }\
            \n.\\E9 dition { \
            \nblah: hi; }\
            \n.\\0000E9dition { \
            \nblah: hi; }"
        )
        .unwrap(),
        "@charset \"UTF-8\";\
        \n[data-icon=\'test-1\']:before {\
        \n  content: \'\\\\\';\
        \n}\
        \n[data-icon=\'test-2\']:before {\
        \n  content: \'\\\'\';\
        \n}\
        \n[data-icon=\'test-3\']:before {\
        \n  content: \"\\\"\";\
        \n}\
        \n[data-icon=\'test-4\']:before {\
        \n  content: \'\\\\\';\
        \n}\
        \n[data-icon=\'test-5\']:before {\
        \n  content: \'\\\'\';\
        \n}\
        \n[data-icon=\'test-6\']:before {\
        \n  content: \"\\\"\";\
        \n}\
        \n.émotion {\
        \n  blah: hi;\
        \n}\
        \n.édition {\
        \n  blah: hi;\
        \n}\
        \n.édition {\
        \n  blah: hi;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/54_adjacent_identifiers_with_hyphens.hrx"
#[test]
fn t54_adjacent_identifiers_with_hyphens() {
    assert_eq!(
        rsass(
            "input {\
            \n    outline: 5px auto -webkit-focus-ring-color;\
            \n    foo: random -hello-this-is-dog;\
            \n    bar: rando -two-or-more -things-that-start -with-hyphens;\
            \n    baz: foo - bar;\
            \n}"
        )
        .unwrap(),
        "input {\
        \n  outline: 5px auto -webkit-focus-ring-color;\
        \n  foo: random -hello-this-is-dog;\
        \n  bar: rando -two-or-more -things-that-start -with-hyphens;\
        \n  baz: foo-bar;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/55_variable_exists.hrx"
#[test]
fn t55_variable_exists() {
    assert_eq!(
        rsass(
            "@function exists($name) {\
            \n  @return variable-exists($name);\
            \n}\
            \n\
            \n@function f() {\
            \n  $foo: hi;\
            \n  @return g();\
            \n}\
            \n\
            \n@function g() {\
            \n  @return variable-exists(foo);\
            \n}\
            \n\
            \ndiv {\
            \n  foo: variable-exists(x);\
            \n  foo: variable-exists(\"x\");\
            \n\
            \n  span {\
            \n    $x: false;\
            \n\
            \n    foo: variable-exists(x);\
            \n    foo: variable-exists(\"x\");\
            \n    foo: variable-exists(y);\
            \n    foo: variable-exists(\"y\");\
            \n    foo: exists(x);\
            \n    foo: exists(\"x\");\
            \n\
            \n    p {\
            \n      foo: variable-exists(x);\
            \n      foo: variable-exists(\"x\");\
            \n      foo: exists(x);\
            \n      foo: exists(\"x\");\
            \n      foo: variable-exists(y);\
            \n      foo: variable-exists(\"y\");\
            \n      foo: f();\
            \n      $y: blah;\
            \n    }\
            \n  }\
            \n\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  foo: false;\
        \n  foo: false;\
        \n}\
        \ndiv span {\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: false;\
        \n}\
        \ndiv span p {\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: false;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/56_global_variable_exists.hrx"
#[test]
fn t56_global_variable_exists() {
    assert_eq!(
        rsass(
            "@function exists($name) {\
            \n  @return global-variable-exists($name);\
            \n}\
            \n\
            \n@function f() {\
            \n  $foo: hi;\
            \n  @return g();\
            \n}\
            \n\
            \n@function g() {\
            \n  @return global-variable-exists(foo);\
            \n}\
            \n\
            \n$z: hi;\
            \n\
            \ndiv {\
            \n  foo: global-variable-exists(x);    \
            \n  foo: global-variable-exists(\"x\");    \
            \n  foo: global-variable-exists(z);\
            \n  foo: global-variable-exists(\"z\");    \
            \n\
            \n  span {\
            \n    $x: false;\
            \n\
            \n    foo: global-variable-exists(x);\
            \n    foo: global-variable-exists(\"x\");    \
            \n    foo: global-variable-exists(y);\
            \n    foo: global-variable-exists(\"y\");    \
            \n\
            \n    foo: global-variable-exists(z);\
            \n    foo: global-variable-exists(\"z\");    \
            \n\
            \n    p {\
            \n      foo: global-variable-exists(x);\
            \n      foo: global-variable-exists(\"x\");    \
            \n      foo: exists(x);\
            \n      foo: exists(\"x\");    \
            \n      foo: global-variable-exists(z);\
            \n      foo: global-variable-exists(\"z\");    \
            \n      foo: global-variable-exists(y);\
            \n      foo: global-variable-exists(\"y\");    \
            \n      foo: f();\
            \n      $y: blah;\
            \n      //TODO: check for shadowing\
            \n    }\
            \n  }\
            \n\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: true;\
        \n  foo: true;\
        \n}\
        \ndiv span {\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: true;\
        \n  foo: true;\
        \n}\
        \ndiv span p {\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: false;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/57_function_exists.hrx"
#[test]
fn t57_function_exists() {
    assert_eq!(
        rsass(
            "@function exists($name) {\
            \n  @return function-exists($name);\
            \n}\
            \n\
            \n@function f() {\
            \n  $foo: hi;\
            \n  @return g();\
            \n}\
            \n\
            \n@function g() {\
            \n  @return function-exists(foo);\
            \n}\
            \n\
            \n@function h() {\
            \n  @return function-exists(lighten);\
            \n}\
            \n\
            \ndiv {\
            \n  foo: function-exists(lighten); \
            \n  foo: function-exists(\"lighten\"); \
            \n  foo: function-exists(exists);\
            \n  foo: function-exists(\"exists\"); \
            \n  foo: function-exists(f);\
            \n  foo: function-exists(\"f\"); \
            \n  foo: function-exists(g);\
            \n  foo: function-exists(\"g\"); \
            \n  foo: function-exists(nope);\
            \n  foo: function-exists(\"nope\"); \
            \n  foo: g();\
            \n  foo: f();\
            \n  foo: h();\
            \n\
            \n\
            \n  span {\
            \n    foo: function-exists(lighten); \
            \n    foo: function-exists(\"lighten\"); \
            \n    foo: function-exists(exists);\
            \n    foo: function-exists(\"exists\"); \
            \n    foo: function-exists(f);\
            \n    foo: function-exists(\"f\"); \
            \n    foo: function-exists(g);\
            \n    foo: function-exists(\"g\"); \
            \n    foo: function-exists(nope);\
            \n    foo: function-exists(\"nope\"); \
            \n    foo: g();\
            \n    foo: f();\
            \n    foo: h();\
            \n    p {\
            \n      foo: function-exists(lighten); \
            \n      foo: function-exists(\"lighten\"); \
            \n      foo: function-exists(exists);\
            \n      foo: function-exists(\"exists\"); \
            \n      foo: function-exists(f);\
            \n      foo: function-exists(\"f\"); \
            \n      foo: function-exists(g);\
            \n      foo: function-exists(\"g\"); \
            \n      foo: function-exists(nope);\
            \n      foo: function-exists(\"nope\"); \
            \n      foo: g();\
            \n      foo: f();\
            \n      foo: h();\
            \n    }\
            \n  }\
            \n\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: true;\
        \n}\
        \ndiv span {\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: true;\
        \n}\
        \ndiv span p {\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: true;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/58_mixin_exists.hrx"
#[test]
fn t58_mixin_exists() {
    assert_eq!(
        rsass(
            "@function exists($name) {\
            \n  @return mixin-exists($name);\
            \n}\
            \n\
            \n@function f() {\
            \n  $foo: hi;\
            \n  @return g();\
            \n}\
            \n\
            \n@function g() {\
            \n  @return mixin-exists(foo);\
            \n}\
            \n\
            \n@function h() {\
            \n  @return mixin-exists(lighten);\
            \n}\
            \n\
            \n@mixin red-text { color: red; }\
            \n@mixin blue-text { color: red; }\
            \n@mixin green-text { color: red; }\
            \n\
            \ndiv {\
            \n  foo: mixin-exists(red-text); \
            \n  foo: mixin-exists(\"red-text\"); \
            \n  foo: mixin-exists(blue-text); \
            \n  foo: mixin-exists(\"blue-text\"); \
            \n  foo: mixin-exists(green-text);   \
            \n  foo: mixin-exists(\"green-text\"); \
            \n  foo: mixin-exists(nope);\
            \n  foo: mixin-exists(\"nope\");\
            \n  foo: g();\
            \n  foo: f();\
            \n  foo: h();\
            \n\
            \n\
            \n  span {\
            \n    foo: mixin-exists(red-text); \
            \n    foo: mixin-exists(\"red-text\"); \
            \n    foo: mixin-exists(blue-text); \
            \n    foo: mixin-exists(\"blue-text\"); \
            \n    foo: mixin-exists(green-text);   \
            \n    foo: mixin-exists(\"green-text\"); \
            \n    foo: mixin-exists(nope);\
            \n    foo: mixin-exists(\"nope\");\
            \n    foo: g();\
            \n    foo: f();\
            \n    foo: h();\
            \n    p {\
            \n      foo: mixin-exists(red-text); \
            \n      foo: mixin-exists(\"red-text\"); \
            \n      foo: mixin-exists(blue-text); \
            \n      foo: mixin-exists(\"blue-text\"); \
            \n      foo: mixin-exists(green-text);   \
            \n      foo: mixin-exists(\"green-text\"); \
            \n      foo: mixin-exists(nope);\
            \n      foo: mixin-exists(\"nope\");\
            \n      foo: g();\
            \n      foo: f();\
            \n      foo: h();\
            \n    }\
            \n  }\
            \n\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: false;\
        \n}\
        \ndiv span {\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: false;\
        \n}\
        \ndiv span p {\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: true;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: false;\
        \n  foo: false;\
        \n}\
        \n"
    );
}

// From "sass-spec/spec/non_conformant/basic/59_if_expression.hrx"
#[test]
fn t59_if_expression() {
    assert_eq!(
        rsass(
            "$x: 0;\
            \n$if-false: whatever;\
            \n\
            \ndiv {\
            \n  foo: if($if-true: hey, $if-false: ho, $condition: true);\
            \n  foo: if($if-true: hey, $if-false: ho, $condition: false);\
            \n  foo: if($x != 0, if($x, true, false), unquote(\"x is zero\"));\
            \n  foo: if(false, 1/0, $if-false: $if-false);\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  foo: hey;\
        \n  foo: ho;\
        \n  foo: x is zero;\
        \n  foo: whatever;\
        \n}\
        \n"
    );
}
