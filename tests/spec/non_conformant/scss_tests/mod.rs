//! Tests auto-converted from "sass-spec/spec/non_conformant/scss-tests"

mod t001_test_one_line_comments;

mod t002_test_one_line_comments;

mod t003_test_variables;

mod t004_test_variables;

// From "sass-spec/spec/non_conformant/scss-tests/005_test_unicode_variables"
#[test]
fn t005_test_unicode_variables() {
    assert_eq!(
        crate::rsass(
            "$vär: foo;\
            \n\
            \nblat {a: $vär}\
            \n"
        )
        .unwrap(),
        "blat {\
        \n  a: foo;\
        \n}\
        \n"
    );
}

mod t006_test_guard_assign;

mod t007_test_guard_assign;

mod t008_test_sass_script;

mod t011_test_if_directive;

mod t012_test_if_directive;

mod t013_test_if_directive;

mod t014_test_comment_after_if_directive;

mod t015_test_comment_after_if_directive;

mod t017_test_each_directive;

mod t019_test_css_import_directive;

mod t020_test_css_import_directive;

mod t021_test_css_import_directive;

mod t022_test_css_import_directive;

mod t023_test_css_import_directive;

mod t024_test_media_import;

mod t025_test_dynamic_media_import;

mod t027_test_protocol_relative_import;

mod t028_test_import_with_interpolation;

mod t029_test_url_import;

mod t030_test_block_comment_in_script;

mod t031_test_line_comment_in_script;

mod t032_test_nested_rules;

mod t033_test_nested_rules;

mod t034_test_nested_rules;

mod t035_test_nested_rules_with_declarations;

mod t036_test_nested_rules_with_declarations;

mod t037_test_nested_rules_with_declarations;

mod t038_test_nested_rules_with_fancy_selectors;

mod t039_test_almost_ambiguous_nested_rules_and_declarations;

mod t040_test_newlines_in_selectors;

mod t041_test_newlines_in_selectors;

mod t042_test_newlines_in_selectors;

mod t043_test_newlines_in_selectors;

mod t044_test_trailing_comma_in_selector;

mod t045_test_parent_selectors;

mod t046_test_parent_selector_with_subject;

mod t047_test_unknown_directive_bubbling;

mod t048_test_namespace_properties;

mod t049_test_several_namespace_properties;

mod t050_test_nested_namespace_properties;

mod t051_test_namespace_properties_with_value;

mod t052_test_namespace_properties_with_script_value;

mod t053_test_no_namespace_properties_without_space;

mod t054_test_basic_mixins;

mod t055_test_basic_mixins;

mod t056_test_basic_mixins;

mod t057_test_mixins_with_empty_args;

mod t058_test_mixins_with_empty_args;

mod t059_test_mixins_with_empty_args;

mod t060_test_mixins_with_args;

mod t061_test_mixins_with_args;

mod t062_test_basic_function;

mod t063_test_function_args;

mod t064_test_mixin_var_args;

mod t065_test_mixin_empty_var_args;

mod t066_test_mixin_var_args_act_like_list;

mod t067_test_mixin_splat_args;

mod t068_test_mixin_splat_expression;

mod t069_test_mixin_splat_args_with_var_args;

mod t070_test_mixin_splat_args_with_var_args_and_normal_args;

mod t071_test_mixin_splat_args_with_var_args_preserves_separator;

mod t072_test_mixin_var_and_splat_args_pass_through_keywords;

mod t078_test_mixin_list_of_pairs_splat_treated_as_list;

mod t083_test_function_var_args;

mod t084_test_function_empty_var_args;

mod t085_test_function_var_args_act_like_list;

mod t086_test_function_splat_args;

mod t087_test_function_splat_expression;

mod t088_test_function_splat_args_with_var_args;

mod t089_test_function_splat_args_with_var_args_and_normal_args;

mod t090_test_function_splat_args_with_var_args_preserves_separator;

mod t091_test_function_var_and_splat_args_pass_through_keywords;

mod t098_test_function_list_of_pairs_splat_treated_as_list;

mod t103_test_function_var_args_passed_to_native;

mod t104_test_basic_selector_interpolation;

mod t105_test_basic_selector_interpolation;

mod t106_test_basic_selector_interpolation;

mod t107_test_selector_only_interpolation;

mod t108_test_selector_interpolation_before_element_name;

mod t109_test_selector_interpolation_in_string;

mod t110_test_selector_interpolation_in_pseudoclass;

mod t111_test_selector_interpolation_at_class_begininng;

mod t112_test_selector_interpolation_at_id_begininng;

mod t113_test_selector_interpolation_at_pseudo_begininng;

mod t114_test_selector_interpolation_at_attr_beginning;

mod t115_test_selector_interpolation_at_attr_end;

mod t116_test_selector_interpolation_at_dashes;

mod t118_test_parent_selector_with_parent_and_subject;

mod t119_test_basic_prop_name_interpolation;

mod t120_test_basic_prop_name_interpolation;

mod t121_test_prop_name_only_interpolation;

mod t122_test_directive_interpolation;

mod t130_test_random_directive_interpolation;

mod t131_test_nested_mixin_def;

mod t132_test_nested_mixin_shadow;

mod t133_test_nested_function_def;

mod t134_test_nested_function_shadow;

mod t171_test_loud_comment_in_compressed_mode;

mod t172_test_parsing_decimals_followed_by_comments_doesnt_take_forever;

mod t173_test_parsing_many_numbers_doesnt_take_forever;

mod t174_test_import_comments_in_imports;

mod t176_test_newline_selector_rendered_multiple_times;

mod t177_test_prop_name_interpolation_after_hyphen;

mod t178_test_star_plus_and_parent;

mod t179_test_weird_added_space;

mod t180_test_interpolation_with_bracket_on_next_line;

mod t181_test_interpolation;

mod t182_test_mixin_with_keyword_args;

mod t183_test_passing_required_args_as_a_keyword_arg;

mod t184_test_passing_all_as_keyword_args_in_opposite_order;

mod t185_test_keyword_args_in_functions;

mod t186_test_newlines_removed_from_selectors_when_compressed;

mod t187_test_multiline_var;

mod t188_test_mixin_content;

mod t189_test_empty_content;

mod t190_test_options_passed_to_script;

mod t191_test_extend_in_media_in_rule;
