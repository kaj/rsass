//! Tests auto-converted from "sass-spec/spec/directives/import/error"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/directives/import/error/conflict.hrx"
mod conflict {
    #[allow(unused)]
    use super::rsass;

    // Ignoring "all", error tests are not supported yet.

    // Ignoring "extension", error tests are not supported yet.
    mod import_only {
        #[allow(unused)]
        use super::rsass;

        // Ignoring "no_extension", error tests are not supported yet.

        // Ignoring "with_extension", error tests are not supported yet.
    }

    // Ignoring "index", error tests are not supported yet.

    // Ignoring "partial", error tests are not supported yet.
}

// From "sass-spec/spec/directives/import/error/member.hrx"
mod member {
    #[allow(unused)]
    use super::rsass;
    mod inaccessible {
        #[allow(unused)]
        use super::rsass;
        mod nested {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // unexepected error
            fn function() {
                assert_eq!(
                    rsass(
                        "a {@import \"other\"}\
            \n\
            \nb {c: d()}\
            \n"
                    )
                    .unwrap(),
                    "b {\
        \n  c: d();\
        \n}\
        \n"
                );
            }

            // Ignoring "mixin", error tests are not supported yet.

            // Ignoring "variable", error tests are not supported yet.
        }
    }
}

// From "sass-spec/spec/directives/import/error/not_found.hrx"
mod not_found {
    #[allow(unused)]
    use super::rsass;

    // Ignoring "directory_dot_import", error tests are not supported yet.

    // Ignoring "no_extension", error tests are not supported yet.

    // Ignoring "parent_relative", error tests are not supported yet.
}
