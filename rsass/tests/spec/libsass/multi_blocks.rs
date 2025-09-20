//! Tests auto-converted from "sass-spec/spec/libsass/multi-blocks.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("multi-blocks")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(
            "a b {\
             \n  color: red;\
             \n  c d {\
             \n    height: 10;\
             \n  }\
             \n  e f {\
             \n    width: 12;\
             \n  }\
             \n}\n\
             \n@media all and (min-width: 960px) {\
             \n  b {\
             \n    font-weight: normal;\
             \n  }\
             \n}\n\
             \n@media (min-width: 980px) {\
             \n  a {\
             \n    color: red;\
             \n  }\
             \n}\n\
             \n@media screen and (all) {\
             \n  /* hey */\
             \n  p {\
             \n    color: blue;\
             \n    a {\
             \n      color: green;\
             \n      &:after {\
             \n        content: \">>\";\
             \n      }\
             \n    }\
             \n  }\
             \n  span {\
             \n    display: inline-block;\
             \n  }\
             \n}\n\
             \na b c {\
             \n  /* a */\
             \n  blee: blee;\
             \n  /* b */\
             \n  d e f {\
             \n    blah: blah;\
             \n    bloo: bloo;\
             \n  }\
             \n  /* c */\
             \n  g h, i j {\
             \n    @media print and (foo: 1 2 3), (bar: 3px hux(muz)), not screen {\
             \n      /* aa */\
             \n      hey: ho;\
             \n      /* bb */\
             \n      k l m {\
             \n        hee: fee;\
             \n      }\
             \n      /* cc */\
             \n      haa: hoo;\
             \n      /* dd */\
             \n    }\
             \n  }\
             \n  /* d */\
             \n  blah: blah;\
             \n}"
        ),
        "a b {\
         \n  color: red;\
         \n}\
         \na b c d {\
         \n  height: 10;\
         \n}\
         \na b e f {\
         \n  width: 12;\
         \n}\
         \n@media all and (min-width: 960px) {\
         \n  b {\
         \n    font-weight: normal;\
         \n  }\
         \n}\
         \n@media (min-width: 980px) {\
         \n  a {\
         \n    color: red;\
         \n  }\
         \n}\
         \n@media screen and (all) {\
         \n  /* hey */\
         \n  p {\
         \n    color: blue;\
         \n  }\
         \n  p a {\
         \n    color: green;\
         \n  }\
         \n  p a:after {\
         \n    content: \">>\";\
         \n  }\
         \n  span {\
         \n    display: inline-block;\
         \n  }\
         \n}\
         \na b c {\
         \n  /* a */\
         \n  blee: blee;\
         \n  /* b */\
         \n}\
         \na b c d e f {\
         \n  blah: blah;\
         \n  bloo: bloo;\
         \n}\
         \na b c {\
         \n  /* c */\
         \n}\
         \n@media print and (foo: 1 2 3), (bar: 3px hux(muz)), not screen {\
         \n  a b c g h, a b c i j {\
         \n    /* aa */\
         \n    hey: ho;\
         \n    /* bb */\
         \n  }\
         \n  a b c g h k l m, a b c i j k l m {\
         \n    hee: fee;\
         \n  }\
         \n  a b c g h, a b c i j {\
         \n    /* cc */\
         \n    haa: hoo;\
         \n    /* dd */\
         \n  }\
         \n}\
         \na b c {\
         \n  /* d */\
         \n  blah: blah;\
         \n}\n"
    );
}
