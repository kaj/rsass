//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_143.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$path: \"images\";\
            \n$file: \"kittens.jpg\";\
            \n$image: \"\";\
            \n$other: file_join(\"images\", \"kittens.jpg\");\
            \n\
            \n@if $image != none {\
            \n\t$image: url(file_join($path, $file));\
            \n}\
            \nbody {\
            \n\tbackground: $image;\
            \n\tcolor: $other;\
            \n}\
            \n"
        )
        .unwrap(),
        "body {\
        \n  background: url(file_join(\"images\", \"kittens.jpg\"));\
        \n  color: file_join(\"images\", \"kittens.jpg\");\
        \n}\
        \n"
    );
}
