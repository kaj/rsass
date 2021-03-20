//! Tests auto-converted from "sass-spec/spec/libsass/test.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "$x: 3;\
            \n\
            \ndiv {\
            \n\tnoo: not $x;\
            \n\tpoo: not 3;\
            \n\tdoo: not($x);\
            \n\tgoo: not(3);\
            \n\tzoo: not 1 + 2;\
            \n\
            \n\troo: not not not $x;\
            \n\thoo: not not not 3;\
            \n}\
            \n\
            \n@mixin foo($x-1, $x-2) {\
            \n  goo: $x-1;\
            \n  poo: $x-2;\
            \n}\
            \n$hux: \"blah.css\";\
            \nspan {\
            \n  a: rgba(100, 20, 0, 1);\
            \n  b: rgba(#abc, 1);\
            \n  c: compact(hello, my, false, name, is, false, aaron, false, false);\
            \n  d: join(1 2 3, 4 5 6, comma);\
            \n  e: join(a b c, d e f);\
            \n  f: change-color(#102030, $blue: 5);\
            \n  g: change-color(#102030, $red: 120, $blue: 5);\
            \n  h: hsl(25, 100%, 80%);\
            \n  h: change-color(#ffc499, $alpha: 0.8, $lightness: 40%);\
            \n  h: change-color(hsl(25, 100%, 80%), $alpha: 0.8, $lightness: 40%);\
            \n  i: hsla(25, 100%, 40%, 0.8);\
            \n  foo: url(\"http://blah/flah/grah\");\
            \n  foo: url(http://foo/bar/buzz.css);\
            \n  foo: url(hey#{1+3}ho.css);\
            \n  foo: url($hux);\
            \n  bug: compact(false 1 2 false 3 4 5 false);\
            \n  pug: compact(false, 1, 2, false, 3, 4, 5, false);\
            \n  mug: compact((flug, false, blug, false, krug, false));\
            \n}\
            \n\
            \n@mixin bg($file) {\
            \n  background: url($file) no-repeat;\
            \n}\
            \n\
            \ndiv {\
            \n  flug: url(bug.mug);\
            \n  krug: nth(1 2 3, 2px);\
            \n  blug: nth(a b c d, 3);\
            \n  flig: comparable(34, 22px) comparable(1%, 3) comparable(2, 1) comparable(4cm, 1in);\
            \n  flug: comparable(1px, 2.3in) comparable(1%, 2pt);\
            \n  flib: comparable(3ex, 2px) comparable(3em, 2cm);\
            \n  glib: not(fudge) not(false) not(0) not(red);\
            \n  trib: if(red, yellow, not taken);\
            \n  trub: if(not(fudge), not taken, here we are);\
            \n}\
            \n\
            \n$width: 10px;\
            \n$height: 10px;\
            \n@media (-webkit-min-device-pixel-ratio: 2), (-moz-min-device-pixel-ratio: 2) {\
            \n  div {\
            \n    background-image: url(fudge);\
            \n    -webkit-background-size: $width $height;\
            \n  }\
            \n}\
            \n\
            \nspan {\
            \n  @media foo {\
            \n    div {\
            \n      blah: blah;\
            \n    }\
            \n    @media bar {\
            \n      p {\
            \n        blah: bloo;\
            \n      }\
            \n    }\
            \n  }\
            \n}\
            \n\
            \ngudge {\
            \n  fudge: 20 + \"%\";\
            \n  mudge: 1 + blang;\
            \n}\
            \n\
            \nh1:lang(en) {\
            \n  foo: bar;\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  noo: false;\
        \n  poo: false;\
        \n  doo: false;\
        \n  goo: false;\
        \n  zoo: false2;\
        \n  roo: false;\
        \n  hoo: false;\
        \n}\
        \nspan {\
        \n  a: #641400;\
        \n  b: #aabbcc;\
        \n  c: compact(hello, my, false, name, is, false, aaron, false, false);\
        \n  d: 1, 2, 3, 4, 5, 6;\
        \n  e: a b c d e f;\
        \n  f: #102005;\
        \n  g: #782005;\
        \n  h: #ffc499;\
        \n  h: rgba(204, 86, 0, 0.8);\
        \n  h: rgba(204, 85, 0, 0.8);\
        \n  i: rgba(204, 85, 0, 0.8);\
        \n  foo: url(\"http://blah/flah/grah\");\
        \n  foo: url(http://foo/bar/buzz.css);\
        \n  foo: url(hey4ho.css);\
        \n  foo: url(\"blah.css\");\
        \n  bug: compact(false 1 2 false 3 4 5 false);\
        \n  pug: compact(false, 1, 2, false, 3, 4, 5, false);\
        \n  mug: compact(flug, false, blug, false, krug, false);\
        \n}\
        \ndiv {\
        \n  flug: url(bug.mug);\
        \n  krug: 2;\
        \n  blug: c;\
        \n  flig: true true true true;\
        \n  flug: true false;\
        \n  flib: false false;\
        \n  glib: false true false false;\
        \n  trib: yellow;\
        \n  trub: here we are;\
        \n}\
        \n@media (-webkit-min-device-pixel-ratio: 2), (-moz-min-device-pixel-ratio: 2) {\
        \n  div {\
        \n    background-image: url(fudge);\
        \n    -webkit-background-size: 10px 10px;\
        \n  }\
        \n}\
        \n@media foo {\
        \n  span div {\
        \n    blah: blah;\
        \n  }\
        \n}\
        \ngudge {\
        \n  fudge: \"20%\";\
        \n  mudge: 1blang;\
        \n}\
        \nh1:lang(en) {\
        \n  foo: bar;\
        \n}\
        \n"
    );
}
