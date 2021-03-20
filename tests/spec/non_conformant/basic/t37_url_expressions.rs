//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/37_url_expressions.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$x: true;\
            \n$file-1x: \"budge.png\";\
            \n\
            \n@function fudge($str) {\
            \n  @return \"assets/fudge/\" + $str;\
            \n}\
            \n\
            \ndiv {\
            \n  blah: url(foo + bar);\
            \n  blah: url(fn(\"s\"));\
            \n  blah: url(if(true, \"red.png\", \"blue.png\"));\
            \n  blah: url(hello-#{world}.png);\
            \n  blah: url(if($x, fudge(\"#{$file-1x}\"), \"#{$file-1x}\"));\
            \n}"
        )
        .unwrap(),
        "div {\
        \n  blah: url(foobar);\
        \n  blah: url(fn(\"s\"));\
        \n  blah: url(\"red.png\");\
        \n  blah: url(hello-world.png);\
        \n  blah: url(\"assets/fudge/budge.png\");\
        \n}\
        \n"
    );
}
