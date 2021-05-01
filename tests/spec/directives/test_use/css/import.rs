//! Tests auto-converted from "sass-spec/spec/directives/use/css/import.hrx"

#[test]
#[ignore] // wrong result
fn import_into_use() {
    assert_eq!(
        crate::rsass(
            "@import \"imported\";\
            \n\
            \nin-input {a: b}\
            \n"
        )
        .unwrap(),
        "in-used {\
        \n  a: b;\
        \n}\
        \nin-imported {\
        \n  a: b;\
        \n}\
        \nin-input {\
        \n  a: b;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // wrong result
fn import_into_use_into_import() {
    assert_eq!(
        crate::rsass(
            "@import \"imported-downstream\";\
            \n\
            \nin-input {a: b}\
            \n"
        )
        .unwrap(),
        "in-imported-upstream {\
        \n  a: b;\
        \n}\
        \nin-used {\
        \n  a: b;\
        \n}\
        \nin-imported-downstream {\
        \n  a: b;\
        \n}\
        \nin-input {\
        \n  a: b;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn import_module_imported_by_use() {
    assert_eq!(
        crate::rsass(
            "@use \"used\";\
            \n@import \"shared\";\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c;\
        \n}\
        \na {\
        \n  b: c;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn nested_import_into_use() {
    assert_eq!(
        crate::rsass(
            "outer {@import \"imported\"}\
            \n"
        )
        .unwrap(),
        "outer in-used {\
        \n  parent: (in-used,);\
        \n}\
        \nouter in-imported {\
        \n  parent: (outer in-imported,);\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn use_and_import_same() {
    assert_eq!(
        crate::rsass(
            "@use \"other\";\
            \n\
            \n// @import always duplicates CSS, even when that CSS has been @used. In other\
            \n// words, @import\'s duplication takes precedence over @use\'s load-once policy.\
            \n@import \"other\";\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c;\
        \n}\
        \na {\
        \n  b: c;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn use_into_import() {
    assert_eq!(
        crate::rsass(
            "@use \"used\";\
            \n\
            \nin-input {a: b}\
            \n"
        )
        .unwrap(),
        "in-imported {\
        \n  a: b;\
        \n}\
        \nin-used {\
        \n  a: b;\
        \n}\
        \nin-input {\
        \n  a: b;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn use_into_import_into_use() {
    assert_eq!(
        crate::rsass(
            "@use \"used-downstream\";\
            \n\
            \nin-input {a: b}\
            \n"
        )
        .unwrap(),
        "in-used-upstream {\
        \n  a: b;\
        \n}\
        \nin-imported {\
        \n  a: b;\
        \n}\
        \nin-used-downstream {\
        \n  a: b;\
        \n}\
        \nin-input {\
        \n  a: b;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn use_module_used_by_import() {
    assert_eq!(
        crate::rsass(
            "@use \"shared\";\
            \n@import \"imported\";\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c;\
        \n}\
        \na {\
        \n  b: c;\
        \n}\
        \n"
    );
}
