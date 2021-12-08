//! These are from the "basic" directory in the sass specification.
//! See <https://github.com/sass/sass-spec> for source material.
//! See `tests/basic/main.rs` for semi-autoimported tests.
//! This file contains old tests that need special handling.
use rsass::{compile_scss, compile_scss_path, compile_value};

#[test]
fn txx_empty_rule() {
    check(b"foo{}", "")
}

#[test]
fn use_module_star() {
    check(
        b"@use 'tests/basic/defs' as *;\
          \ndiv {\
          \n  color: $color;\
          \n  col1: foo(1);\
          \n  col2: foo(0);\
          \n  @include myem;
          \n}\n",
        "div {\
         \n  color: purple;\
         \n  col1: purple;\
         \n  col2: pink;\
         \n}\
         \ndiv em {\
         \n  color: pink;\
         \n}\n",
    );
}
#[test]
fn use_module() {
    check(
        b"@use 'tests/basic/defs';\
          \ndiv {\
          \n  color: defs.$color;\
          \n  col1: defs.foo(1);\
          \n  col2: defs.foo(0);\
          \n  @include defs.myem;
          \n}\n",
        "div {\
         \n  color: purple;\
         \n  col1: purple;\
         \n  col2: pink;\
         \n}\
         \ndiv em {\
         \n  color: pink;\
         \n}\n",
    );
}

#[test]
fn t14_imports() {
    let path = "tests/basic/14_imports/input.scss";
    assert_eq!(
        String::from_utf8(
            compile_scss_path(path.as_ref(), Default::default()).unwrap()
        )
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
         h: 15/3/5;\n  \
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
         h: 15/3/5;\n  \
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
        String::from_utf8(
            compile_scss_path(path.as_ref(), Default::default()).unwrap()
        )
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
         calc-2-args: rgba(blue, calc(0.4));\n  \
         var-2-args-alpha: rgba(blue, var(0.4));\n  \
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
fn div_simliar_unit() {
    check(
        b"p { a: (1cm / 1mm); b: (200grad / 360deg); c: (1in / 1pt); }",
        "p {\
         \n  a: 10;\
         \n  b: 0.5;\
         \n  c: 72;\
         \n}\n",
    )
}

#[test]
fn different_numbers_should_compare_as_same() {
    check(
        b"@use 'sass:math' as m;\
          \np {\
          \n  $t: 0.2;
          \n  a: max(1, m.sin(30deg));\
          \n  b: max(0.2, m.sin(30deg));\
          \n  c: max($t, 0.23233234232231232312312323231223323);\
          \n  d: max($t+0.1, 0.23233234232231232312312323231223323);\
          \n}",
        "p {\
         \n  a: 1;\
         \n  b: 0.5;\
         \n  c: 0.2323323423;\
         \n  d: 0.3;\
         \n}\n",
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
    check_value(".9999999999", "0.9999999999");
}
#[test]
fn test_number_nines_d() {
    check_value("-.9999999999", "-0.9999999999");
}
#[test]
fn test_number_nines_e() {
    check_value("0.99999999999", "1");
}
#[test]
fn test_number_nines_f() {
    check_value("-0.99999999999", "-1");
}
#[test]
fn test_number_zeroes_a() {
    check_value("0.000000000000000001", "0");
}
#[test]
fn test_number_zeroes_b() {
    check_value("-0.000000000000000001", "0");
}

/// https://github.com/kaj/rsass/issues/98
#[test]
fn test_rational_overflow_mul() {
    check_value("1.4142135623 * 1.4142135623", "1.9999999998")
}
/// https://github.com/kaj/rsass/issues/98
#[test]
fn test_rational_overflow_div() {
    check_value("1.4142135623 / 1000000000 + 1", "1.0000000014")
}
/// https://github.com/kaj/rsass/issues/98
#[test]
fn test_rational_overflow_add() {
    check_value("4142135623 + 1.4142135623", "4142135624.4142135623")
}
/// https://github.com/kaj/rsass/issues/98
#[test]
fn test_rational_overflow_sub() {
    check_value("4142135623 - 1.4142135623", "4142135621.5857864377")
}

/// https://github.com/kaj/rsass/issues/116
/// map.merge incorrect output for nested map
#[test]
fn issue_116() {
    check(
        b"@use \"sass:map\";\n\
         \n$fonts: (\
         \n  \"Helvetica\": (\
         \n    \"weights\": (\
         \n      \"regular\": 400,\
         \n      \"medium\": 500,\
         \n      \"bold\": 700\
         \n    )\
         \n  )\
         \n);\n\
         \na {\
         \n    color: inspect(map.merge($fonts, \"Helvetica\", \"weights\", \"regular\", (a: 300)));\
         \n}\n",
        "a {\
         \n  color: (\"Helvetica\": (\"weights\": (\"regular\": (a: 300), \"medium\": 500, \"bold\": 700)));\
         \n}\n",
    );
}

/// https://github.com/kaj/rsass/issues/122
/// A division by zero that causes a panic
mod issue_122 {
    use super::check_value;
    // Note: The important thing here is not to panic, the exact
    // output may be changed in the future, maybe to report an error.
    #[test]
    fn reduced() {
        check_value("(#111 + #aaa)/0", "#bbbbbb/0")
    }
    #[test]
    fn reported() {
        check_value(
            "54A444/0+-4444M4#444/-4444/0+-4444M4#444+44/0+444/0+.44444O#444+44/0+4/46",
            "InfinityA444+-4444M4 black/0-4444M4 #444InfinityInfinity0.44444O #444Infinity0.0869565217",
        )
    }
    /// https://github.com/kaj/rsass/issues/121 is very similar.
    #[test]
    fn issue_121() {
        check_value(
            "44A-#444/0+-\0\0\0+44/0+444&",
            "44A-#444/0-\u{1}\u{0}\u{0}\u{0}Infinity444",
        )
    }
}

/// Overflow in gcd for subtraction.
#[test]
fn issue_120() {
    check_value(
        "4444#4444-.4555555555555555555555555",
        "4444 rgba(68, 68, 68, 0.2666666667)",
    )
}

/// Test auto-converted from "sass-spec/spec/libsass/rel.hrx", except one failing unit calculation.
#[test]
fn rel() {
    check(
        b"div {\
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
          \n  eq: (2cm*1in/2cm) == (1in*2cm/2cm);\
          \n  blah: (1cm/1in);\
          \n  in: (1in*2.54cm/1in);\
          \n  lt: 1in < 2.54cm;\
          \n  lt: 2.54cm < 1in;\
          \n  lt: 5 < 4;\
          \n}\n",
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
         \n  eq: true;\
         \n  blah: 0.3937007874;\
         \n  in: 2.54cm;\
         \n  lt: false;\
         \n  lt: false;\
         \n  lt: false;\
         \n}\n",
    );
}

fn check_value(input: &str, expected: &str) {
    assert_eq!(
        String::from_utf8(
            compile_value(input.as_ref(), Default::default()).unwrap()
        )
        .unwrap(),
        expected,
    );
}

fn check(input: &[u8], expected: &str) {
    assert_eq!(
        compile_scss(input, Default::default())
            .map(|s| String::from_utf8(s).unwrap())
            .map_err(|e| {
                eprintln!("{}", e);
                "rsass failed"
            })
            .unwrap(),
        expected
    );
}
