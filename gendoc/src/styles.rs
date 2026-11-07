use crate::defs::*;
use domrs::*;
use std::sync::LazyLock;

static FONT_NORMAL: LazyLock<CssFontFamily> =
    LazyLock::new(|| CssFontFamily::new(&["Barlow".to_string()], CssFontGenericFamily::SansSerif));
static FONT_CONDENSED: LazyLock<CssFontFamily> = LazyLock::new(|| {
    CssFontFamily::new(
        &["Barlow Condensed".to_string()],
        CssFontGenericFamily::SansSerif,
    )
});
static FONT_MONO: LazyLock<CssFontFamily> = LazyLock::new(|| {
    CssFontFamily::new(
        &["JetBrains Mono".to_string()],
        CssFontGenericFamily::Monospace,
    )
});

/// Creates styles common for the whole document.
pub fn create_document_style() -> CssDocument {
    CssDocument::new()
        .group(
            CssGroup::media_print()
                .ruleset(
                    CssRuleset::new(CssSelector::new().element("body"))
                        .declaration(CssProperty::FontSize, pt!(10.0))
                        .declaration(CssProperty::Margin, CssValue::Zero),
                )
                .ruleset(
                    CssRuleset::new(CssSelector::new().element("svg"))
                        .declaration(CssProperty::Border, CssValue::Unset)
                        .declaration(CssProperty::Width, perc!(100)),
                )
                .ruleset(
                    CssRuleset::new(CssSelector::new().class(CLASS_DIAGRAM_SECTION)).declaration(
                        CssProperty::GridTemplateColumns,
                        (zero!(), auto!(), zero!()),
                    ),
                ),
        )
        .ruleset(
            CssRuleset::new(CssSelector::new().element("body"))
                .declaration(CssProperty::Color, CssColor::Black)
                .declaration(CssProperty::FontFamily, FONT_NORMAL.clone())
                .declaration(CssProperty::FontSize, pt!(16))
                .declaration(CssProperty::FontWeight, 300)
                .declaration(CssProperty::BackgroundColor, CssColor::White),
        )
        .ruleset(
            CssRuleset::new(CssSelector::new().element("section"))
                .declaration(CssProperty::Display, CssValue::Block),
        )
        .ruleset(
            CssRuleset::new(CssSelector::new().element("svg"))
                .declaration(CssProperty::Color, CssColor::Black)
                .declaration(
                    CssProperty::Border,
                    border!(px!(1), CssBorderStyle::Solid, CssColor::Hex(0xdbdbdb)),
                )
                .declaration(SvgAttribute::Fill, CssColor::White)
                .declaration(CssProperty::FontFamily, FONT_CONDENSED.clone())
                .declaration(CssProperty::FontWeight, 400)
                .declaration(CssProperty::FontSize, px!(20))
                .declaration(CssProperty::Overflow, CssValue::Hidden)
                .declaration(SvgAttribute::Stroke, CssColor::Black)
                .declaration(SvgAttribute::StrokeWidth, 1)
                .declaration(CssProperty::UserSelect, CssValue::None),
        )
        .ruleset(
            CssRuleset::new(CssSelector::new().element("h1"))
                .declaration(CssProperty::BackgroundColor, CssColor::Hex(0x002f56))
                .declaration(CssProperty::Color, CssColor::White)
                .declaration(CssProperty::FontSize, em!(1.4, 1))
                .declaration(CssProperty::FontWeight, 600)
                .declaration(CssProperty::Margin, (zero!(), zero!(), px!(8), zero!()))
                .declaration(CssProperty::TextAlign, CssValue::Left)
                .declaration(CssProperty::Padding, (px!(8), px!(20))),
        )
        .ruleset(
            CssRuleset::new(CssSelector::new().element("h2"))
                .declaration(CssProperty::BackgroundColor, CssColor::Hex(0xd8ecff))
                .declaration(CssProperty::FontSize, em!(1.2, 1))
                .declaration(CssProperty::FontWeight, 600)
                .declaration(CssProperty::Margin, CssValue::Zero)
                .declaration(CssProperty::TextAlign, CssValue::Left)
                .declaration(CssProperty::Padding, (px!(8), px!(20))),
        )
        .ruleset(
            CssRuleset::new(CssSelector::new().element("h3"))
                .declaration(CssProperty::BackgroundColor, CssColor::White)
                .declaration(CssProperty::FontSize, em!(1.2, 1))
                .declaration(CssProperty::FontWeight, 400)
                .declaration(CssProperty::Margin, CssValue::Zero)
                .declaration(CssProperty::TextAlign, CssValue::Center)
                .declaration(
                    CssProperty::Padding,
                    (px!(12), px!(20.0), px!(4.0), px!(20)),
                ),
        )
        .ruleset(
            CssRuleset::new(CssSelector::new().element("strong"))
                .declaration(CssProperty::FontWeight, 600),
        )
}

