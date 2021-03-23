//! Tests auto-converted from "sass-spec/spec/core_functions/modules/selector.hrx"

#[test]
fn append() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:selector\";\
            \na {b: selector.append(c, d)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: cd;\
        \n}\
        \n"
    );
}
mod error {

    // Ignoring "selector_append", error tests are not supported yet.

    // Ignoring "selector_extend", error tests are not supported yet.

    // Ignoring "selector_nest", error tests are not supported yet.

    // Ignoring "selector_parse", error tests are not supported yet.

    // Ignoring "selector_replace", error tests are not supported yet.

    // Ignoring "selector_unify", error tests are not supported yet.
}
#[test]
#[ignore] // unexepected error
fn extend() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:selector\";\
            \na {b: selector.extend(c, c, d)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c, d;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn is_superselector() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:selector\";\
            \na {b: selector.is-superselector(c, d)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: false;\
        \n}\
        \n"
    );
}
#[test]
fn nest() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:selector\";\
            \na {b: selector.nest(c, d)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c d;\
        \n}\
        \n"
    );
}
#[test]
fn parse() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:selector\";\
            \na {b: selector.parse(\".c, .d\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: .c, .d;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn replace() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:selector\";\
            \na {b: selector.replace(c, c, d)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: d;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn simple_selectors() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:selector\";\
            \na {b: selector.simple-selectors(\".c.d\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: .c, .d;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn unify() {
    assert_eq!(
        crate::rsass(
            "@use \"sass:selector\";\
            \na {b: selector.unify(\".c\", \".d\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: .c.d;\
        \n}\
        \n"
    );
}
