//! These are from the "output_styles/compressed/basic" directory in the
//! sass specification.
//! See https://github.com/sass/sass-spec for source material.
//! I add one a test function for one specification at a time and then
//! try to implement that functionality without breaking those already
//! added.
extern crate rsass;
use rsass::{OutputStyle, compile_scss};

#[test]
fn t01_simple_css() {
    check(b"a {\n  \
            color: blue;\n\
            }",
          "a{color:blue}\n")
}

#[test]
fn t02_simple_nesting() {
    check(b"div {\n  img {\n    border: 0px;\n  }\n}",
          "div img{border:0px}\n")
}

#[test]
fn t03_simple_variable() {
    check(b"$color: red;\n\na {\n  color: $color;\n}",
          "a{color:red}\n")
}

#[test]
fn t04_basic_variables() {
    check(b"$color: \"black\";\n$color: red;\n$background: \"blue\";\n\n\
            a {\n  color: $color;\n  background: $background;\n}\n\n\
            $y: before;\n\n\
            $x: 1 2 $y;\n\n\
            foo {\n  a: $x;\n}\n\n\
            $y: after;\n\n\
            foo {\n  a: $x;\n}",
          "a{color:red;background:\"blue\"}foo{a:1 2 before}\
           foo{a:1 2 before}\n")
}

#[test]
fn t05_empty_levels() {
    check(b"div {\n  \
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
           empty1 empty2 div empty3 span{blah:blah;blah:blah}\n")
}

#[test]
fn t06_nesting_and_comments() {
    // No comments preserved in compressed output!
    check(b"$blah: bloo blee;\n$blip: \"a 'red' and \\\"blue\\\" value\";\n\n\
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
           div{f:g}div empty span{a:b}div empty_with_comment span{c:d}\n")
}

#[test]
fn t08_selector_combinators() {
    check(b"a   +   b  >  c {\n  \
            d e {\n    color: blue;\n    background: white;\n  }\n  \
            color: red;\n  background: gray;\n}",
          "a+b>c{color:red;background:gray}\
           a+b>c d e{color:blue;background:white}\n")
}

fn check(input: &[u8], expected: &str) {
    assert_eq!(compile_scss(input, OutputStyle::Compressed).and_then(|s| {
                   String::from_utf8(s)
                       .map_err(|e| format!("Non-utf8 output: {}", e))
               }),
               Ok(expected.into()));
}
