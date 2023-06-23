//! Tests auto-converted from "sass-spec/spec/directives/use/extend/extended.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("extended")
        .mock_file("extended/from_other_file/_midstream.scss", "@use \"upstream\";\n\nin-midstream {@extend in-upstream}\n")
        .mock_file("extended/from_other_file/_upstream.scss", "in-upstream {x: y}\n")
        .mock_file("extended/from_same_file/_other.scss", "in-other-extender {@extend in-other-extendee}\n\nin-other-extendee {x: y}\n")
}

mod extended {
    #[allow(unused)]
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("extended")
    }

    #[test]
    #[ignore] // unexepected error
    fn from_other_file() {
        let runner = runner().with_cwd("from_other_file");
        assert_eq!(
            runner.ok("@use \"midstream\";\n\
             \nin-input {@extend in-midstream}\n"),
            "in-upstream, in-midstream, in-input {\
         \n  x: y;\
         \n}\n"
        );
    }
    #[test]
    #[ignore] // unexepected error
    fn from_same_file() {
        let runner = runner().with_cwd("from_same_file");
        assert_eq!(
            runner.ok("@use \"other\";\n\
             \nin-input {@extend in-other-extender}\n"),
            "in-other-extendee, in-other-extender, in-input {\
         \n  x: y;\
         \n}\n"
        );
    }
}
