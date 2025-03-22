//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/06_nesting_and_comments.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("06_nesting_and_comments")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("$blah: bloo blee;\
             \n$blip: \"a \'red\' and \\\"blue\\\" value\";\n\
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
             \n  p { /* comment after open brace goes in */\
             \n    padding: 10px 8%;\
             \n    -webkit-box-sizing: $blux;\
             \n  } /* comment after close brace goes out */\
             \n  margin: 10px 5px;\
             \n  h1 {\
             \n    color: $blip;\
             \n  }\
             \n}\
             \n/* last comment, top level again --\
             \n   compare the indentation! */\n\
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
             \n}"),
        "/* top level comment -- should be preserved */\
         \ndiv {\
         \n  /* another comment that should be preserved */\
         \n  color: red;\
         \n  background: blue;\
         \n  /* the next selector should be indented two spaces */\
         \n  /* comment after close brace goes out */\
         \n  margin: 10px 5px;\
         \n}\
         \ndiv span {\
         \n  font-weight: bold;\
         \n  /* yet another comment that should be preserved */\
         \n  display: inline-block;\
         \n}\
         \ndiv span a {\
         \n  text-decoration: none; /* where will this comment go? */\
         \n  color: green;\
         \n  /* what about this comment? */\
         \n  border: 1px bloo blee red;\
         \n}\
         \ndiv empty not_empty {\
         \n  blah: blah;\
         \n  bloo: bloo;\
         \n}\
         \ndiv p { /* comment after open brace goes in */\
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
         \n}\n"
    );
}
