//! Tests auto-converted from "sass-spec/spec/non_conformant/basic/54_adjacent_identifiers_with_hyphens.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "input {\
            \n    outline: 5px auto -webkit-focus-ring-color;\
            \n    foo: random -hello-this-is-dog;\
            \n    bar: rando -two-or-more -things-that-start -with-hyphens;\
            \n    baz: foo - bar;\
            \n}"
        )
        .unwrap(),
        "input {\
        \n  outline: 5px auto -webkit-focus-ring-color;\
        \n  foo: random -hello-this-is-dog;\
        \n  bar: rando -two-or-more -things-that-start -with-hyphens;\
        \n  baz: foo-bar;\
        \n}\
        \n"
    );
}
