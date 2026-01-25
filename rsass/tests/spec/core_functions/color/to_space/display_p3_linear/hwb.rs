//! Tests auto-converted from "sass-spec/spec/core_functions/color/to_space/display_p3_linear/hwb.hrx"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("hwb")
}

mod alpha {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn partial() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.to-space(color(display-p3-linear 0.1 0.2 0.3 / 0.4), hwb));\n"
        ),
        "a {\
         \n  value: hsla(202.040183322, 31.6511999115%, 45.1325807768%, 0.4);\
         \n  space: hwb;\
         \n  channels: 202.040183322deg 30.8475774099% 40.5824158563% / 0.4;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn transparent() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.to-space(color(display-p3-linear 0.1 0.2 0.3 / 0.0), hwb));\n"
        ),
        "a {\
         \n  value: hsla(202.040183322, 31.6511999115%, 45.1325807768%, 0);\
         \n  space: hwb;\
         \n  channels: 202.040183322deg 30.8475774099% 40.5824158563% / 0;\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn black() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.to-space(color(display-p3-linear 0 0 0), hwb));\n"
        ),
        "a {\
         \n  value: black;\
         \n  space: hwb;\
         \n  channels: 0deg 0% 100% / 1;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn gray() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.to-space(color(display-p3-linear 0.5 0.5 0.5), hwb));\n"
        ),
        "a {\
         \n  value: hsl(0, 0%, 73.5356983052%);\
         \n  space: hwb;\
         \n  channels: 0deg 73.5356983052% 26.4643016948% / 1;\
         \n}\n"
    );
}
#[test]
#[ignore] // unexepected error
fn middle() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.to-space(color(display-p3-linear 0.2 0.4 0.8), hwb));\n"
        ),
        "a {\
         \n  value: hsl(210.8995407892, 77.4665072553%, 67.8915160094%);\
         \n  space: hwb;\
         \n  channels: 210.8995407892deg 43.0181949293% 7.2351629105% / 1;\
         \n}\n"
    );
}
mod missing {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn blue() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \na {b: color.to-space(color(display-p3-linear 0.1 0.2 none), hwb)}\n"
        ),
        "a {\
         \n  b: hsl(77.1972503288, 181.3073182011%, 17.3912222966%);\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn green() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.to-space(color(display-p3-linear 0.1 none 0.3), hwb));\n"
        ),
        "a {\
         \n  value: hsl(279.7647236052, 119.0885815094%, 27.7341794019%);\
         \n  space: hwb;\
         \n  channels: 279.7647236052deg -5.2940614411% 39.2375797552% / 1;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn red() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.to-space(color(display-p3-linear none 0.2 0.3), hwb));\n"
        ),
        "a {\
         \n  value: hsl(187.3682558048, 230.0217528421%, 18.0556689104%);\
         \n  space: hwb;\
         \n  channels: 187.3682558048deg -23.4762972047% 40.4123649744% / 1;\
         \n}\n"
    );
    }
}
mod out_of_range {
    use super::runner;

    #[test]
    #[ignore] // unexepected error
    fn far() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.to-space(color(display-p3-linear -999999 0 0), hwb));\n"
        ),
        "a {\
         \n  value: hsl(356.7844906486, 165.0029138842%, -13697.7855994258%);\
         \n  space: hwb;\
         \n  channels: 176.7844906486deg -36299.5309760879% -8803.9597772363% / 1;\
         \n}\n"
    );
    }
    #[test]
    #[ignore] // unexepected error
    fn near() {
        assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.to-space(color(display-p3-linear -1 0.4 2), hwb));\n"
        ),
        "a {\
         \n  value: hsl(196.5412734015, 909.4415388743%, 13.9290938037%);\
         \n  space: hwb;\
         \n  channels: 196.5412734015deg -112.7478712361% -40.6060588436% / 1;\
         \n}\n"
    );
    }
}
#[test]
#[ignore] // unexepected error
fn white() {
    assert_eq!(
        runner().ok(
            "@use \"sass:color\";\
             \n@use \'core_functions/color/utils\';\
             \n@include utils.inspect(color.to-space(color(display-p3-linear 1 1 1), hwb));\n"
        ),
        "a {\
         \n  value: white;\
         \n  space: hwb;\
         \n  channels: 0deg 100% 0% / 1;\
         \n}\n"
    );
}
