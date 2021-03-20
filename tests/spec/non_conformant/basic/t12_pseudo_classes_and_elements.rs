//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/12_pseudo_classes_and_elements.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "a b {\
            \n  color: red;\
            \n  :first-child, :nth-of-type(  -2n+1 ) {\
            \n    .foo#bar:nth-child(even) {\
            \n      hoo: goo;\
            \n    }\
            \n    blah: bloo;\
            \n    ::after {\
            \n      content: \"glux\";\
            \n      color: green;\
            \n    }\
            \n    :not(.foo) {\
            \n      hoo: boo;\
            \n    }\
            \n    a { b: c; }\
            \n  }\
            \n}"
        )
        .unwrap(),
        "a b {\
        \n  color: red;\
        \n}\
        \na b :first-child, a b :nth-of-type(-2n+1) {\
        \n  blah: bloo;\
        \n}\
        \na b :first-child .foo#bar:nth-child(even), a b :nth-of-type(-2n+1) .foo#bar:nth-child(even) {\
        \n  hoo: goo;\
        \n}\
        \na b :first-child ::after, a b :nth-of-type(-2n+1) ::after {\
        \n  content: \"glux\";\
        \n  color: green;\
        \n}\
        \na b :first-child :not(.foo), a b :nth-of-type(-2n+1) :not(.foo) {\
        \n  hoo: boo;\
        \n}\
        \na b :first-child a, a b :nth-of-type(-2n+1) a {\
        \n  b: c;\
        \n}\
        \n"
    );
}
