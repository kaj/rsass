//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/15_arithmetic_and_lists.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
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
