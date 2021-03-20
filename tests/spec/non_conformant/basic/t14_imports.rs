//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/14_imports.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@import \"a.scss\";\
            \n\
            \nfoo {\
            \n  blah: blah;\
            \n  goo {\
            \n    blee: blee;\
            \n    @import \"../14_imports/b.scss\";\
            \n    hello: world;\
            \n  }\
            \n  @import \"sub/c.scss\";\
            \n}"
        )
        .unwrap(),
        "div span {\
        \n  moo: goo;\
        \n}\
        \nfoo {\
        \n  blah: blah;\
        \n}\
        \nfoo goo {\
        \n  blee: blee;\
        \n  hello: world;\
        \n}\
        \nfoo goo hoo {\
        \n  mux: scooba-dee-doo;\
        \n  flux: gooboo boo;\
        \n}\
        \nfoo goo hoo d {\
        \n  inside: d now;\
        \n}\
        \nfoo blux {\
        \n  hey: another thing;\
        \n  ho: will this work;\
        \n}\
        \n"
    );
}
