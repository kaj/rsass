//! Tests auto-converted from "sass-spec/spec/libsass-closed-issues/issue_2291.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            ".m__exhibit-header--medium {\
            \n    @extend #{&}--plain;\
            \n    &--plain {\
            \n        font-size: 1em;\
            \n    }\
            \n}\
            \n\
            \nfoo {\
            \n  bar[baz=\"#{&}\"][str=\"&\"] {\
            \n    asd: qwe;\
            \n  }\
            \n}\
            \n\
            \nA, B, C {\
            \n  #{&}-foo#{&}-bar {\
            \n    color: blue;\
            \n  }\
            \n  #{\"A, B, C\"}-foo#{\"A, B, C\"}-bar {\
            \n    color: blue;\
            \n  }\
            \n}\
            \n\
            \nA {\
            \n  B#{&}C {\
            \n    .b, .c, .d {\
            \n      #{&}-foo {\
            \n        parent: &bar;\
            \n        itpl: #{&}bar;\
            \n      }\
            \n      #{\"A .b, A .c, A .d\"}-foo {\
            \n        parent: &bar;\
            \n        itpl: #{&}bar;\
            \n      }\
            \n    }\
            \n  }\
            \n}"
        )
        .unwrap(),
        ".m__exhibit-header--medium--plain, .m__exhibit-header--medium {\
        \n  font-size: 1em;\
        \n}\
        \nfoo bar[baz=foo][str=\"&\"] {\
        \n  asd: qwe;\
        \n}\
        \nA A, A B, A C-fooA, A B, A C-bar, B A, B B, B C-fooA, B B, B C-bar, C A, C B, C C-fooA, C B, C C-bar {\
        \n  color: blue;\
        \n}\
        \nA A, A B, A C-fooA, A B, A C-bar, B A, B B, B C-fooA, B B, B C-bar, C A, C B, C C-fooA, C B, C C-bar {\
        \n  color: blue;\
        \n}\
        \nA BAC .b A BAC .b, A BAC .b A BAC .c, A BAC .b A BAC .d-foo, A BAC .c A BAC .b, A BAC .c A BAC .c, A BAC .c A BAC .d-foo, A BAC .d A BAC .b, A BAC .d A BAC .c, A BAC .d A BAC .d-foo {\
        \n  parent: A BAC .b A BAC .b, A BAC .b A BAC .c, A BAC .b A BAC .d-foo, A BAC .c A BAC .b, A BAC .c A BAC .c, A BAC .c A BAC .d-foo, A BAC .d A BAC .b, A BAC .d A BAC .c, A BAC .d A BAC .d-foo bar;\
        \n  itpl: A BAC .b A BAC .b, A BAC .b A BAC .c, A BAC .b A BAC .d-foo, A BAC .c A BAC .b, A BAC .c A BAC .c, A BAC .c A BAC .d-foo, A BAC .d A BAC .b, A BAC .d A BAC .c, A BAC .d A BAC .d-foobar;\
        \n}\
        \nA BAC .b A .b, A BAC .b A .c, A BAC .b A .d-foo, A BAC .c A .b, A BAC .c A .c, A BAC .c A .d-foo, A BAC .d A .b, A BAC .d A .c, A BAC .d A .d-foo {\
        \n  parent: A BAC .b A .b, A BAC .b A .c, A BAC .b A .d-foo, A BAC .c A .b, A BAC .c A .c, A BAC .c A .d-foo, A BAC .d A .b, A BAC .d A .c, A BAC .d A .d-foo bar;\
        \n  itpl: A BAC .b A .b, A BAC .b A .c, A BAC .b A .d-foo, A BAC .c A .b, A BAC .c A .c, A BAC .c A .d-foo, A BAC .d A .b, A BAC .d A .c, A BAC .d A .d-foobar;\
        \n}\
        \n"
    );
}