/// Creates styles needed to properly visualize the model.
pub fn create_model_style() -> CssDocument {
    let description_sub_element = |sub_element: &str| {
        CssSelector::new().part(
            CssSelectorPart::new()
                .class(CLASS_DESCRIPTION)
                .element(sub_element),
        )
    };
    CssDocument::new()
        .ruleset(
            CssRuleset::new(CssSelector::new().class(CLASS_DIAGRAM_SECTION))
                .declaration(CssProperty::Display, CssValue::Grid)
                .declaration(
                    CssProperty::GridTemplateColumns,
                    (auto!(), perc!(80), auto!()),
                ),
        )
        .ruleset(
            CssRuleset::new(CssSelector::new().class(CLASS_DIAGRAM_CONTAINER))
                .declaration(CssProperty::AlignItems, CssValue::Center)
                .declaration(CssProperty::BreakInside, CssValue::Avoid)
                .declaration(CssProperty::Display, CssValue::Flex)
                .declaration(CssProperty::FlexDirection, CssValue::Column)
                .declaration(CssProperty::JustifyContent, CssValue::Center)
                .declaration(CssProperty::Padding, px!(5))
                .declaration(CssProperty::Width, perc!(100)),
        )
        .ruleset(
            CssRuleset::new(CssSelector::new().class(CLASS_DIAGRAM_TITLE))
                .declaration(CssProperty::FontSize, em!(1.2, 1))
                .declaration(CssProperty::FontWeight, 400)
                .declaration(CssProperty::Margin, zero!())
                .declaration(CssProperty::Padding, (px!(12), px!(12), px!(4), px!(12)))
                .declaration(CssProperty::TextAlign, CssValue::Center),
        )
        .ruleset(
            CssRuleset::new(CssSelector::new().class(CLASS_MODEL_ELEMENT_CONTAINER))
                .declaration(CssProperty::Margin, zero!())
                .declaration(CssProperty::Padding, (px!(20), zero!(), zero!(), px!(20))),
        )
        .ruleset(
            CssRuleset::new(CssSelector::new().class(CLASS_MODEL_ELEMENT_NAME))
                .declaration(CssProperty::Display, CssValue::InlineBlock)
                .declaration(CssProperty::FontSize, em!(1.2, 1))
                .declaration(CssProperty::FontWeight, 600),
        )
        .ruleset(
            CssRuleset::new(CssSelector::new().class(CLASS_MODEL_ELEMENT_TYPE))
                .declaration(CssProperty::Display, CssValue::InlineBlock)
                .declaration(CssProperty::FontSize, em!(1))
                .declaration(CssProperty::FontWeight, 300),
        )
        .ruleset(
            CssRuleset::new(CssSelector::new().class(CLASS_VARIABLE_DETAILS_CONTAINER))
                .declaration(CssProperty::AlignContent, CssValue::FlexStart)
                .declaration(CssProperty::BreakInside, CssValue::Avoid)
                .declaration(CssProperty::Display, CssValue::Flex)
                .declaration(CssProperty::FlexDirection, CssValue::Column)
                .declaration(CssProperty::Margin, zero!())
                .declaration(CssProperty::Padding, (px!(12), zero!(), px!(12), px!(30))),
        )
        .ruleset(
            CssRuleset::new(CssSelector::new().class(CLASS_VARIABLE_DETAILS_HEADING))
                .declaration(CssProperty::FontSize, em!(1))
                .declaration(CssProperty::FontWeight, 600)
                .declaration(CssProperty::TextAlign, CssValue::Left)
                .declaration(CssProperty::Margin, zero!())
                .declaration(CssProperty::Padding, (zero!(), zero!(), px!(4), zero!())),
        )
        .ruleset(
            CssRuleset::new(CssSelector::new().class(CLASS_VARIABLE_DETAILS_PROPERTIES))
                .declaration(CssProperty::AlignSelf, CssValue::Start)
                .declaration(CssProperty::BackgroundColor, CssColor::Hex(0xb6b6b6))
                .declaration(
                    CssProperty::Border,
                    border!(px!(2), CssBorderStyle::Solid, CssColor::Hex(0xb6b6b6)),
                )
                .declaration(CssProperty::Display, CssValue::Grid)
                .declaration(CssProperty::GridGap, px!(2))
                .declaration(CssProperty::GridTemplateColumns, (auto!(), auto!())),
        )
        .ruleset(
            CssRuleset::new(CssSelector::new().class(CLASS_VARIABLE_DETAILS_PROPERTY_NAME))
                .declaration(CssProperty::BackgroundColor, CssColor::Hex(0xe1e1e1))
                .declaration(CssProperty::FontSize, em!(1))
                .declaration(CssProperty::FontWeight, 300)
                .declaration(CssProperty::TextAlign, CssValue::Right)
                .declaration(CssProperty::Margin, zero!())
                .declaration(CssProperty::Padding, (px!(4), px!(10), px!(4), px!(10))),
        )
        .ruleset(
            CssRuleset::new(CssSelector::new().class(CLASS_VARIABLE_DETAILS_PROPERTY_VALUE))
                .declaration(CssProperty::BackgroundColor, CssColor::White)
                .declaration(CssProperty::FontSize, em!(1))
                .declaration(CssProperty::FontWeight, 500)
                .declaration(CssProperty::TextAlign, CssValue::Left)
                .declaration(CssProperty::Margin, zero!())
                .declaration(CssProperty::Padding, (px!(4), px!(10), px!(4), px!(10))),
        )
        .ruleset(
            CssRuleset::new(CssSelector::new().class(CLASS_VARIABLE_DETAILS_PROPERTY_VALUE_TYPE))
                .declaration(CssProperty::BackgroundColor, CssColor::White)
                .declaration(CssProperty::FontSize, em!(1))
                .declaration(CssProperty::FontStyle, CssFontStyle::Italic)
                .declaration(CssProperty::FontWeight, 600)
                .declaration(CssProperty::TextAlign, CssValue::Left)
                .declaration(CssProperty::Margin, zero!())
                .declaration(CssProperty::Padding, (px!(4), px!(10), px!(4), px!(10))),
        )
        .ruleset(
            CssRuleset::new(CssSelector::new().class(CLASS_DESCRIPTION_CONTAINER))
                .declaration(
                    CssProperty::BorderLeft,
                    border!(px!(3), CssBorderStyle::Solid, CssColor::Hex(0xffac41)),
                )
                .declaration(CssProperty::Margin, (px!(10), px!(30), px!(10), px!(30)))
                .declaration(CssProperty::Padding, (zero!(), zero!(), zero!(), px!(8))),
        )
        .ruleset(
            CssRuleset::new(CssSelector::new().class(CLASS_DESCRIPTION))
                .declaration(CssProperty::FontSize, em!(1))
                .declaration(CssProperty::FontWeight, 300)
                .declaration(CssProperty::Margin, zero!())
                .declaration(CssProperty::Padding, zero!()),
        )
        .ruleset(
            CssRuleset::new(description_sub_element("p"))
                .declaration(CssProperty::Margin, zero!())
                .declaration(CssProperty::Padding, zero!()),
        )
        .ruleset(
            CssRuleset::new(description_sub_element("ol"))
                .declaration(CssProperty::Margin, (px!(4), zero!())),
        )
        .ruleset(
            CssRuleset::new(description_sub_element("ul"))
                .declaration(CssProperty::Margin, (px!(4), zero!())),
        )
        .ruleset(
            CssRuleset::new(CssSelector::new().class(CLASS_EXPRESSION_INSTANCE_CONTAINER))
                .declaration(CssProperty::AlignContent, CssValue::FlexStart)
                .declaration(CssProperty::BreakInside, CssValue::Avoid)
                .declaration(CssProperty::Display, CssValue::Flex)
                .declaration(CssProperty::FlexDirection, CssValue::Column)
                .declaration(CssProperty::Margin, zero!())
                .declaration(CssProperty::Padding, (px!(12), zero!(), px!(12), px!(30))),
        )
        .ruleset(
            CssRuleset::new(CssSelector::new().class(CLASS_LITERAL_EXPRESSION))
                .declaration(CssProperty::AlignSelf, CssValue::FlexStart)
                .declaration(
                    CssProperty::Border,
                    border!(px!(1), CssBorderStyle::Solid, CssColor::Hex(0x9b9b9b)),
                )
                .declaration(CssProperty::Color, CssColor::BlueViolet)
                .declaration(CssProperty::FontFamily, FONT_MONO.clone())
                .declaration(CssProperty::FontSize, em!(0.8, 1))
                .declaration(CssProperty::FontWeight, 400)
                .declaration(CssProperty::Margin, zero!())
                .declaration(CssProperty::Padding, (px!(8), px!(20)))
                .declaration(CssProperty::TextAlign, CssValue::Left),
        )
}

