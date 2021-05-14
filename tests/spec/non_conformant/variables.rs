//! Tests auto-converted from "sass-spec/spec/non_conformant/variables.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

mod global {
    #[allow(unused)]
    use super::runner;

    mod first_declaration {
        #[allow(unused)]
        use super::runner;

        #[test]
        fn nested() {
            assert_eq!(
                runner().ok("x {$var: value !global}\
             \na {b: $var}\n"),
                "a {\
         \n  b: value;\
         \n}\n"
            );
        }
        #[test]
        fn top_level() {
            assert_eq!(
                runner().ok("$var: value !global;\
             \na {b: $var}\n"),
                "a {\
         \n  b: value;\
         \n}\n"
            );
        }
    }
}
