//! Tests auto-converted from "sass-spec/spec/core_functions/selector/is_superselector/simple/pseudo/selector_arg"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

// From "sass-spec/spec/core_functions/selector/is_superselector/simple/pseudo/selector_arg/any.hrx"
mod any {
    #[allow(unused)]
    use super::rsass;
    mod prefix {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn subset() {
            assert_eq!(
        rsass(
            "a {b: is-superselector(\":-pfx-any(c d.i, e j f)\", \"c d, e f, g h\")}\
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
        #[ignore] // wrong result
        fn superset() {
            assert_eq!(
        rsass(
            "a {b: is-superselector(\":-pfx-any(c d, e f, g h)\", \"c d.i, e j f\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn subset() {
        assert_eq!(
        rsass(
            "a {b: is-superselector(\":any(c d.i, e j f)\", \"c d, e f, g h\")}\
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
    #[ignore] // wrong result
    fn superset() {
        assert_eq!(
        rsass(
            "a {b: is-superselector(\":any(c d, e f, g h)\", \"c d.i, e j f\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
    }
}

// From "sass-spec/spec/core_functions/selector/is_superselector/simple/pseudo/selector_arg/current.hrx"
mod current {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn bare_sub() {
        assert_eq!(
        rsass(
            "a {b: is-superselector(\":current(c d, e f)\", \"c d, e f\")}\
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
    #[ignore] // wrong result
    fn equal() {
        assert_eq!(
        rsass(
            "a {b: is-superselector(\":current(c d, e f)\", \":current(c d, e f)\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
    }
    mod prefix {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn equal() {
            assert_eq!(
        rsass(
            "a {b: is-superselector(\":-pfx-current(c d, e f)\", \":-pfx-current(c d, e f)\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
        }
        #[test]
        #[ignore] // wrong result
        fn subset() {
            assert_eq!(
                rsass(
                    "a {\
                     \n  b: is-superselector(\
                     \n      \":-pfx-current(c d.i, e j f)\",\
                     \n      \":-pfx-current(c d, e f, g h)\");\
                     \n}\
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
        #[ignore] // wrong result
        fn superset() {
            assert_eq!(
                rsass(
                    "a {\
                     \n  b: is-superselector(\
                     \n      \":-pfx-current(c d, e f, g h)\",\
                     \n      \":-pfx-current(c d.i, e j f)\");\
                     \n}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: false;\
                 \n}\
                 \n"
            );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn subset() {
        assert_eq!(
        rsass(
            "a {b: is-superselector(\":current(c d.i, e j f)\", \":current(c d, e f, g h)\")}\
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
    #[ignore] // wrong result
    fn superset() {
        assert_eq!(
        rsass(
            "a {b: is-superselector(\":current(c d, e f, g h)\", \":current(c d.i, e j f)\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: false;\
        \n}\
        \n"
    );
    }
}

// From "sass-spec/spec/core_functions/selector/is_superselector/simple/pseudo/selector_arg/has.hrx"
mod has {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn bare_sub() {
        assert_eq!(
        rsass(
            "a {b: is-superselector(\":has(c d, e f, g h)\", \"c d, e f, g h\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: false;\
        \n}\
        \n"
    );
    }
    mod prefix {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn subset() {
            assert_eq!(
        rsass(
            "a {b: is-superselector(\":-pfx-has(c d.i, e j f)\", \":-pfx-has(c d, e f, g h)\")}\
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
        #[ignore] // wrong result
        fn superset() {
            assert_eq!(
        rsass(
            "a {b: is-superselector(\":-pfx-has(c d, e f, g h)\", \":-pfx-has(c d.i, e j f)\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn subset() {
        assert_eq!(
        rsass(
            "a {b: is-superselector(\":has(c d.i, e j f)\", \":has(c d, e f, g h)\")}\
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
    #[ignore] // wrong result
    fn superset() {
        assert_eq!(
        rsass(
            "a {b: is-superselector(\":has(c d, e f, g h)\", \":has(c d.i, e j f)\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
    }
}

// From "sass-spec/spec/core_functions/selector/is_superselector/simple/pseudo/selector_arg/host.hrx"
mod host {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn bare_sub() {
        assert_eq!(
        rsass(
            "a {b: is-superselector(\":host(c d, e f, g h)\", \"c d, e f, g h\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: false;\
        \n}\
        \n"
    );
    }
    mod prefix {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn subset() {
            assert_eq!(
        rsass(
            "a {b: is-superselector(\":-pfx-host(c d.i, e j f)\", \":-pfx-host(c d, e f, g h)\")}\
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
        #[ignore] // wrong result
        fn superset() {
            assert_eq!(
        rsass(
            "a {b: is-superselector(\":-pfx-host(c d, e f, g h)\", \":-pfx-host(c d.i, e j f)\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn subset() {
        assert_eq!(
        rsass(
            "a {b: is-superselector(\":host(c d.i, e j f)\", \":host(c d, e f, g h)\")}\
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
    #[ignore] // wrong result
    fn superset() {
        assert_eq!(
        rsass(
            "a {b: is-superselector(\":host(c d, e f, g h)\", \":host(c d.i, e j f)\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
    }
}

// From "sass-spec/spec/core_functions/selector/is_superselector/simple/pseudo/selector_arg/host_context.hrx"
mod host_context {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn bare_sub() {
        assert_eq!(
        rsass(
            "a {b: is-superselector(\":host-context(c d, e f, g h)\", \"c d, e f, g h\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: false;\
        \n}\
        \n"
    );
    }
    mod prefix {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn subset() {
            assert_eq!(
        rsass(
            "a {b: is-superselector(\":-pfx-host-context(c d.i, e j f)\", \":-pfx-host-context(c d, e f, g h)\")}\
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
        #[ignore] // wrong result
        fn superset() {
            assert_eq!(
        rsass(
            "a {b: is-superselector(\":-pfx-host-context(c d, e f, g h)\", \":-pfx-host-context(c d.i, e j f)\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn subset() {
        assert_eq!(
        rsass(
            "a {b: is-superselector(\":host-context(c d.i, e j f)\", \":host-context(c d, e f, g h)\")}\
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
    #[ignore] // wrong result
    fn superset() {
        assert_eq!(
        rsass(
            "a {b: is-superselector(\":host-context(c d, e f, g h)\", \":host-context(c d.i, e j f)\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
    }
}

// From "sass-spec/spec/core_functions/selector/is_superselector/simple/pseudo/selector_arg/matches.hrx"
mod matches {
    #[allow(unused)]
    use super::rsass;
    mod both {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn subset() {
            assert_eq!(
        rsass(
            "a {b: is-superselector(\":matches(c d.i, e j f)\", \":matches(c d, e f, g h)\")}\
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
        #[ignore] // wrong result
        fn superset() {
            assert_eq!(
        rsass(
            "a {b: is-superselector(\":matches(c d, e f, g h)\", \":matches(c d.i, e j f)\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
        }
    }
    mod complex {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn subset() {
            assert_eq!(
                rsass(
                    "a {b: is-superselector(\":matches(c d e)\", \"c e\")}\
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
        #[ignore] // wrong result
        fn superset() {
            assert_eq!(
                rsass(
                    "a {b: is-superselector(\":matches(c e)\", \"c d e\")}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: true;\
                 \n}\
                 \n"
            );
        }
    }
    mod compound {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn subset() {
            assert_eq!(
                rsass(
                    "a {b: is-superselector(\":matches(c.d.e)\", \"c e\")}\
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
        #[ignore] // wrong result
        fn superset() {
            assert_eq!(
                rsass(
                    "a {b: is-superselector(\":matches(c.e)\", \"c.d.e\")}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: true;\
                 \n}\
                 \n"
            );
        }
    }
    mod list {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn subset() {
            assert_eq!(
        rsass(
            "a {b: is-superselector(\":matches(c d, e f)\", \"c d, e f, g h\")}\
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
        #[ignore] // wrong result
        fn superset() {
            assert_eq!(
        rsass(
            "a {b: is-superselector(\":matches(c d, e f, g h)\", \"c d, e f\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
        }
    }
    mod not_superselector_of {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn any() {
            assert_eq!(
        rsass(
            "a {b: is-superselector(\":matches(c, d)\", \":any(c, d)\")}\
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
        #[ignore] // wrong result
        fn prefixed() {
            assert_eq!(
        rsass(
            "a {b: is-superselector(\":matches(c, d)\", \":-pfx-matches(c, d)\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: false;\
        \n}\
        \n"
    );
        }
    }
    mod prefix {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn subset() {
            assert_eq!(
        rsass(
            "a {b: is-superselector(\":-pfx-matches(c d.i, e j f)\", \"c d, e f, g h\")}\
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
        #[ignore] // wrong result
        fn superset() {
            assert_eq!(
        rsass(
            "a {b: is-superselector(\":-pfx-matches(c d, e f, g h)\", \"c d.i, e j f\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
        }
    }
    mod simple {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn equal() {
            assert_eq!(
                rsass(
                    "a {b: is-superselector(\":matches(c)\", \"c\")}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: true;\
                 \n}\
                 \n"
            );
        }
        #[test]
        #[ignore] // wrong result
        fn unequal() {
            assert_eq!(
                rsass(
                    "a {b: is-superselector(\":matches(c)\", \"d\")}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: false;\
                 \n}\
                 \n"
            );
        }
    }
}

// From "sass-spec/spec/core_functions/selector/is_superselector/simple/pseudo/selector_arg/not.hrx"
mod not {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn bare_sub() {
        assert_eq!(
        rsass(
            "a {b: is-superselector(\":not(c d, e f, g h)\", \"c d, e f, g h\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: false;\
        \n}\
        \n"
    );
    }
    mod equivalence {
        #[allow(unused)]
        use super::rsass;
        mod split_sub {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn subset() {
                assert_eq!(
        rsass(
            "a {b: is-superselector(\":not(c d.i, e j f)\", \":not(c d):not(e f):not(g h)\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
            }
            #[test]
            #[ignore] // wrong result
            fn superset() {
                assert_eq!(
        rsass(
            "a {b: is-superselector(\":not(c d, e f, g h)\", \":not(c d.i):not(e j f)\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: false;\
        \n}\
        \n"
    );
            }
        }
        mod split_super {
            #[allow(unused)]
            use super::rsass;
            #[test]
            #[ignore] // wrong result
            fn subset() {
                assert_eq!(
        rsass(
            "a {b: is-superselector(\":not(c d.i):not(e j f)\", \":not(c d, e f, g h)\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
            }
            #[test]
            #[ignore] // wrong result
            fn superset() {
                assert_eq!(
        rsass(
            "a {b: is-superselector(\":not(c d):not(e f):not(g h)\", \":not(c d.i, e j f)\")}"
        )
        .unwrap(),
        "a {\
        \n  b: false;\
        \n}\
        \n"
    );
            }
        }
    }
    #[test]
    #[ignore] // wrong result
    fn id() {
        assert_eq!(
            rsass(
                "a {b: is-superselector(\":not(#c.d)\", \"#e\")}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: true;\
             \n}\
             \n"
        );
    }
    mod prefix {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn subset() {
            assert_eq!(
        rsass(
            "a {b: is-superselector(\":-pfx-not(c d.i, e j f)\", \":-pfx-not(c d, e f, g h)\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
        }
        #[test]
        #[ignore] // wrong result
        fn superset() {
            assert_eq!(
        rsass(
            "a {b: is-superselector(\":-pfx-not(c d, e f, g h)\", \":-pfx-not(c d.i, e j f)\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: false;\
        \n}\
        \n"
    );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn subset() {
        assert_eq!(
        rsass(
            "a {b: is-superselector(\":not(c d.i, e j f)\", \":not(c d, e f, g h)\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
    }
    #[test]
    #[ignore] // wrong result
    fn superset() {
        assert_eq!(
        rsass(
            "a {b: is-superselector(\":not(c d, e f, g h)\", \":not(c d.i, e j f)\")}\
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
    #[ignore] // wrong result
    fn test_type() {
        assert_eq!(
            rsass(
                "a {b: is-superselector(\":not(c.d)\", \"e\")}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: true;\
             \n}\
             \n"
        );
    }
}

// From "sass-spec/spec/core_functions/selector/is_superselector/simple/pseudo/selector_arg/nth_child.hrx"
mod nth_child {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn bare_sub() {
        assert_eq!(
        rsass(
            "a {b: is-superselector(\":nth-child(n+1 of c d, e f, g h)\", \"c d, e f, g h\")}\
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
    #[ignore] // wrong result
    fn bare_super() {
        assert_eq!(
            rsass(
                "a {b: is-superselector(\"c\", \":nth-child(n+1 of c)\")}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: true;\
             \n}\
             \n"
        );
    }
    #[test]
    #[ignore] // wrong result
    fn different_arg() {
        assert_eq!(
        rsass(
            "a {b: is-superselector(\":nth-child(n+1 of c)\", \":nth-child(n+2 of c)\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: false;\
        \n}\
        \n"
    );
    }
    mod prefix {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn subset() {
            assert_eq!(
                rsass(
                    "a {\
                     \n  b: is-superselector(\
                     \n      \":-pfx-nth-child(n+1 of c d.i, e j f)\",\
                     \n      \":-pfx-nth-child(n+1 of c d, e f, g h)\");\
                     \n}\
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
        #[ignore] // wrong result
        fn superset() {
            assert_eq!(
                rsass(
                    "a {\
                     \n  b: is-superselector(\
                     \n      \":-pfx-nth-child(n+1 of c d, e f, g h)\",\
                     \n      \":-pfx-nth-child(n+1 of c d.i, e j f)\");\
                     \n}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: true;\
                 \n}\
                 \n"
            );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn subset() {
        assert_eq!(
            rsass(
                "a {\
                 \n  b: is-superselector(\
                 \n      \":nth-child(n+1 of c d.i, e j f)\",\
                 \n      \":nth-child(n+1 of c d, e f, g h)\");\
                 \n}\
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
    #[ignore] // wrong result
    fn superset() {
        assert_eq!(
            rsass(
                "a {\
                 \n  b: is-superselector(\
                 \n    \":nth-child(n+1 of c d, e f, g h)\",\
                 \n    \":nth-child(n+1 of c d.i, e j f)\");\
                 \n}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: true;\
             \n}\
             \n"
        );
    }
}

// From "sass-spec/spec/core_functions/selector/is_superselector/simple/pseudo/selector_arg/nth_last_child.hrx"
mod nth_last_child {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn bare_sub() {
        assert_eq!(
        rsass(
            "a {b: is-superselector(\":nth-last-child(n+1 of c d, e f, g h)\", \"c d, e f, g h\")}\
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
    #[ignore] // wrong result
    fn bare_super() {
        assert_eq!(
        rsass(
            "a {b: is-superselector(\"c\", \":nth-last-child(n+1 of c)\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
    }
    #[test]
    #[ignore] // wrong result
    fn different_arg() {
        assert_eq!(
            rsass(
                "a {\
                 \n  b: is-superselector(\
                 \n      \":nth-last-child(n+1 of c)\",\
                 \n      \":nth-last-child(n+2 of c)\");\
                 \n}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: false;\
             \n}\
             \n"
        );
    }
    mod prefix {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn subset() {
            assert_eq!(
                rsass(
                    "a {\
                     \n  b: is-superselector(\
                     \n      \":-pfx-nth-last-child(n+1 of c d.i, e j f)\",\
                     \n      \":-pfx-nth-last-child(n+1 of c d, e f, g h)\");\
                     \n}\
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
        #[ignore] // wrong result
        fn superset() {
            assert_eq!(
                rsass(
                    "a {\
                     \n  b: is-superselector(\
                     \n      \":-pfx-nth-last-child(n+1 of c d, e f, g h)\",\
                     \n      \":-pfx-nth-last-child(n+1 of c d.i, e j f)\");\
                     \n}\
                     \n"
                )
                .unwrap(),
                "a {\
                 \n  b: true;\
                 \n}\
                 \n"
            );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn subset() {
        assert_eq!(
            rsass(
                "a {\
                 \n  b: is-superselector(\
                 \n      \":nth-last-child(n+1 of c d.i, e j f)\",\
                 \n      \":nth-last-child(n+1 of c d, e f, g h)\");\
                 \n}\
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
    #[ignore] // wrong result
    fn superset() {
        assert_eq!(
            rsass(
                "a {\
                 \n  b: is-superselector(\
                 \n      \":nth-last-child(n+1 of c d, e f, g h)\",\
                 \n      \":nth-last-child(n+1 of c d.i, e j f)\");\
                 \n}\
                 \n"
            )
            .unwrap(),
            "a {\
             \n  b: true;\
             \n}\
             \n"
        );
    }
}

// From "sass-spec/spec/core_functions/selector/is_superselector/simple/pseudo/selector_arg/slotted.hrx"
mod slotted {
    #[allow(unused)]
    use super::rsass;
    #[test]
    #[ignore] // wrong result
    fn bare_sub() {
        assert_eq!(
        rsass(
            "a {b: is-superselector(\"::slotted(c d, e f, g h)\", \"c d, e f, g h\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: false;\
        \n}\
        \n"
    );
    }
    mod prefix {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // wrong result
        fn subset() {
            assert_eq!(
        rsass(
            "a {b: is-superselector(\"::-pfx-slotted(c d.i, e j f)\", \"::-pfx-slotted(c d, e f, g h)\")}\
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
        #[ignore] // wrong result
        fn superset() {
            assert_eq!(
        rsass(
            "a {b: is-superselector(\"::-pfx-slotted(c d, e f, g h)\", \"::-pfx-slotted(c d.i, e j f)\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
        }
    }
    #[test]
    #[ignore] // wrong result
    fn subset() {
        assert_eq!(
        rsass(
            "a {b: is-superselector(\"::slotted(c d.i, e j f)\", \"::slotted(c d, e f, g h)\")}\
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
    #[ignore] // wrong result
    fn superset() {
        assert_eq!(
        rsass(
            "a {b: is-superselector(\"::slotted(c d, e f, g h)\", \"::slotted(c d.i, e j f)\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: true;\
        \n}\
        \n"
    );
    }
}
