//! Tests auto-converted from "sass-spec/spec/css/custom_properties/value_interpolation.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("value_interpolation")
}

mod error {
    #[allow(unused)]
    use super::runner;

    mod sass {
        #[allow(unused)]
        use super::runner;
    }
}
mod sass {
    #[allow(unused)]
    use super::runner;
}
mod scss {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn alone() {
        assert_eq!(
            runner().ok("a{\
             \n  --b: #{1 + 2};\
             \n}\n"),
            "a {\
         \n  --b: 3;\
         \n}\n"
        );
    }
    #[test]
    fn in_ident() {
        assert_eq!(
            runner().ok("a{\
             \n  --b: c#{1 + 2}d;\
             \n}\n"),
            "a {\
         \n  --b: c3d;\
         \n}\n"
        );
    }
    #[test]
    fn in_list() {
        assert_eq!(
            runner().ok("a{\
             \n  --b: c #{1 + 2} d;\
             \n}\n"),
            "a {\
         \n  --b: c 3 d;\
         \n}\n"
        );
    }
    #[test]
    fn in_string() {
        assert_eq!(
            runner().ok("a{\
             \n  --b: \"c#{1 + 2}d\";\
             \n}\n"),
            "a {\
         \n  --b: \"c3d\";\
         \n}\n"
        );
    }
    #[test]
    fn in_uri() {
        assert_eq!(
            runner().ok("a{\
             \n  --b: uri(c#{1 + 2}d);\
             \n}\n"),
            "a {\
         \n  --b: uri(c3d);\
         \n}\n"
        );
    }
    #[test]
    fn linebreak_interpolation() {
        assert_eq!(
            runner().ok("a{\
             \n  --b: #{1 \
             \n    + \
             \n    2};\
             \n}\n"),
            "a {\
         \n  --b: 3;\
         \n}\n"
        );
    }
    #[test]
    fn value_interpolation() {
        assert_eq!(
        runner().ok(
            ".value-interpolation {\
             \n  // Interpolation is the only Sass construct that\'s supported in custom\
             \n  // variables.\
             \n  --alone: #{1 + 2};\
             \n  --in-list: a #{1 + 2} c;\
             \n  --in-ident: foo#{1 + 2}bar;\
             \n  --in-string: \"foo#{1 + 2}bar\";\
             \n  --in-uri: uri(foo#{1 + 2}bar);\
             \n}\n"
        ),
        ".value-interpolation {\
         \n  --alone: 3;\
         \n  --in-list: a 3 c;\
         \n  --in-ident: foo3bar;\
         \n  --in-string: \"foo3bar\";\
         \n  --in-uri: uri(foo3bar);\
         \n}\n"
    );
    }
}
