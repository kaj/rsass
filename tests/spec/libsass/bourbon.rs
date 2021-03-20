//! Tests auto-converted from "sass-spec/spec/libsass/bourbon.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "@import \"lib/_bourbon.scss\";\
            \n\
            \nbox:hover {\
            \n  @include animation-name(scale, slide);\
            \n  @include animation-duration(2s);\
            \n  @include animation-timing-function(ease);\
            \n  @include animation-iteration-count(infinite);\
            \n\
            \n  // Animation shorthand works the same as the CSS3 animation shorthand\
            \n  @include animation(scale 1s ease-in, slide 2s ease);\
            \n}\
            \n\
            \ndiv {\
            \n  @include appearance(none);\
            \n\
            \n   /* boo boo boo */\
            \n  // Multiple image assets\
            \n  @include background-image(url(\"/images/a.png\"), url(\"images/b.png\"));\
            \n\
            \n  // Image asset with a linear-gradient\
            \n  @include background-image(url(\"/images/a.png\"), linear-gradient(white 0, yellow 50%, transparent 50%));\
            \n\
            \n  // Multiple linear-gradients - Demo\
            \n  @include background-image(linear-gradient(hsla(0, 100%, 100%, 0.25) 0%, hsla(0, 100%, 100%, 0.08) 50%, transparent 50%),\
            \n                             linear-gradient(#4e7ba3, darken(#4e7ba4, 10%)));\
            \n\
            \n  // NOT SUPPORTED - Multiple image assets with shorthand notation\
            \n  @include background-image(url(\"/images/a.png\") center no-repeat, url(\"images/b.png\") left repeat);\
            \n}\
            \n"
        )
        .unwrap(),
        "box:hover {\
        \n  -webkit-animation-name: scale, slide;\
        \n  -moz-animation-name: scale, slide;\
        \n  animation-name: scale, slide;\
        \n  -webkit-animation-duration: 2s;\
        \n  -moz-animation-duration: 2s;\
        \n  animation-duration: 2s;\
        \n  -webkit-animation-timing-function: ease;\
        \n  -moz-animation-timing-function: ease;\
        \n  animation-timing-function: ease;\
        \n  -webkit-animation-iteration-count: infinite;\
        \n  -moz-animation-iteration-count: infinite;\
        \n  animation-iteration-count: infinite;\
        \n  -webkit-animation: scale 1s ease-in, slide 2s ease;\
        \n  -moz-animation: scale 1s ease-in, slide 2s ease;\
        \n  animation: scale 1s ease-in, slide 2s ease;\
        \n}\
        \ndiv {\
        \n  -webkit-appearance: none;\
        \n  -moz-appearance: none;\
        \n  -ms-appearance: none;\
        \n  -o-appearance: none;\
        \n  appearance: none;\
        \n  /* boo boo boo */\
        \n  background-image: url(\"/images/a.png\"), url(\"images/b.png\");\
        \n  background-image: url(\"/images/a.png\"), url(\"images/b.png\");\
        \n  background-image: url(\"/images/a.png\"), -webkit-linear-gradient(white 0, yellow 50%, transparent 50%);\
        \n  background-image: url(\"/images/a.png\"), linear-gradient( white 0, yellow 50%, transparent 50%);\
        \n  background-image: -webkit-linear-gradient(rgba(255, 255, 255, 0.25) 0%, rgba(255, 255, 255, 0.08) 50%, transparent 50%), -webkit-linear-gradient(#4e7ba3, #3e6181);\
        \n  background-image: linear-gradient( rgba(255, 255, 255, 0.25) 0%, rgba(255, 255, 255, 0.08) 50%, transparent 50%), linear-gradient( #4e7ba3, #3e6181);\
        \n  background-image: -webkit-url(\"/images/a.png\")-gradient(center), -webkit-url(\"images/b.png\")-gradient(left);\
        \n  background-image: url(\"/images/a.png\")-gradient( center), url(\"images/b.png\")-gradient( left);\
        \n}\
        \n"
    );
}
