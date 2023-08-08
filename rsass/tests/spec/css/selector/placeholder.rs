//! Tests auto-converted from "sass-spec/spec/css/selector/placeholder.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("placeholder")
}

mod pseudoselectors {
    #[allow(unused)]
    use super::runner;

    mod is {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn solo() {
            assert_eq!(
        runner().ok(
            "// Since `%b` doesn\'t exist, no selectors can match it, so this rule should be\
             \n// removed.\
             \na:is(%b) {x: y}\n"
        ),
        ""
    );
        }
        #[test]
        fn with_real() {
            assert_eq!(
        runner().ok(
            "// Since `%b` doesn\'t exist, an element matches `%b` or `c` iff it matches `c`.\
             \na:is(%b, c) {x: y}\n"
        ),
        "a:is(c) {\
         \n  x: y;\
         \n}\n"
    );
        }
    }
    mod matches {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn solo() {
            assert_eq!(
        runner().ok(
            "// Since `%b` doesn\'t exist, no selectors can match it, so this rule should be\
             \n// removed.\
             \na:matches(%b) {x: y}\n"
        ),
        ""
    );
        }
        #[test]
        fn with_real() {
            assert_eq!(
        runner().ok(
            "// Since `%b` doesn\'t exist, an element matches `%b` or `c` iff it matches `c`.\
             \na:matches(%b, c) {x: y}\n"
        ),
        "a:matches(c) {\
         \n  x: y;\
         \n}\n"
    );
        }
    }
    mod not {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn solo() {
            assert_eq!(
        runner().ok(
            "// Since `%b` doesn\'t exist, all `a` elements match `a:not(%b)`.\
             \na:not(%b) {x: y}\n"
        ),
        "a {\
         \n  x: y;\
         \n}\n"
    );
        }
        #[test]
        fn universal() {
            assert_eq!(
        runner().ok(
            "// Since `%b` doesn\'t exist, all elements match `:not(%b)`.\
             \n:not(%b) {x: y}\n"
        ),
        "* {\
         \n  x: y;\
         \n}\n"
    );
        }
        #[test]
        fn with_real() {
            assert_eq!(
        runner().ok(
            "// Since `%b` doesn\'t exist, it can be removed from the `:not` pseudoselector.\
             \na:not(%b, c) {x: y}\n"
        ),
        "a:not(c) {\
         \n  x: y;\
         \n}\n"
    );
        }
    }
    mod test_where {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn nesting() {
            assert_eq!(
                runner().ok("a {\
             \n  :where(&) {\
             \n    b: c;\
             \n  }\
             \n}\n"),
                ":where(a) {\
         \n  b: c;\
         \n}\n"
            );
        }
        #[test]
        fn solo() {
            assert_eq!(
        runner().ok(
            "// Since `%b` doesn\'t exist, no selectors can match it, so this rule should be\
             \n// removed.\
             \na:where(%b) {x: y}\n"
        ),
        ""
    );
        }
        #[test]
        fn with_real() {
            assert_eq!(
        runner().ok(
            "// Since `%b` doesn\'t exist, an element matches `%b` or `c` iff it matches `c`.\
             \na:where(%b, c) {x: y}\n"
        ),
        "a:where(c) {\
         \n  x: y;\
         \n}\n"
    );
        }
    }
}
