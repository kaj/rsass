//! Tests auto-converted from "sass-spec/spec/directives/use/css/order.hrx"

#[test]
#[ignore] // unexepected error
fn diamond() {
    assert_eq!(
        crate::rsass(
            "@use \"left\";\
            \n@use \"right\";\
            \n\
            \na {file: input}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  file: shared;\
        \n}\
        \na {\
        \n  file: left;\
        \n}\
        \na {\
        \n  file: right;\
        \n}\
        \na {\
        \n  file: input;\
        \n}\
        \n"
    );
}
mod import_order {
    #[test]
    #[ignore] // unexepected error
    fn comments_and_imports() {
        assert_eq!(
            crate::rsass(
                "/* input comment before use */\
            \n@use \"midstream\";\
            \n\
            \n/* input comment before import */\
            \n@import \"input.css\";\
            \n\
            \n/* input comment after import */\
            \n"
            )
            .unwrap(),
            "/* upstream comment before import */\
        \n@import \"upstream.css\";\
        \n/* midstream comment before use */\
        \n/* midstream comment before first import */\
        \n@import \"midstream1.css\";\
        \n/* midstream comment before second import */\
        \n@import \"midstream2.css\";\
        \n/* input comment before use */\
        \n/* input comment before import */\
        \n@import \"input.css\";\
        \n/* upstream comment after import */\
        \n/* midstream comment after imports */\
        \na {\
        \n  file: midstream;\
        \n}\
        \n/* input comment after import */\
        \n"
        );
    }
    mod import_into_use {
        #[test]
        #[ignore] // wrong result
        fn css_import_above_rule() {
            assert_eq!(
                crate::rsass(
                    "@import \"imported\";\
            \n\
            \n@import \"input.css\";\
            \n"
                )
                .unwrap(),
                "@import \"used.css\";\
        \n@import \"imported.css\";\
        \n@import \"input.css\";\
        \na {\
        \n  file: used;\
        \n}\
        \na {\
        \n  file: imported;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn css_import_below_rule() {
            assert_eq!(
                crate::rsass(
                    "@import \"imported\";\
            \n\
            \n@import \"input.css\";\
            \n"
                )
                .unwrap(),
                "@import \"used.css\";\
        \n@import \"imported.css\";\
        \n@import \"input.css\";\
        \na {\
        \n  file: used;\
        \n}\
        \na {\
        \n  file: imported;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn sass_import_below_css_import() {
            assert_eq!(
                crate::rsass(
                    "@import \"input.css\";\
            \n\
            \n@import \"imported\";\
            \n"
                )
                .unwrap(),
                "@import \"input.css\";\
        \n@import \"used.css\";\
        \n@import \"imported.css\";\
        \n"
            );
        }
    }
    mod use_into_import {
        #[test]
        #[ignore] // unexepected error
        fn css_import_above_rule() {
            assert_eq!(
                crate::rsass(
                    "@use \"used\";\
            \n\
            \n@import \"input.css\";\
            \n"
                )
                .unwrap(),
                "@import \"imported.css\";\
        \n@import \"used.css\";\
        \n@import \"input.css\";\
        \na {\
        \n  file: imported;\
        \n}\
        \na {\
        \n  file: used;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn css_import_below_rule() {
            assert_eq!(
                crate::rsass(
                    "@use \"used\";\
            \n\
            \n@import \"input.css\";\
            \n"
                )
                .unwrap(),
                "@import \"imported.css\";\
        \n@import \"used.css\";\
        \n@import \"input.css\";\
        \na {\
        \n  file: imported;\
        \n}\
        \na {\
        \n  file: used;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn sass_import_below_css_import() {
            assert_eq!(
                crate::rsass(
                    "@use \"used\";\
            \n\
            \n@import \"input.css\";\
            \n"
                )
                .unwrap(),
                "@import \"used.css\";\
        \n@import \"imported.css\";\
        \n@import \"input.css\";\
        \n"
            );
        }
    }
    mod use_into_use {
        #[test]
        #[ignore] // unexepected error
        fn import_above_rule() {
            assert_eq!(
                crate::rsass(
                    "@use \"midstream\";\
            \n\
            \n@import \"input.css\";\
            \n"
                )
                .unwrap(),
                "@import \"upstream.css\";\
        \n@import \"midstream.css\";\
        \n@import \"input.css\";\
        \na {\
        \n  file: upstream;\
        \n}\
        \na {\
        \n  file: midstream;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn import_below_rule() {
            assert_eq!(
                crate::rsass(
                    "@use \"midstream\";\
            \n\
            \n@import \"input.css\";\
            \n"
                )
                .unwrap(),
                "@import \"upstream.css\";\
        \n@import \"midstream.css\";\
        \n@import \"input.css\";\
        \na {\
        \n  file: upstream;\
        \n}\
        \na {\
        \n  file: midstream;\
        \n}\
        \n"
            );
        }
    }
}
#[test]
#[ignore] // unexepected error
fn once() {
    assert_eq!(
        crate::rsass(
            "@use \"other\" as o1;\
            \n@use \"other\" as o2;\
            \n@use \"other\" as o3;\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: c;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn triangle() {
    assert_eq!(
        crate::rsass(
            "@use \"midstream\";\
            \n@use \"upstream\";\
            \n\
            \na {file: input}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  file: upstream;\
        \n}\
        \na {\
        \n  file: midstream;\
        \n}\
        \na {\
        \n  file: input;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn unrelated_branches() {
    assert_eq!(
        crate::rsass(
            "@use \"left_midstream\";\
            \n@use \"right_midstream\";\
            \n\
            \na {file: input}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  file: left upstream;\
        \n}\
        \na {\
        \n  file: left midstream;\
        \n}\
        \na {\
        \n  file: right upstream;\
        \n}\
        \na {\
        \n  file: right midstream;\
        \n}\
        \na {\
        \n  file: input;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn use_into_use() {
    assert_eq!(
        crate::rsass(
            "@use \"midstream\";\
            \n\
            \na {file: input}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  file: upstream;\
        \n}\
        \na {\
        \n  file: midstream;\
        \n}\
        \na {\
        \n  file: input;\
        \n}\
        \n"
    );
}
#[test]
#[ignore] // unexepected error
fn use_order() {
    assert_eq!(
        crate::rsass(
            "@use \"other1\";\
            \n@use \"other2\";\
            \n@use \"other3\";\
            \n\
            \na {file: input}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  file: other1;\
        \n}\
        \na {\
        \n  file: other2;\
        \n}\
        \na {\
        \n  file: other3;\
        \n}\
        \na {\
        \n  file: input;\
        \n}\
        \n"
    );
}