/// Creates styles needed to properly visualize decision tables.
pub fn create_decision_table_style() -> CssDocument {
    let decision_table_sub_class = |sub_class: &str| {
        CssSelector::new().part(
            CssSelectorPart::new()
                .class(CLASS_DECISION_TABLE)
                .class(sub_class),
        )
    };

    // common definitions
    let n_2px = CssNumber::new(2.0, 0, CssUnit::Px);
    let n_4px = CssNumber::new(4.0, 0, CssUnit::Px);

    let color_black = CssColor::Black;
    let color_border = CssColor::Hex(0x727272);
    let color_grid_body = CssColor::Hex(0x404040);

    let bg_color_information_item_name = CssColor::Hex(0xEFEFEF);
    let bg_color_hit_policy = CssColor::Hex(0xFFB25B);
    let bg_color_rule_number = CssColor::Hex(0xFF6300);
    let bg_color_input_expression = CssColor::Hex(0x96C1E3);
    let bg_color_input_allowed_values = CssColor::Hex(0x9A9A9A);
    let bg_color_input_entry = CssColor::Hex(0xFAFAFA);
    let bg_color_output_label = CssColor::Hex(0xE8B4D1);
    let bg_color_output_component = CssColor::Hex(0xFADBEB);
    let bg_color_output_allowed_values = CssColor::Hex(0x9A9A9A);
    let bg_color_output_entry = CssColor::Hex(0xFAFAFA);
    let bg_color_annotation_label = CssValue::Color(CssColor::Hex(0xC1D9C0));
    let bg_color_annotation_allowed_values = CssColor::Hex(0x9A9A9A);
    let bg_color_annotation_entry = CssColor::Hex(0xFAFAFA);

    let border_1px = border!(px!(1), CssBorderStyle::Solid, color_border);
    let border_2px = border!(px!(2), CssBorderStyle::Solid, color_border);

    let padding_information_item_name = CssValue::Num2(n_2px, n_4px);
    let padding_box = CssValue::Num2(n_4px, n_4px);

    let box_declarations = vec![
        CssDeclaration::new(CssProperty::Color, color_black),
        CssDeclaration::new(CssProperty::Display, CssValue::Flex),
        CssDeclaration::new(CssProperty::FontWeight, 500),
        CssDeclaration::new(CssProperty::JustifyContent, CssValue::Center),
        CssDeclaration::new(CssProperty::Padding, padding_box.clone()),
        CssDeclaration::new(CssProperty::TextAlign, CssValue::Center),
    ];

    let mut aligned_box_declarations = box_declarations.clone();
    aligned_box_declarations.push(CssDeclaration::new(
        CssProperty::AlignItems,
        CssValue::Center,
    ));

    CssDocument::new()
        .ruleset(
            CssRuleset::new(CssSelector::new().class(CLASS_DECISION_TABLE))
                .declaration(CssProperty::AlignContent, CssValue::FlexStart)
                .declaration(CssProperty::BreakInside, CssValue::Avoid)
                .declaration(CssProperty::Color, CssColor::Black)
                .declaration(CssProperty::Display, CssValue::Flex)
                .declaration(CssProperty::FlexDirection, CssValue::Column)
                .declaration(CssProperty::FontWeight, 300),
        )
        .ruleset(
            CssRuleset::new(decision_table_sub_class(CLASS_INFORMATION_ITEM_NAME))
                .declaration(CssProperty::AlignSelf, CssValue::FlexStart)
                .declaration(CssProperty::BackgroundColor, bg_color_information_item_name)
                .declaration(CssProperty::Border, border_2px)
                .declaration(CssProperty::FontWeight, 500)
                .declaration(CssProperty::Padding, padding_information_item_name)
                .declaration(CssProperty::Position, CssValue::Relative)
                .declaration(CssProperty::Top, px!(2)),
        )
        .ruleset(
            CssRuleset::new(decision_table_sub_class(CLASS_GRID_CONTAINER))
                .declaration(CssProperty::Display, CssValue::Flex)
                .declaration(CssProperty::FlexDirection, CssValue::Row),
        )
        .ruleset(
            CssRuleset::new(decision_table_sub_class(CLASS_GRID_BODY))
                .declaration(CssProperty::BackgroundColor, color_border)
                .declaration(CssProperty::Border, border_2px)
                .declaration(CssProperty::Color, color_grid_body)
                .declaration(CssProperty::Display, CssValue::Grid)
                .declaration(CssProperty::GridGap, px!(2)),
        )
        .ruleset(
            CssRuleset::new(decision_table_sub_class(CLASS_VERTICAL_DOUBLE_LINE))
                .declaration(CssProperty::BackgroundColor, CssColor::White)
                .declaration(CssProperty::Width, px!(2)),
        )
        .ruleset(
            CssRuleset::new(decision_table_sub_class(CLASS_HORIZONTAL_DOUBLE_LINE))
                .declaration(CssProperty::BackgroundColor, CssColor::White)
                .declaration(CssProperty::Height, px!(2)),
        )
        .ruleset(
            CssRuleset::new(decision_table_sub_class(CLASS_HIT_POLICY))
                .declarations(&box_declarations)
                .declaration(CssProperty::BackgroundColor, bg_color_hit_policy)
                .declaration(CssProperty::Color, CssColor::White),
        )
        .ruleset(
            CssRuleset::new(decision_table_sub_class(CLASS_RULE_NUMBER))
                .declarations(&aligned_box_declarations)
                .declaration(CssProperty::BackgroundColor, bg_color_rule_number)
                .declaration(CssProperty::Color, CssColor::White),
        )
        .ruleset(
            CssRuleset::new(decision_table_sub_class(CLASS_INPUT_EXPRESSION))
                .declarations(&aligned_box_declarations)
                .declaration(CssProperty::BackgroundColor, bg_color_input_expression),
        )
        .ruleset(
            CssRuleset::new(decision_table_sub_class(CLASS_INPUT_ALLOWED_VALUES))
                .declarations(&aligned_box_declarations)
                .declaration(CssProperty::BackgroundColor, bg_color_input_allowed_values)
                .declaration(CssProperty::Color, CssColor::White)
                .declaration(CssProperty::FontWeight, 400),
        )
        .ruleset(
            CssRuleset::new(decision_table_sub_class(CLASS_INPUT_ENTRY))
                .declarations(&aligned_box_declarations)
                .declaration(CssProperty::BackgroundColor, bg_color_input_entry),
        )
        .ruleset(
            CssRuleset::new(decision_table_sub_class(CLASS_OUTPUT_LABEL))
                .declarations(&aligned_box_declarations)
                .declaration(CssProperty::BackgroundColor, bg_color_output_label),
        )
        .ruleset(
            CssRuleset::new(decision_table_sub_class(CLASS_OUTPUT_COMPONENT))
                .declarations(&aligned_box_declarations)
                .declaration(CssProperty::BackgroundColor, bg_color_output_component),
        )
        .ruleset(
            CssRuleset::new(decision_table_sub_class(CLASS_OUTPUT_ALLOWED_VALUES))
                .declarations(&aligned_box_declarations)
                .declaration(CssProperty::BackgroundColor, bg_color_output_allowed_values)
                .declaration(CssProperty::Color, CssColor::White)
                .declaration(CssProperty::FontWeight, 400),
        )
        .ruleset(
            CssRuleset::new(decision_table_sub_class(CLASS_OUTPUT_ENTRY))
                .declarations(&aligned_box_declarations)
                .declaration(CssProperty::BackgroundColor, bg_color_output_entry),
        )
        .ruleset(
            CssRuleset::new(decision_table_sub_class(CLASS_ANNOTATION_LABEL))
                .declarations(&aligned_box_declarations)
                .declaration(CssProperty::BackgroundColor, bg_color_annotation_label),
        )
        .ruleset(
            CssRuleset::new(decision_table_sub_class(CLASS_ANNOTATION_ALLOWED_VALUES))
                .declarations(&aligned_box_declarations)
                .declaration(
                    CssProperty::BackgroundColor,
                    bg_color_annotation_allowed_values,
                )
                .declaration(CssProperty::Color, CssColor::White)
                .declaration(CssProperty::FontWeight, 400),
        )
        .ruleset(
            CssRuleset::new(decision_table_sub_class(CLASS_ANNOTATION_ENTRY))
                .declarations(&aligned_box_declarations)
                .declaration(CssProperty::BackgroundColor, bg_color_annotation_entry),
        )
        .group(
            CssGroup::media_print()
                .ruleset(
                    CssRuleset::new(decision_table_sub_class(CLASS_GRID_BODY))
                        .declaration(CssProperty::Border, border_1px)
                        .declaration(CssProperty::GridGap, px!(1)),
                )
                .ruleset(
                    CssRuleset::new(decision_table_sub_class(CLASS_HORIZONTAL_DOUBLE_LINE))
                        .declaration(CssProperty::Height, px!(1)),
                )
                .ruleset(
                    CssRuleset::new(decision_table_sub_class(CLASS_INFORMATION_ITEM_NAME))
                        .declaration(CssProperty::Border, border_1px)
                        .declaration(CssProperty::Top, px!(1)),
                )
                .ruleset(
                    CssRuleset::new(decision_table_sub_class(CLASS_VERTICAL_DOUBLE_LINE))
                        .declaration(CssProperty::Width, px!(1)),
                ),
        )
}
