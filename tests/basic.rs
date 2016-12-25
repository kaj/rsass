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
