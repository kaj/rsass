//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/ie-functions.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin ie-opacity($opacity) {\
            \n  opacity: $opacity / 100;\
            \n  filter: alpha(opacity=$opacity);\
            \n  bilter: alpha(opacity=$opacity);\
            \n  kilter: type-of(opacity=$opacity);\
            \n  left: expression(document.body.clientWidth/2-oDiv.offsetWidth/2);\
            \n  flop: expression(document.body.clientHeight/2-oDiv.offsetHeight/2);\
            \n}\
            \n\
            \n$startColor: red;\
            \n$endColor: green;\
            \n\
            \nfoo {\
            \n  filter: progid:Microsoft.foo.bar.Baz(flip=#{foo + bar}, bang=#00ff00cc);\
            \n  something: blah(hux = mumble);\
            \n  blah: progid:something.something(flip=foobar, bang=#abc);\
            \n  blah: progid:bar.hux();\
            \n  blah: type-of(hux = mumble);\
            \n  @include ie-opacity(.5);\
            \n  left: expression(document.body.clientWidth/4);\
            \n  filter: progid:DXImageTransform.Microsoft.gradient(startColorstr=\'#{ie-hex-str($startColor)}\', endColorstr=\'#{ie-hex-str($endColor)}\', GradientType=1);\
            \n}\
            \n\
            \n.parser {\
            \n    filter: progid:DXImageTransform.Microsoft.Alpha(opacity=20);\
            \n    filter: progid:DXImageTransform.Microsoft.MotionBlur(strength=50)\
            \n            progid:DXImageTransform.Microsoft.BasicImage(rotation=2, mirror=1);\
            \n    filter: progid:DXImageTransform.Microsoft.gradient(startColorstr=#550000FF, endColorstr=#55FFFF00);\
            \n    filter: progid:DXImageTransform.Microsoft.BasicImage(rotation=2, mirror=1)\
            \n            progid:DXImageTransform.Microsoft.Alpha(opacity=50)\
            \n            progid:DXImageTransform.Microsoft.Blur(strength=10);\
            \n    filter: progid:DXImageTransform.Microsoft.Wave(strength=100)\
            \n            progid:DXImageTransform.Microsoft.CheckerBoard(duration=4);\
            \n    filter: progid:DXImageTransform.Microsoft.Wave(strength=100)\
            \n            progid:DXImageTransform.Microsoft.BasicImage(rotation=2, mirror=1)\
            \n            progid:DXImageTransform.Microsoft.Iris(irisstyle=\'STAR\', duration=4);\
            \n    filter: progid:DXImageTransform.Microsoft.MotionBlur(strength=13, direction=310)\
            \n            progid:DXImageTransform.Microsoft.Blur(pixelradius=2)\
            \n            progid:DXImageTransform.Microsoft.Wheel(duration=3);\
            \n    filter: progid:DXImageTransform.Microsoft.gradient(enabled=\'false\',\
            \n            startColorstr=#550000FF, endColorstr=#55FFFF00);\
            \n}\
            \n"
        )
        .unwrap(),
        "foo {\
        \n  filter: progid:Microsoft.foo.bar.Baz(flip=foobar, bang=#00ff00cc);\
        \n  something: blah(hux=mumble);\
        \n  blah: progid:something.something(flip=foobar, bang=#abc);\
        \n  blah: progid:bar.hux();\
        \n  blah: string;\
        \n  opacity: 0.005;\
        \n  filter: alpha(opacity=0.5);\
        \n  bilter: alpha(opacity=0.5);\
        \n  kilter: string;\
        \n  left: expression(document.body.clientWidth/2-oDiv.offsetWidth/2);\
        \n  flop: expression(document.body.clientHeight/2-oDiv.offsetHeight/2);\
        \n  left: expression(document.body.clientWidth/4);\
        \n  filter: progid:DXImageTransform.Microsoft.gradient(startColorstr=\"#FFFF0000\", endColorstr=\"#FF008000\", GradientType=1);\
        \n}\
        \n.parser {\
        \n  filter: progid:DXImageTransform.Microsoft.Alpha(opacity=20);\
        \n  filter: progid:DXImageTransform.Microsoft.MotionBlur(strength=50) progid:DXImageTransform.Microsoft.BasicImage(rotation=2, mirror=1);\
        \n  filter: progid:DXImageTransform.Microsoft.gradient(startColorstr=#550000FF, endColorstr=#55FFFF00);\
        \n  filter: progid:DXImageTransform.Microsoft.BasicImage(rotation=2, mirror=1) progid:DXImageTransform.Microsoft.Alpha(opacity=50) progid:DXImageTransform.Microsoft.Blur(strength=10);\
        \n  filter: progid:DXImageTransform.Microsoft.Wave(strength=100) progid:DXImageTransform.Microsoft.CheckerBoard(duration=4);\
        \n  filter: progid:DXImageTransform.Microsoft.Wave(strength=100) progid:DXImageTransform.Microsoft.BasicImage(rotation=2, mirror=1) progid:DXImageTransform.Microsoft.Iris(irisstyle=\"STAR\", duration=4);\
        \n  filter: progid:DXImageTransform.Microsoft.MotionBlur(strength=13, direction=310) progid:DXImageTransform.Microsoft.Blur(pixelradius=2) progid:DXImageTransform.Microsoft.Wheel(duration=3);\
        \n  filter: progid:DXImageTransform.Microsoft.gradient(enabled=\"false\", startColorstr=#550000FF, endColorstr=#55FFFF00);\
        \n}\
        \n"
    );
}
