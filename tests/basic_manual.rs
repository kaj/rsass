//! These are from the "basic" directory in the sass specification.
//! See <https://github.com/sass/sass-spec> for source material.
//! See `tests/basic/main.rs` for semi-autoimported tests.
//! This file contains old tests that need special handling.
use rsass::{compile_scss, compile_scss_file, compile_value};

#[test]
fn txx_empty_rule() {
    check(b"foo{}", "")
}

#[test]
fn t14_imports() {
    let path = "tests/basic/14_imports/input.scss";
    assert_eq!(
        compile_scss_file(path.as_ref(), Default::default())
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
fn t33_ambigous_imports() {
    let path = "tests/basic/33_ambiguous_imports/input.scss";
    assert_eq!(
        compile_scss_file(path.as_ref(), Default::default())
            .and_then(|s| Ok(String::from_utf8(s)?))
            .unwrap(),
        "main {\n  color: red;\n}\n\n\
         dir {\n  color: blue;\n}\n\n\
         fudge {\n  color: brown;\n}\n"
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

#[test]
fn test_number_0() {
    check_value("0", "0");
}
#[test]
fn test_number_neg0() {
    check_value("-0", "0");
}
#[test]
fn test_number_1() {
    check_value("1", "1");
}
#[test]
fn test_number_neg1() {
    check_value("-1", "-1");
}
#[test]
fn test_number_nines_a() {
    check_value("0.999", "0.999");
}
#[test]
fn test_number_nines_b() {
    check_value("-0.999", "-0.999");
}
#[test]
fn test_number_nines_c() {
    check_value(".999", ".999");
}
#[test]
fn test_number_nines_d() {
    check_value("-.999", "-.999");
}
#[test]
#[ignore = "Should be fixed by #67"]
fn test_number_nines_e() {
    check_value("0.9999999", "0.9999999");
}
#[test]
#[ignore = "Should be fixed by #67"]
fn test_number_nines_f() {
    check_value("-0.9999999", "-0.9999999");
}
#[test]
#[ignore = "Should be fixed by #67"]
fn test_number_zeroes_a() {
    check_value("0.000000000000000001", "0");
}

fn check_value(input: &str, expected: &str) {
    assert_eq!(
        compile_value(input.as_ref(), Default::default())
            .and_then(|s| Ok(String::from_utf8(s)?))
            .unwrap(),
        expected,
    );
}

fn check(input: &[u8], expected: &str) {
    assert_eq!(
        compile_scss(input, Default::default())
            .and_then(|s| Ok(String::from_utf8(s)?))
            .unwrap(),
        expected
    );
}
