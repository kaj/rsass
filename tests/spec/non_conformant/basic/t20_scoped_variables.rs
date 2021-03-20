//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/20_scoped_variables.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin foo() {\
            \n  /* begin foo */\
            \n  /* assigning to $x */\
            \n  $x: inside foo;\
            \n  x: $x;\
            \n  /* end foo */\
            \n}\
            \n\
            \nouter {\
            \n  /* assigning to $x */\
            \n  $x: inside outer scope;\
            \n  blah: blah;\
            \n  inner {\
            \n    @include foo();\
            \n    x: $x;\
            \n  }\
            \n}"
        )
        .unwrap(),
        "outer {\
        \n  /* assigning to $x */\
        \n  blah: blah;\
        \n}\
        \nouter inner {\
        \n  /* begin foo */\
        \n  /* assigning to $x */\
        \n  x: inside foo;\
        \n  /* end foo */\
        \n  x: inside outer scope;\
        \n}\
        \n"
    );
}
