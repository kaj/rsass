//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/17_basic_mixins.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin foo($x, $y) {\
            \n  hugabug: $y $x;\
            \n}\
            \n\
            \n@mixin bar($a, $b: flug) {\
            \n  flugablug: $a $b glug;\
            \n}\
            \n\
            \n@mixin hux() {\
            \n  no: parameters here;\
            \n  div, span {\
            \n    some: nested stuff;\
            \n    foo, bar {\
            \n      more: stuff so forth;\
            \n      blah: blah;\
            \n    }\
            \n  }\
            \n  /* end of hux */\
            \n}\
            \n\
            \na {\
            \n  hey: ho;\
            \n  @include foo(second, third);\
            \n  @include foo($y: kwd-y, $x: kwd-x);\
            \n  goo: boo hoo;\
            \n  @include hux;\
            \n  @include bar(pug);\
            \n  @include bar(pug, mug);\
            \n}\
            \n\
            \n\
            \n$x: from a variable;\
            \n\
            \ndiv {\
            \n  blah: blah $x blah;\
            \n}"
        )
        .unwrap(),
        "a {\
        \n  hey: ho;\
        \n  hugabug: third second;\
        \n  hugabug: kwd-y kwd-x;\
        \n  goo: boo hoo;\
        \n  no: parameters here;\
        \n  /* end of hux */\
        \n  flugablug: pug flug glug;\
        \n  flugablug: pug mug glug;\
        \n}\
        \na div, a span {\
        \n  some: nested stuff;\
        \n}\
        \na div foo, a div bar, a span foo, a span bar {\
        \n  more: stuff so forth;\
        \n  blah: blah;\
        \n}\
        \ndiv {\
        \n  blah: blah from a variable blah;\
        \n}\
        \n"
    );
}
