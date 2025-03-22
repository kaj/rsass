//! Tests auto-converted from "sass-spec/spec/non_conformant/misc"

fn runner() -> crate::TestRunner {
    super::runner().with_cwd("misc")
}

mod jma_pseudo_test;

mod directive_interpolation;

mod empty_content;

mod error_directive;

mod import_in_mixin;

mod import_with_interpolation;

mod lang_bug;

mod media_interpolation;

mod mixin_content;

mod namespace_properties_with_script_value;

mod negative_numbers;

mod selector_interpolation_before_element_name;

mod selector_only_interpolation;

mod trailing_comma_in_selector;

mod unicode_variables;
