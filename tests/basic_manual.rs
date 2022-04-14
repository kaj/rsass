//! These are from the "basic" directory in the sass specification.
//! See <https://github.com/sass/sass-spec> for source material.
//! See `tests/basic/main.rs` for semi-autoimported tests.
//! This file contains old tests that need special handling.
use rsass::{compile_scss, compile_scss_path, compile_value};

#[test]
fn txx_empty_rule() {
    assert_eq!(rsass(b"foo{}").unwrap(), "");
}

#[test]
fn use_module_star() {
    assert_eq!(
        rsass(
            b"@use 'tests/basic/defs' as *;\
              \ndiv {\
              \n  color: $color;\
              \n  col1: foo(1);\
              \n  col2: foo(0);\
              \n  @include myem;\
              \n}\n"
        )
        .unwrap(),
        "div {\
         \n  color: purple;\
         \n  col1: purple;\
         \n  col2: pink;\
         \n}\
         \ndiv em {\
         \n  color: pink;\
         \n}\n"
    );
}

#[test]
fn use_module() {
    assert_eq!(
        rsass(
            b"@use 'tests/basic/defs';\
              \ndiv {\
              \n  color: defs.$color;\
              \n  col1: defs.foo(1);\
              \n  col2: defs.foo(0);\
              \n  @include defs.myem;\
              \n}\n"
        )
        .unwrap(),
        "div {\
         \n  color: purple;\
         \n  col1: purple;\
         \n  col2: pink;\
         \n}\
         \ndiv em {\
         \n  color: pink;\
         \n}\n"
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
    assert_eq!(
        rsass(
            b"div {\n  a: 1 + 2;\n  b: 3 + 3/4;\n  c: 1/2 + 1/2;\n  \
          /* shouldn't eval the following \"300\" */\n  \
          d: 300;\n}"
        )
        .unwrap(),
        "div {\n  a: 3;\n  b: 3.75;\n  c: 1;\n  \
         /* shouldn't eval the following \"300\" */\n  \
         d: 300;\n}\n"
    );
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
    assert_eq!(
        rsass(
            b"foo {\n  foo: str-slice(\"bar\", 1, 2);\
              \n  bar: str-slice(\"bar\", 3);\n}\n"
        )
        .unwrap(),
        "foo {\n  foo: \"ba\";\n  bar: \"r\";\n}\n"
    );
}

/// From `spec/libsass-closed-issues/issue_574`
#[test]
fn ti574_map_trailing_comma() {
    assert_eq!(
        rsass(
            b"$flow: left;\n\n\
          $map: (\n  margin-#{$flow}: 3em,\n  foo: bar,\n);\n\n\
          .test {\n  margin-left: map-get($map, margin-left);\n}\n"
        )
        .unwrap(),
        ".test {\n  margin-left: 3em;\n}\n"
    );
}

/// From `spec/core_functions/invert/weight-parameter`
#[test]
fn weight_parameter() {
    assert_eq!(
        rsass(
            b".invert-with-weight {\n  zero-percent: invert(#edc, 0%);\n  \
              ten-percent: invert(#edc, 10%);\n  \
              keyword: invert(#edc, $weight: 10%);\n  \
              one-hundred-percent: invert(#edc, 100%);\n}\n"
        )
        .unwrap(),
        ".invert-with-weight {\n  zero-percent: #eeddcc;\n  \
         ten-percent: #d8cabd;\n  keyword: #d8cabd;\n  \
         one-hundred-percent: #112233;\n}\n"
    );
}

/// From `spec/libsass-closed-issues/issue_1133/normal`
#[test]
fn each_binds_multiple() {
    assert_eq!(
        rsass(
            b"@function foo($map) {\n    @return $map;\n}\n\n\
              a {\n    $map: foo((this: is, my: map));\
              \n    @each $k, $v in $map {\n        #{$k}: $v;\n    }\n}\n\n\
              b {\n    $map: call(\"foo\", (this: is, my: map));\
              \n    @each $k, $v in $map {\n        #{$k}: $v;\n    }\n}\n"
        )
        .unwrap(),
        "a {\n  this: is;\n  my: map;\n}\n\n\
         b {\n  this: is;\n  my: map;\n}\n"
    );
}

#[test]
fn div_simliar_unit() {
    assert_eq!(
        rsass(
            b"p { a: (1cm / 1mm); b: (200grad / 360deg); c: (1in / 1pt); }"
        )
        .unwrap(),
        "p {\
         \n  a: 10;\
         \n  b: 0.5;\
         \n  c: 72;\
         \n}\n"
    );
}

#[test]
fn different_numbers_should_compare_as_same() {
    assert_eq!(
        rsass(
            b"@use 'sass:math' as m;\
              \np {\
              \n  $t: 0.2;\
              \n  a: max(1, m.sin(30deg));\
              \n  b: max(0.2, m.sin(30deg));\
              \n  c: max($t, 0.23233234232231232312312323231223323);\
              \n  d: max($t+0.1, 0.23233234232231232312312323231223323);\
              \n}"
        )
        .unwrap(),
        "p {\
         \n  a: 1;\
         \n  b: 0.5;\
         \n  c: 0.2323323423;\
         \n  d: 0.3;\
         \n}\n"
    );
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
    assert_eq!(
        rsass(
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
              \n}\n"
        ).unwrap(),
        "a {\
         \n  color: (\"Helvetica\": (\"weights\": (\"regular\": (a: 300), \"medium\": 500, \"bold\": 700)));\
         \n}\n"
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
            "InfinityA444-4444M4 #000000/0-4444M4 #444+Infinity+Infinity+0.44444O #444+Infinity+0.0869565217",
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
    assert_eq!(
        rsass(
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
          \n}\n"
        )
        .unwrap(),
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
         \n}\n"
    );
}

