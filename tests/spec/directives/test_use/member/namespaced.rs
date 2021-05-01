//! Tests auto-converted from "sass-spec/spec/directives/use/member/namespaced.hrx"

mod default {
    #[test]
    #[ignore] // unexepected error
    fn basename() {
        assert_eq!(
        crate::rsass(
            "// Only the basename of the URL is used for the namespace. Previous components\
            \n// are discarded.\
            \n@use \"foo/bar/../baz/qux/other\";\
            \n\
            \na {b: other.$variable}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: value;\
        \n}\
        \n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn function() {
        assert_eq!(
            crate::rsass(
                "@use \"other\";\
            \n\
            \na {b: other.member()}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: value;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn mixin() {
        assert_eq!(
            crate::rsass(
                "@use \"other\";\
            \n\
            \n@include other.member;\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: c;\
        \n}\
        \n"
        );
    }
    mod variable_assignment {
        #[test]
        #[ignore] // unexepected error
        fn in_declaration() {
            assert_eq!(
        crate::rsass(
            "@use \"other\";\
            \n\
            \na {\
            \n  b: {\
            \n    // Test assignments within a declaration specially, because declarations\
            \n    // disallow style rules and variable assignments need to be disambiguated\
            \n    // with those.\
            \n    other.$member: new value;\
            \n\
            \n    c: other.get-member();\
            \n  }\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b-c: new value;\
        \n}\
        \n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn in_function() {
            assert_eq!(
        crate::rsass(
            "@use \"other\";\
            \n\
            \n@function a() {\
            \n  // Test assignments within a function specially, because functions disallow\
            \n  // property declarations and variable assignments need to be disambiguated\
            \n  // with those.\
            \n  other.$member: new value;\
            \n\
            \n  @return other.get-member();\
            \n}\
            \n\
            \nb {c: a()}\
            \n"
        )
        .unwrap(),
        "b {\
        \n  c: new value;\
        \n}\
        \n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn nested() {
            assert_eq!(
        crate::rsass(
            "@use \"other\";\
            \n\
            \na {\
            \n  // Namespaced assignments always assign to the other module\'s variable, even\
            \n  // if they\'re nested in a block scope.\
            \n  other.$member: new value;\
            \n\
            \n  b: other.get-member();\
            \n}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: new value;\
        \n}\
        \n"
    );
        }
        #[test]
        #[ignore] // unexepected error
        fn top_level() {
            assert_eq!(
                crate::rsass(
                    "@use \"other\";\
            \n\
            \nother.$member: new value;\
            \n\
            \na {b: other.get-member()};\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: new value;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    #[ignore] // unexepected error
    fn variable_use() {
        assert_eq!(
            crate::rsass(
                "@use \"other\";\
            \n\
            \na {b: other.$member}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: value;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn without_extensions() {
        assert_eq!(
        crate::rsass(
            "// All extensions on the URL are discarded before determining the namespace.\
            \n@use \"other.foo.bar.baz.scss\";\
            \n\
            \na {b: other.$variable}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: value;\
        \n}\
        \n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn without_underscore() {
        assert_eq!(
        crate::rsass(
            "// A single leading underscore is removed before determining the namespace.\
            \n@use \"_other\";\
            \n\
            \na {b: other.$variable}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: value;\
        \n}\
        \n"
    );
    }
}
mod explicit {
    #[test]
    #[ignore] // unexepected error
    fn function() {
        assert_eq!(
            crate::rsass(
                "@use \"other\" as o;\
            \n\
            \na {b: o.member()}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: value;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn mixin() {
        assert_eq!(
            crate::rsass(
                "@use \"other\" as o;\
            \n\
            \n@include o.member;\
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
    fn variable_assignment() {
        assert_eq!(
            crate::rsass(
                "@use \"other\" as o;\
            \n\
            \no.$member: new value;\
            \n\
            \na {b: o.get-member()}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: new value;\
        \n}\
        \n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn variable_use() {
        assert_eq!(
            crate::rsass(
                "@use \"other\" as o;\
            \n\
            \na {b: o.$member}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: value;\
        \n}\
        \n"
        );
    }
}
