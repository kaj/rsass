//! These are from the "basic" directory in the sass specification.
//! See https://github.com/sass/sass-spec for source material.
//! I add one a test function for one specification at a time and then
//! try to implement that functionality without breaking those already
//! added.
extern crate rsass;
use rsass::compile_scss;

#[test]
fn t00_empty() {
    check(b"\n",
          b"")
}

#[test]
fn txx_empty_rule() {
    check(b"foo{}",
          b"")
}

#[test]
fn t01_simple_css() {
    check(b"a {\n  \
            color: blue;\n\
            }",
          b"a {\n  \
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
          b"div {\n  \
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
          b"a {\n  \
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
          b"a {\n  \
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
          b"div span {\n  color: red;\n  background: blue;\n}\n\
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
          b"/* top level comment\n   \
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
          b"span {\n  \
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
          b"span {\n  \
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
            \n\
/* top level comment -- should be preserved */
div {
  /* another comment that should be preserved */
  color: red;
  background: blue;
  $blux: hux; // gone!
  span {
    font-weight: bold;
    a {
      text-decoration: none; /* where will this comment go? */
      color: green;
      /* what about this comment? */ border: 1px $blah red;
    }
    /* yet another comment that should be preserved */
    display: inline-block;
  }  // gone!
  /* the next selector should be indented two spaces */
  empty {
    not_empty {
      blah: blah; // gone!
      bloo: bloo;
    }
  }
  p {
    padding: 10px 8%;
    -webkit-box-sizing: $blux;
  }
  margin: 10px 5px;
  h1 {
    color: $blip;
  }
}
/* last comment, top level again --
   compare the indentation! */
   
div {
  f: g;
  empty {
    span {
      a: b;
    }
  }
  empty_with_comment {
    /* hey now */
    span {
      c: d;
    }
  }
}",
          b"/* top level comment -- should be preserved */
div {
  /* another comment that should be preserved */
  color: red;
  background: blue;
  /* the next selector should be indented two spaces */
  margin: 10px 5px;
}
div span {
  font-weight: bold;
  /* yet another comment that should be preserved */
  display: inline-block;
}
div span a {
  text-decoration: none;
  /* where will this comment go? */
  color: green;
  /* what about this comment? */
  border: 1px bloo blee red;
}
div empty not_empty {
  blah: blah;
  bloo: bloo;
}
div p {
  padding: 10px 8%;
  -webkit-box-sizing: hux;
}
div h1 {
  color: \"a 'red' and \\\"blue\\\" value\";
}

/* last comment, top level again --
   compare the indentation! */
div {
  f: g;
}
div empty span {
  a: b;
}
div empty_with_comment {
  /* hey now */
}
div empty_with_comment span {
  c: d;
}
")
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
            wow: we are far inside;\n        but: it still works;\n      }\n      \
            hoo: boo;\n    }\n  }\n}",
          b"a, b {\n  color: red;\n  background: blue;\n}\n\n\
            c, d {\n  color: gray;\n}\n\
            c e, c f, d e, d f {\n  background: blue;\n  padding: 10px 5px;\n}\n\
            c g, c h, d g, d h {\n  blah: blah;\n  bloo: bloo;\n}\n\
            c i, c j, d i, d j {\n  foo: goo;\n}\n\
            c i k, c i l, c j k, c j l, d i k, d i l, d j k, d j l {\n  hoo: boo;\n}\n\
            c i k m, c i k n, c i k o, c i l m, c i l n, c i l o, c j k m, c j k n, c j k o, c j l m, c j l n, c j l o, d i k m, d i k n, d i k o, d i l m, d i l n, d i l o, d j k m, d j k n, d j k o, d j l m, d j l n, d j l o {\n  \
            wow: we are far inside;\n  but: it still works;\n}\n")
}

#[test]
fn t08_selector_combinators() {
    check(b"a   +   b  >  c {\n  \
            d e {\n    color: blue;\n    background: white;\n  }\n  \
            color: red;\n  background: gray;\n}",
          b"a + b > c {\n  color: red;\n  background: gray;\n}\n\
            a + b > c d e {\n  color: blue;\n  background: white;\n}\n")
}

#[test]
fn t09_selector_groups_and_combinators() {
    check(b"a + b, c {\n  blah: blah;\n  bleh: bleh;\n  \
            d e, f ~ g + h, > i {\n    bloo: bloo;\n    blee: blee;\n  }\n}",
          b"a + b, c {\n  blah: blah;\n  bleh: bleh;\n}\n\
            a + b d e, a + b f ~ g + h, a + b > i, c d e, c f ~ g + h, c > i \
            {\n  bloo: bloo;\n  blee: blee;\n}\n")
}

#[test]
fn t10_classes_and_ids() {
    check(b"a + b, .class {\n  blah: blah;\n  bleh: bleh;\n  \
            d #id, f ~ g.other + h, > i#grar \
            {\n    bloo: bloo;\n    blee: blee;\n  }\n}",
          b"a + b, .class {\n  blah: blah;\n  bleh: bleh;\n}\n\
            a + b d #id, a + b f ~ g.other + h, a + b > i#grar, \
            .class d #id, .class f ~ g.other + h, .class > i#grar \
            {\n  bloo: bloo;\n  blee: blee;\n}\n")
}

#[test]
fn t11_attribute_selectors() {
    check(b"[hey  =  'ho'], a > b {\n  blah: blah;\n  \
            c, [hoo *=    \"ha\" ] {\n    bloo: bloo;\n  }\n}",
          b"[hey='ho'], a > b {\n  blah: blah;\n}\n\
            [hey='ho'] c, [hey='ho'] [hoo*=\"ha\"], a > b c, \
            a > b [hoo*=\"ha\"] {\n  bloo: bloo;\n}\n")
}

#[test]
fn t13_back_references() {
    check(b"hey, ho {\n  \
            & > boo, foo &.goo {\n    bloo: bloo;\n  }\n  \
            blah: blah;\n\
            }",
          b"hey, ho {\n  blah: blah;\n}\n\
            hey > boo, foo hey.goo, ho > boo, foo ho.goo {\n  bloo: bloo;\n}\n")
}

fn check(input: &[u8], expected: &[u8]) {
    use std::str::from_utf8;
    let result = compile_scss(input);
    if let Ok(output) = result {
        if let (Ok(output), Ok(expected)) = (from_utf8(&output), from_utf8(expected)) {
            assert_eq!(output, expected)
        } else {
            assert_eq!(output, expected)
        }
    } else {
        assert_eq!(result, Ok(expected.into()))
    }
}