#[test]
fn minmax_length_units() {
    assert_eq!(
        rsass(
            b"sel {\
         \n  a: max(-.8em, -.2vw);\
         \n  a: min(-.8em, -.2vw);\
         \n}\n"
        )
        .unwrap(),
        "sel {\
         \n  a: max(-0.8em, -0.2vw);\
         \n  a: min(-0.8em, -0.2vw);\
         \n}\n"
    );
}

#[test]
fn use_raw_css() {
    assert_eq!(
        rsass(b"@use 'tests/basic/misc.css'").unwrap(),
        "/* Some raw css for testing. */\
         \n@media screen and (min-width: 42em) {\
         \n  /* Do this for wide viewports */\
         \n  div.something {\n    display: flex;\n  }\
         \n}\
         \n@font-face {\
         \n  font-family: Cocanut;\n  font-style: regular;\
         \n  font-weight: regular;\
         \n  src: local(\"Cocanut.otf\"), url(\"cocanut.otf\");\
         \n}\
         \n#foo {\n  /* Draw a border on this. */\
         \n  border: solid 1px black;\n}\n"
    );
}
#[test]
fn import_raw_css() {
    assert_eq!(
        rsass(b"@import 'tests/basic/misc.css'").unwrap(),
        "/* Some raw css for testing. */\
         \n@media screen and (min-width: 42em) {\
         \n  /* Do this for wide viewports */\
         \n  div.something {\n    display: flex;\n  }\
         \n}\
         \n@font-face {\
         \n  font-family: Cocanut;\n  font-style: regular;\
         \n  font-weight: regular;\
         \n  src: local(\"Cocanut.otf\"), url(\"cocanut.otf\");\
         \n}\
         \n#foo {\n  /* Draw a border on this. */\
         \n  border: solid 1px black;\n}\n"
    );
}

#[test]
fn load_raw_css() {
    assert_eq!(
        rsass(
            b"@use 'sass:meta';\
              \n@include meta.load-css('tests/basic/misc.css');"
        )
        .unwrap(),
        "/* Some raw css for testing. */\
         \n@media screen and (min-width: 42em) {\
         \n  /* Do this for wide viewports */\
         \n  div.something {\n    display: flex;\n  }\
         \n}\
         \n@font-face {\
         \n  font-family: Cocanut;\n  font-style: regular;\
         \n  font-weight: regular;\
         \n  src: local(\"Cocanut.otf\"), url(\"cocanut.otf\");\
         \n}\
         \n#foo {\n  /* Draw a border on this. */\
         \n  border: solid 1px black;\n}\n"
    );
}

#[test]
fn open_by_path() {
    use rsass::compile_scss_path;
    use std::path::PathBuf;
    // Note use of path.join here.  I hope this will expose some
    // potential windows-only errors where path format differs from
    // url format.
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("basic");
    let result =
        compile_scss_path(&path.join("file.scss"), Default::default())
            .unwrap_or_else(|e| panic!("{}", e));
    assert_eq!(
        String::from_utf8(result).unwrap(),
        "em {\n  color: pink;\n}\n"
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

fn rsass(input: &[u8]) -> Result<String, String> {
    compile_scss(input, Default::default())
        .map(|s| String::from_utf8(s).unwrap())
        .map_err(|e| {
            eprintln!("{}", e);
            "rsass failed".into()
        })
}
