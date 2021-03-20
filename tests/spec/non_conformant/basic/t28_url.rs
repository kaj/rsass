//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/28_url.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$x: pop;\
            \n$y: 123;\
            \n\
            \n\
            \n\
            \ndiv {\
            \n  foo: url(bloo/blah.css);\
            \n  bar: url(http://foo/bar/hux.css);\
            \n  foo: url(fudge#{$x}.css);\
            \n  bar: url(\"http://fudge#{$x}/styles.css\");\
            \n  hux: url(http://box_#{$y}////fudge#{$x}.css);\
            \n  @each $i in (1 2 3 4 5) {\
            \n    hux: url(http://box_#{$y}////fudge#{$x}.css);\
            \n    foo: url(http://blah.com/bar-#{$i}.css);\
            \n    bar: url(http://fonts.googleapis.com/css?family=Karla:400,700,400italic|Anonymous+Pro:400,700,400italic);\
            \n  }\
            \n  gloo: url(\"hey#{1+2}.css\");\
            \n  floo: url(hadoop-#{$y+321}.css);\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  foo: url(bloo/blah.css);\
        \n  bar: url(http://foo/bar/hux.css);\
        \n  foo: url(fudgepop.css);\
        \n  bar: url(\"http://fudgepop/styles.css\");\
        \n  hux: url(http://box_123////fudgepop.css);\
        \n  hux: url(http://box_123////fudgepop.css);\
        \n  foo: url(http://blah.com/bar-1.css);\
        \n  bar: url(http://fonts.googleapis.com/css?family=Karla:400,700,400italic|Anonymous+Pro:400,700,400italic);\
        \n  hux: url(http://box_123////fudgepop.css);\
        \n  foo: url(http://blah.com/bar-2.css);\
        \n  bar: url(http://fonts.googleapis.com/css?family=Karla:400,700,400italic|Anonymous+Pro:400,700,400italic);\
        \n  hux: url(http://box_123////fudgepop.css);\
        \n  foo: url(http://blah.com/bar-3.css);\
        \n  bar: url(http://fonts.googleapis.com/css?family=Karla:400,700,400italic|Anonymous+Pro:400,700,400italic);\
        \n  hux: url(http://box_123////fudgepop.css);\
        \n  foo: url(http://blah.com/bar-4.css);\
        \n  bar: url(http://fonts.googleapis.com/css?family=Karla:400,700,400italic|Anonymous+Pro:400,700,400italic);\
        \n  hux: url(http://box_123////fudgepop.css);\
        \n  foo: url(http://blah.com/bar-5.css);\
        \n  bar: url(http://fonts.googleapis.com/css?family=Karla:400,700,400italic|Anonymous+Pro:400,700,400italic);\
        \n  gloo: url(\"hey3.css\");\
        \n  floo: url(hadoop-444.css);\
        \n}\
        \n"
    );
}
