//! Tests auto-converted from "sass-spec/spec/directives/import/configuration.hrx"

mod import_twice {
    #[test]
    #[ignore] // wrong result
    fn no_change() {
        assert_eq!(
            crate::rsass(
                "$a: configured;\
            \n@import \"other\";\
            \n@import \"other\";\
            \n"
            )
            .unwrap(),
            "b {\
        \n  c: configured;\
        \n}\
        \nb {\
        \n  c: configured;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn still_changes_in_same_file() {
        assert_eq!(
            crate::rsass(
                "@import \"other\";\
            \n$a: changed;\
            \n@import \"other\";\
            \n\
            \nd {\
            \n  e: $a;\
            \n}\
            \n"
            )
            .unwrap(),
            "b {\
        \n  c: original;\
        \n}\
        \nb {\
        \n  c: original;\
        \n}\
        \nd {\
        \n  e: changed;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn with_change() {
        assert_eq!(
            crate::rsass(
                "$a: configured;\
            \n@import \"other\";\
            \n$a: changed; // This should be ignored\
            \n@import \"other\";\
            \n"
            )
            .unwrap(),
            "b {\
        \n  c: configured;\
        \n}\
        \nb {\
        \n  c: configured;\
        \n}\
        \n"
        );
    }
}
mod indirect {
    #[test]
    #[ignore] // wrong result
    fn through_forward() {
        assert_eq!(
            crate::rsass(
                "$a: configured;\
            \n@import \"midstream\";\
            \n"
            )
            .unwrap(),
            "b {\
        \n  c: configured;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn through_import() {
        assert_eq!(
            crate::rsass(
                "$a: configured;\
            \n@import \"midstream\";\
            \n"
            )
            .unwrap(),
            "b {\
        \n  c: configured;\
        \n}\
        \n"
        );
    }
}
mod midstream_definition {
    #[test]
    #[ignore] // wrong result
    fn no_config() {
        assert_eq!(
            crate::rsass(
                "@import \"midstream\";\
            \n"
            )
            .unwrap(),
            "b {\
        \n  c: original;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn with_config() {
        assert_eq!(
            crate::rsass(
                "$a: configured;\
            \n@import \"midstream\";\
            \n"
            )
            .unwrap(),
            "b {\
        \n  c: configured;\
        \n}\
        \n"
        );
    }
}
#[test]
#[ignore] // wrong result
fn nested() {
    assert_eq!(
        crate::rsass(
            "a {\
            \n  $a: configured;\
            \n  @import \"midstream\";\
            \n}\
            \n"
        )
        .unwrap(),
        "a b {\
        \n  c: configured;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // wrong result
fn prefixed_as() {
    assert_eq!(
        crate::rsass(
            "$d-a: configured;\
            \n@import \"midstream\";\
            \n"
        )
        .unwrap(),
        "b {\
        \n  c: configured;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // wrong result
fn same_file() {
    assert_eq!(
        crate::rsass(
            "$a: configured;\
            \n@import \"midstream\";\
            \n"
        )
        .unwrap(),
        "b {\
        \n  c: configured;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // wrong result
fn separate_file() {
    assert_eq!(
        crate::rsass(
            "@import \"config\";\
            \n@import \"midstream\";\
            \n"
        )
        .unwrap(),
        "b {\
        \n  c: configured;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // wrong result
fn unrelated_variable() {
    assert_eq!(
        crate::rsass(
            "$a: configured;\
            \n$d: other;\
            \n@import \"midstream\";\
            \n"
        )
        .unwrap(),
        "b {\
        \n  c: configured;\
        \n}\
        \n"
    );
}
