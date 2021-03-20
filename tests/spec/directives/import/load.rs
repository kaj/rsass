//! Tests auto-converted from "sass-spec/spec/directives/import/load.hrx"

mod explicit_extension {
    #[test]
    #[ignore] // wrong result
    fn sass() {
        assert_eq!(
            crate::rsass(
                "@import \"other.sass\"\
            \n"
            )
            .unwrap(),
            "a {\
        \n  syntax: sass;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn scss() {
        assert_eq!(
            crate::rsass(
                "@import \"other.scss\"\
            \n"
            )
            .unwrap(),
            "a {\
        \n  syntax: scss;\
        \n}\
        \n"
        );
    }
}
mod index {
    #[test]
    #[ignore] // wrong result
    fn dir_dot_foo() {
        assert_eq!(
            crate::rsass(
                "@import \"dir.foo\";\
            \n"
            )
            .unwrap(),
            ".foo {\
        \n  a: b;\
        \n}\
        \n"
        );
    }

    // Ignoring "dir_dot_scss", error tests are not supported yet.
    #[test]
    #[ignore] // wrong result
    fn partial() {
        assert_eq!(
            crate::rsass(
                "@import \"dir\";\
            \n"
            )
            .unwrap(),
            ".foo {\
        \n  a: b;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn sass() {
        assert_eq!(
            crate::rsass(
                "@import \"dir\";\
            \n"
            )
            .unwrap(),
            ".foo {\
        \n  a: b;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn scss() {
        assert_eq!(
            crate::rsass(
                "@import \"dir\";\
            \n"
            )
            .unwrap(),
            ".foo {\
        \n  a: b;\
        \n}\
        \n"
        );
    }
}
mod precedence {
    mod import_only {
        #[test]
        #[ignore] // wrong result
        fn before_index() {
            assert_eq!(
        crate::rsass(
            "// A non-index import-only file takes precedence over an index file.\
            \n@import \"other\";\
            \n"
        )
        .unwrap(),
        "a {\
        \n  import-only: true;\
        \n}\
        \n"
    );
        }
        #[test]
        #[ignore] // wrong result
        fn explicit_extension() {
            assert_eq!(
                crate::rsass(
                    "@import \"other\";\
            \n"
                )
                .unwrap(),
                "a {\
        \n  import-only: true;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn implicit_extension() {
            assert_eq!(
        crate::rsass(
            "// The extension of the import-only file doesn\'t need to match the extension of\
            \n// the use-only file.\
            \n@import \"other\";\
            \n"
        )
        .unwrap(),
        "a {\
        \n  import-only: true;\
        \n}\
        \n"
    );
        }
        #[test]
        #[ignore] // wrong result
        fn index() {
            assert_eq!(
        crate::rsass(
            "// A import-only index file takes precedence over a normal index file.\
            \n@import \"other\";\
            \n"
        )
        .unwrap(),
        "a {\
        \n  import-only: true;\
        \n}\
        \n"
    );
        }
        #[test]
        #[ignore] // wrong result
        fn index_after_normal() {
            assert_eq!(
        crate::rsass(
            "// Index files, even import-only ones, always come after non-index files.\
            \n@import \"other\";\
            \n"
        )
        .unwrap(),
        "a {\
        \n  import-only: false;\
        \n}\
        \n"
    );
        }
        #[test]
        #[ignore] // wrong result
        fn normal_before_partial() {
            assert_eq!(
        crate::rsass(
            "// A normal import-only file takes precedence over a non-import-only partial.\
            \n@import \"other\";\
            \n"
        )
        .unwrap(),
        "a {\
        \n  import-only: true;\
        \n}\
        \n"
    );
        }
        #[test]
        #[ignore] // wrong result
        fn partial_before_normal() {
            assert_eq!(
        crate::rsass(
            "// An import-only partial takes precedence over a normal non-import-only file.\
            \n@import \"other\";\
            \n"
        )
        .unwrap(),
        "a {\
        \n  import-only: true;\
        \n}\
        \n"
    );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn normal_before_index() {
        assert_eq!(
            crate::rsass(
                "@import \"dir\";\
            \n"
            )
            .unwrap(),
            "a {\
        \n  index: false;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn sass_before_css() {
        assert_eq!(
            crate::rsass(
                "@import \"other\";\
            \n"
            )
            .unwrap(),
            "a {\
        \n  syntax: sass;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn scss_before_css() {
        assert_eq!(
            crate::rsass(
                "@import \"other\";\
            \n"
            )
            .unwrap(),
            "a {\
        \n  syntax: scss;\
        \n}\
        \n"
        );
    }
}
