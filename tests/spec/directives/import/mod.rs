//! Tests auto-converted from "sass-spec/spec/directives/import"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/directives/import/configuration.hrx"
mod configuration {
    #[allow(unused)]
    use super::rsass;
    mod import_twice {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn no_change() {
            assert_eq!(
                rsass(
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
                rsass(
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
                rsass(
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
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn through_forward() {
            assert_eq!(
                rsass(
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
                rsass(
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
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn no_config() {
            assert_eq!(
                rsass(
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
                rsass(
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
    #[ignore] // unexepected error
    fn nested() {
        assert_eq!(
            rsass(
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
            rsass(
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
            rsass(
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
            rsass(
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
            rsass(
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
}

// From "sass-spec/spec/directives/import/css.hrx"
mod css {
    #[allow(unused)]
    use super::rsass;
}

mod error;

// From "sass-spec/spec/directives/import/load.hrx"
mod load {
    #[allow(unused)]
    use super::rsass;
    mod explicit_extension {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // unexepected error
        fn sass() {
            assert_eq!(
                rsass(
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
        #[ignore] // unexepected error
        fn scss() {
            assert_eq!(
                rsass(
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
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn dir_dot_foo() {
            assert_eq!(
                rsass(
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
                rsass(
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
                rsass(
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
                rsass(
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
        #[allow(unused)]
        use super::rsass;
        mod import_only {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn before_index() {
                assert_eq!(
        rsass(
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
                    rsass(
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
        rsass(
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
        rsass(
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
        rsass(
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
        rsass(
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
        rsass(
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
                rsass(
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
                rsass(
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
                rsass(
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
}

// From "sass-spec/spec/directives/import/nested.hrx"
mod nested {
    #[allow(unused)]
    use super::rsass;
    mod at_rule {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // unexepected error
        fn childless() {
            assert_eq!(
                rsass(
                    "a {@import \"other\"}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  @b c;\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn declaration_child() {
            assert_eq!(
                rsass(
                    "a {@import \"other\"}\
            \n"
                )
                .unwrap(),
                "@b {\
        \n  a {\
        \n    c: d;\
        \n  }\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn keyframes() {
            assert_eq!(
                rsass(
                    "a {@import \"other\"}\
            \n"
                )
                .unwrap(),
                "@keyframes b {\
        \n  0% {\
        \n    c: d;\
        \n  }\
        \n}\
        \n"
            );
        }
        #[test]
        #[ignore] // unexepected error
        fn rule_child() {
            assert_eq!(
                rsass(
                    "a {@import \"other\"}\
            \n"
                )
                .unwrap(),
                "@b {\
        \n  a c {\
        \n    d: e;\
        \n  }\
        \n}\
        \n"
            );
        }
    }
    mod scope {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // unexepected error
        fn function() {
            assert_eq!(
        rsass(
            ".parent {\
            \n  // This should be visible to the imported stylesheet. There\'s not really a\
            \n  // good reason for this, but it\'s the historical behavior so whatever.\
            \n  @function local() {\
            \n    @return value;\
            \n  }\
            \n\
            \n  @import \'other\';\
            \n}\
            \n"
        )
        .unwrap(),
        ".parent x {\
        \n  function: value;\
        \n}\
        \n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn mixin() {
            assert_eq!(
        rsass(
            ".parent {\
            \n  // This should be visible to the imported stylesheet. There\'s not really a\
            \n  // good reason for this, but it\'s the historical behavior so whatever.\
            \n  @mixin local {\
            \n    x {y: z}\
            \n  }\
            \n\
            \n  @import \'other\';\
            \n}\
            \n"
        )
        .unwrap(),
        ".parent x {\
        \n  y: z;\
        \n}\
        \n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn variable() {
            assert_eq!(
        rsass(
            ".parent {\
            \n  // This should be visible to the imported stylesheet. There\'s not really a\
            \n  // good reason for this, but it\'s the historical behavior so whatever.\
            \n  $var: value;\
            \n  @import \'other\';\
            \n}\
            \n"
        )
        .unwrap(),
        ".parent x {\
        \n  var: value;\
        \n}\
        \n"
    );
        }
    }
}
