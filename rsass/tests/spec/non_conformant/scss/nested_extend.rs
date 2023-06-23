//! Tests auto-converted from "sass-spec/spec/non_conformant/scss/nested-extend.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("nested-extend")
}

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        runner().ok(".sprites-nav {\
             \n  color: red;\
             \n}\n\
             \n.sprites-nav_up {\
             \n  color: green;\
             \n}\n\
             \n.mw_nav_button {\
             \n  float: right;\
             \n  width: 30px;\
             \n  height: 30px;\
             \n  margin: 10px 10px 10px 0;\
             \n  overflow: hidden;\
             \n  &[data-ur-state=\"disabled\"] {\
             \n    @extend .sprites-nav;\
             \n  }\
             \n  &[data-ur-state=\"enabled\"] {\
             \n    @extend .sprites-nav_up;\
             \n  }\
             \n}\n"),
        ".sprites-nav, .mw_nav_button[data-ur-state=disabled] {\
         \n  color: red;\
         \n}\
         \n.sprites-nav_up, .mw_nav_button[data-ur-state=enabled] {\
         \n  color: green;\
         \n}\
         \n.mw_nav_button {\
         \n  float: right;\
         \n  width: 30px;\
         \n  height: 30px;\
         \n  margin: 10px 10px 10px 0;\
         \n  overflow: hidden;\
         \n}\n"
    );
}
