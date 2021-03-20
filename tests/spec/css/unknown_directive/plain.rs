//! Tests auto-converted from "sass-spec/spec/css/unknown_directive/plain.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            "// Unknown directives should support almost any sequence of valid tokens.\
            \n\
            \n// By default, characters are passed through unaltered.\
            \n@asdf .~@#$%^&*()_-+=[]|:<>,.?/;\
            \n\
            \n// Strings are tokenized as strings.\
            \n@asdf \"f\'o\" \'b\"r\' url(baz) url(\"qux\");\
            \n\
            \n// Comments are preserved.\
            \n@asdf foo //\
            \n      bar;\
            \n@asdf foo /* bar */ baz;\
            \n"
        )
        .unwrap(),
        "@asdf .~@#$%^&*()_-+=[]|:<>,.?/;\
        \n@asdf \"f\'o\" \'b\"r\' url(baz) url(\"qux\");\
        \n@asdf foo //\
        \n      bar;\
        \n@asdf foo /* bar */ baz;\
        \n"
    );
}
