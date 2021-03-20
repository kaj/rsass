//! Tests auto-converted from "sass-spec/spec/directives/import/error/member.hrx"

mod inaccessible {
    mod nested {
        #[test]
        #[ignore] // unexepected error
        fn function() {
            assert_eq!(
                crate::rsass(
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
