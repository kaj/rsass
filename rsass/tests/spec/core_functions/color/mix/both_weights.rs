//! Tests auto-converted from "sass-spec/spec/core_functions/color/mix/both_weights.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("both_weights")
}

#[test]
fn contradiction() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \n// When we weight entirely towards a transparent color, the formula for\
             \n// computing the combined alpha would divide by zero, so we just return\
             \n// transparent as a special case.\
             \na {b: color.mix(transparent, #0144bf, 100%)}\n"
        ),
        "a {\
         \n  b: rgba(0, 0, 0, 0);\
         \n}\n"
    );
}
mod mixed {
    use super::runner;

    #[test]
    fn firstwards() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(rgba(#91e16f, 0.8), rgba(#0144bf, 0.3), 63%)}\n"
        ),
        "a {\
         \n  b: rgba(121.4247787611, 199.296460177, 124.0973451327, 0.615);\
         \n}\n"
    );
    }
    #[test]
    fn lastwards() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(rgba(#91e16f, 0.2), rgba(#0144bf, 0.7), 42%)}\n"
        ),
        "a {\
         \n  b: rgba(29, 98.5277777778, 175.4444444444, 0.49);\
         \n}\n"
    );
    }
}
mod transparent {
    use super::runner;

    #[test]
    fn first() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.mix(transparent, #0144bf, 70%)}\n"),
            "a {\
         \n  b: rgba(1, 68, 191, 0.3);\
         \n}\n"
        );
    }
    #[test]
    fn last() {
        assert_eq!(
            runner().ok("@use \"sass:color\";\
             \na {b: color.mix(#91e16f, transparent, 70%)}\n"),
            "a {\
         \n  b: rgba(145, 225, 111, 0.7);\
         \n}\n"
        );
    }
}
mod weighted {
    use super::runner;

    #[test]
    fn first() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(rgba(#91e16f, 0.2), rgba(#0144bf, 0.7), 100%)}\n"
        ),
        "a {\
         \n  b: rgba(145, 225, 111, 0.2);\
         \n}\n"
    );
    }
    #[test]
    fn last() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \na {b: color.mix(rgba(#91e16f, 0.2), rgba(#0144bf, 0.7), 0%)}\n"
        ),
        "a {\
         \n  b: rgba(1, 68, 191, 0.7);\
         \n}\n"
    );
    }
}
