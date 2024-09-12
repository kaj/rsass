//! Tests auto-converted from "sass-spec/spec/core_functions/color/scale/error/incompatible_channel.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("incompatible_channel")
}

#[test]
#[ignore] // wrong error
fn legacy_channel() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale(color(srgb 0.2 0.5 0.7), $whiteness: 50%)}\n"
        ),
        "Error: $whiteness: Color space srgb doesn\'t have a channel with this name.\
         \n  ,\
         \n2 | a {b: color.scale(color(srgb 0.2 0.5 0.7), $whiteness: 50%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn legacy_space() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale(red, $chroma: 50%)}\n"
        ),
        "Error: $chroma: Color space rgb doesn\'t have a channel with this name.\
         \n  ,\
         \n2 | a {b: color.scale(red, $chroma: 50%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn modern_both() {
    assert_eq!(
        runner().err(
            "@use \"sass:color\";\
             \na {b: color.scale(color(srgb 0.2 0.5 0.7), $chroma: 50%)}\n"
        ),
        "Error: $chroma: Color space srgb doesn\'t have a channel with this name.\
         \n  ,\
         \n2 | a {b: color.scale(color(srgb 0.2 0.5 0.7), $chroma: 50%)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:7  root stylesheet",
    );
}
