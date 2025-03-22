//! Tests auto-converted from "sass-spec/spec/libsass"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("libsass")
}

mod sass_utf8;

mod arg_eval;

mod at_error;

mod at_root;

mod at_stuff;

mod base_level_parent;

mod basic;

mod bool;

mod bourbon;

mod charset;

mod color_functions;

mod conversions;

mod css_import;

mod css_nth_selectors;

mod css_unicode;

mod debug_directive_nested;

mod delayed;

mod div;

mod env;

mod eq;

mod error_directive_nested;

mod http_import;

mod image_url;

mod import;

mod inh;

mod interpolated_function_call;

mod interpolated_urls;

mod keyframes;

mod list_evaluation;

mod lists;

mod media_hoisting;

mod media;

mod mixin;

mod mixins_and_media_queries;

mod multi_blocks;

mod parent_selector;

mod placeholder_mediaquery;

mod placeholder_nested;

mod precision;

mod properties_in_media;

mod propsets;

mod rel;

mod selector_functions;

mod selector_interpolation_in_string;

mod selectors;

mod test;

mod unary_ops;

mod unicode_bom;

mod units;

mod url;

mod variable_scoping;

mod variables_in_media;

mod wrapped_selector_whitespace;
