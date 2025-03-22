//! Tests auto-converted from "sass-spec/spec/core_functions/meta/inspect/color.hrx"

fn runner() -> crate::TestRunner {
    super::runner()
        .with_cwd("color")
        .mock_file("generated/_utils.scss", "@use \"sass:color\";\n/// Returns a copy of `$color` that doesn't have color-literal metadata\n/// associated with it.\n@function generated-color($color) {\n  // This doesn't change the value of `$color` at all, but it does construct a\n  // new object.\n  @return color.scale($color, $blue: 0%);\n}\n")
}

mod generated {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("generated")
    }

    #[test]
    fn alpha() {
        let runner = runner().with_cwd("alpha");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n$result: meta.inspect(rgba(1, 2, 3, 0.4));\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n}\n"),
            "a {\
         \n  value: rgba(1, 2, 3, 0.4);\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    fn long_hex() {
        let runner = runner().with_cwd("long_hex");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@use \"../utils\";\
             \n$result: meta.inspect(utils.generated-color(#abcdef));\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n}\n"),
            "a {\
         \n  value: #abcdef;\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    fn named() {
        let runner = runner().with_cwd("named");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@use \"../utils\";\
             \n$result: meta.inspect(utils.generated-color(#00f));\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n}\n"),
            "a {\
         \n  value: blue;\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    fn short_hex() {
        let runner = runner().with_cwd("short_hex");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@use \"../utils\";\
             \n$result: meta.inspect(utils.generated-color(#abc));\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n}\n"),
            "a {\
         \n  value: #aabbcc;\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    fn transparent() {
        let runner = runner().with_cwd("transparent");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n@use \"../utils\";\
             \n$result: meta.inspect(utils.generated-color(transparent));\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n}\n"),
            "a {\
         \n  value: rgba(0, 0, 0, 0);\
         \n  type: string;\
         \n}\n"
        );
    }
}
mod literal {
    fn runner() -> crate::TestRunner {
        super::runner().with_cwd("literal")
    }

    #[test]
    fn long_hex() {
        let runner = runner().with_cwd("long_hex");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n$result: meta.inspect(#0000ff);\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n}\n"),
            "a {\
         \n  value: #0000ff;\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    fn named() {
        let runner = runner().with_cwd("named");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n$result: meta.inspect(blue);\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n}\n"),
            "a {\
         \n  value: blue;\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    fn short_hex() {
        let runner = runner().with_cwd("short_hex");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n$result: meta.inspect(#00f);\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n}\n"),
            "a {\
         \n  value: #00f;\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    fn transparent() {
        let runner = runner().with_cwd("transparent");
        assert_eq!(
            runner.ok("@use \"sass:meta\";\
             \n$result: meta.inspect(transparent);\
             \na {\
             \n  value: $result;\
             \n  type: meta.type-of($result);\
             \n}\n"),
            "a {\
         \n  value: transparent;\
         \n  type: string;\
         \n}\n"
        );
    }
}
