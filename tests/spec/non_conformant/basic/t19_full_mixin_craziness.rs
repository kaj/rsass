//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/19_full_mixin_craziness.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$x: global-x;\
            \n$y: global-y;\
            \n$z: global-z;\
            \n\
            \n@mixin foo($x, $y) {\
            \n  /* begin foo */\
            \n  margin: $x $y;\
            \n  blip {\
            \n    hey: now;\
            \n  }\
            \n  /* end foo */\
            \n}\
            \n\
            \n@mixin foogoo($x, $y, $z) {\
            \n  margin: $x $y $z;\
            \n}\
            \n\
            \n@mixin hux($y) {\
            \n  /* begin hux */\
            \n  color: $y;\
            \n  @include foo(called-from-hux, $y: $y);\
            \n  /* end hux */\
            \n}\
            \n\
            \ndiv {\
            \n  @include foo(1, 2);\
            \n  @include foo(1, 3);\
            \n  @include foogoo(1, 2, $z: zee);\
            \n  @include foogoo(1, $y /* blah */ : kwd-y, $z: kwd-z);\
            \n}\
            \n\
            \ndiv {\
            \n  @include hux($y: $y);\
            \n}\
            \n\
            \n$y: different-global-y;\
            \n\
            \ndiv {\
            \n  @include hux(calling-hux-again);\
            \n}\
            \n\
            \n@mixin bung() {\
            \n  blah: original-bung;\
            \n}\
            \n\
            \ndiv {\
            \n  @include bung();\
            \n}\
            \n\
            \n@mixin bung() {\
            \n  blah: redefined-bung;\
            \n}\
            \n\
            \ndiv {\
            \n  @include bung();\
            \n}\
            \n\
            \ndiv {\
            \n  /* calls to nullary mixins may omit the empty argument list */\
            \n  @include bung;\
            \n}\
            \n\
            \ndiv {\
            \n  @include foo($x: kwdarg1, $y: kwdarg2);\
            \n}\
            \n\
            \n@mixin ruleset() {\
            \n  hoo {\
            \n    color: boo;\
            \n  }\
            \n}\
            \n\
            \n@include ruleset();\
            \n\
            \n$da: default argument;\
            \n\
            \n@mixin default_args($x, $y: $da) {\
            \n  blah: $x $y;\
            \n}\
            \n$da: some other default;\
            \n\
            \ndiv {\
            \n  @include default_args(boogoo);\
            \n}\
            \n\
            \n@mixin original() {\
            \n  value: original;\
            \n}\
            \n\
            \ndiv {\
            \n  @include original();\
            \n}\
            \n\
            \n@mixin original() {\
            \n  value: no longer original;\
            \n}\
            \n\
            \ndiv {\
            \n  @include original();\
            \n}\
            \n\
            \n@mixin set-x($x) {\
            \n  $x: changed local x;\
            \n  arg: $x;\
            \n  $y: changed global y !global;\
            \n  blarg: $y;\
            \n}\
            \n\
            \ndiv {\
            \n  @include set-x(blah);\
            \n  a: $x;\
            \n  b: $y;\
            \n}\
            \n"
        )
        .unwrap(),
        "div {\
        \n  /* begin foo */\
        \n  margin: 1 2;\
        \n  /* end foo */\
        \n  /* begin foo */\
        \n  margin: 1 3;\
        \n  /* end foo */\
        \n  margin: 1 2 zee;\
        \n  margin: 1 kwd-y kwd-z;\
        \n}\
        \ndiv blip {\
        \n  hey: now;\
        \n}\
        \ndiv blip {\
        \n  hey: now;\
        \n}\
        \ndiv {\
        \n  /* begin hux */\
        \n  color: global-y;\
        \n  /* begin foo */\
        \n  margin: called-from-hux global-y;\
        \n  /* end foo */\
        \n  /* end hux */\
        \n}\
        \ndiv blip {\
        \n  hey: now;\
        \n}\
        \ndiv {\
        \n  /* begin hux */\
        \n  color: calling-hux-again;\
        \n  /* begin foo */\
        \n  margin: called-from-hux calling-hux-again;\
        \n  /* end foo */\
        \n  /* end hux */\
        \n}\
        \ndiv blip {\
        \n  hey: now;\
        \n}\
        \ndiv {\
        \n  blah: original-bung;\
        \n}\
        \ndiv {\
        \n  blah: redefined-bung;\
        \n}\
        \ndiv {\
        \n  /* calls to nullary mixins may omit the empty argument list */\
        \n  blah: redefined-bung;\
        \n}\
        \ndiv {\
        \n  /* begin foo */\
        \n  margin: kwdarg1 kwdarg2;\
        \n  /* end foo */\
        \n}\
        \ndiv blip {\
        \n  hey: now;\
        \n}\
        \nhoo {\
        \n  color: boo;\
        \n}\
        \ndiv {\
        \n  blah: boogoo some other default;\
        \n}\
        \ndiv {\
        \n  value: original;\
        \n}\
        \ndiv {\
        \n  value: no longer original;\
        \n}\
        \ndiv {\
        \n  arg: changed local x;\
        \n  blarg: changed global y;\
        \n  a: global-x;\
        \n  b: changed global y;\
        \n}\
        \n"
    );
}
