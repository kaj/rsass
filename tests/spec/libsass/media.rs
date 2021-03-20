//! Tests auto-converted from "sass-spec/spec/libsass/media.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@media all and (min-width: 960px) {\
            \n  b {\
            \n    font-weight: normal;\
            \n  }\
            \n}\
            \n\
            \n@media (min-width: 980px) {\
            \n  a {\
            \n    color: red;\
            \n  }\
            \n}\
            \n\
            \n@media all {\
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
            \n}\
            \n\
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
            \n}\
            \n\
            \n@mixin simple-media-query($max-width, $min-width) {\
            \n  @media only screen and (max-width : $max-width) and (min-width : $min-width) {\
            \n    @content;\
            \n  }\
            \n}\
            \n\
            \n@mixin test($value) {\
            \n  border-color: $value;\
            \n}\
            \n\
            \nbody {\
            \n  @include test(\"#ccc\");\
            \n  @include simple-media-query(900px, 400px) {\
            \n    border-color: black;\
            \n  }\
            \n}\
            \n\
            \n$foo: 23;\
            \n$bar: 45;\
            \n\
            \n@media only screen and (max-width: $foo) and (min-width: $bar) {\
            \n  hey {\
            \n    ho: hoo;\
            \n  }\
            \n}\
            \n\
            \n@media (max-width: 200) and (min-width: 100) {\
            \n  div {\
            \n    color: red;\
            \n  }\
            \n}\
            \n\
            \n@media not bl#{ah} and (width: 200px) {\
            \n  div {\
            \n    color: brown, blue, black;\
            \n  }\
            \n}\
            \n\
            \n@mixin media($var1, $var2) {\
            \n  @media screen and ($var1: $var2) {\
            \n    @content;\
            \n  }\
            \n}\
            \n\
            \n@include media(max-device-width, 500px) {\
            \n  foo {\
            \n    bar: \"works\";\
            \n  }\
            \n}\
            \n\
            \ndiv {\
            \n  color: red;\
            \n  span {\
            \n    color: blue;\
            \n    @media screen {\
            \n      p {\
            \n        color: green;\
            \n      }\
            \n    }\
            \n  }\
            \n}"
        )
        .unwrap(),
        "@media all and (min-width: 960px) {\
        \n  b {\
        \n    font-weight: normal;\
        \n  }\
        \n}\
        \n@media (min-width: 980px) {\
        \n  a {\
        \n    color: red;\
        \n  }\
        \n}\
        \n@media all {\
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
        \n  /* c */\
        \n  /* d */\
        \n  blah: blah;\
        \n}\
        \na b c d e f {\
        \n  blah: blah;\
        \n  bloo: bloo;\
        \n}\
        \n@media print and (foo: 1 2 3), (bar: 3px hux(muz)), not screen {\
        \n  a b c g h, a b c i j {\
        \n    /* aa */\
        \n    hey: ho;\
        \n    /* bb */\
        \n    /* cc */\
        \n    haa: hoo;\
        \n    /* dd */\
        \n  }\
        \n  a b c g h k l m, a b c i j k l m {\
        \n    hee: fee;\
        \n  }\
        \n}\
        \nbody {\
        \n  border-color: \"#ccc\";\
        \n}\
        \n@media only screen and (max-width: 900px) and (min-width: 400px) {\
        \n  body {\
        \n    border-color: black;\
        \n  }\
        \n}\
        \n@media only screen and (max-width: 23) and (min-width: 45) {\
        \n  hey {\
        \n    ho: hoo;\
        \n  }\
        \n}\
        \n@media (max-width: 200) and (min-width: 100) {\
        \n  div {\
        \n    color: red;\
        \n  }\
        \n}\
        \n@media not blah and (width: 200px) {\
        \n  div {\
        \n    color: brown, blue, black;\
        \n  }\
        \n}\
        \n@media screen and (max-device-width: 500px) {\
        \n  foo {\
        \n    bar: \"works\";\
        \n  }\
        \n}\
        \ndiv {\
        \n  color: red;\
        \n}\
        \ndiv span {\
        \n  color: blue;\
        \n}\
        \n@media screen {\
        \n  div span p {\
        \n    color: green;\
        \n  }\
        \n}\
        \n"
    );
}
