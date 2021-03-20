//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_931.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin img-opacity($trans) {\
            \n          filter : alpha(opacity=($trans * 100));\
            \n      -ms-filter : \"progid:DXImageTransform.Microsoft.Alpha(Opacity=#{$trans * 100})\";\
            \n    -moz-opacity : $trans;\
            \n  -khtml-opacity : $trans;\
            \n         opacity : $trans;\
            \n}\
            \n\
            \nimg {\
            \n  @include img-opacity(.5);\
            \n}"
        )
        .unwrap(),
        "img {\
        \n  filter: alpha(opacity=50);\
        \n  -ms-filter: \"progid:DXImageTransform.Microsoft.Alpha(Opacity=50)\";\
        \n  -moz-opacity: 0.5;\
        \n  -khtml-opacity: 0.5;\
        \n  opacity: 0.5;\
        \n}\
        \n"
    );
}
