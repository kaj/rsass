//! Tests auto-converted from "sass-spec/spec/directives"
#[allow(unused)]
use super::rsass;

// From "sass-spec/spec/directives/at_root.hrx"
mod at_root {
    #[allow(unused)]
    use super::rsass;
    mod keyframes {
        #[allow(unused)]
        use super::rsass;
        #[test]
        #[ignore] // unexepected error
        fn all() {
            assert_eq!(
                rsass(
                    "@keyframes a {\
            \n  @at-root (without: all) {\
            \n    b {c: d}\
            \n  }\
            \n}\
            \n"
                )
                .unwrap(),
                "@keyframes a {}\
        \nb {\
        \n  c: d;\
        \n}\
        \n"
            );
        }
    }
}

// Ignoring "extend", not expected to work yet.

// Ignoring "forward", not expected to work yet.

mod function;

mod test_if;

mod import;

// Ignoring "use", not expected to work yet.

mod warn;
