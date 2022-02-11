use crate::{
    format_elements, formatter_traits::FormatTokenAndNode, group_elements, soft_block_indent,
    space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsObjectAssignmentPattern;
use rslint_parser::ast::JsObjectAssignmentPatternSlots;

impl ToFormatElement for JsObjectAssignmentPattern {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsObjectAssignmentPatternSlots {
            l_curly_token,
            properties,
            r_curly_token,
        } = self.as_slots();

        Ok(group_elements(formatter.format_delimited(
            &l_curly_token?,
            |open_token_trailing, close_token_leading| {
                Ok(format_elements![
                    space_token(),
                    soft_block_indent(format_elements![
                        open_token_trailing,
                        properties.format(formatter)?,
                        close_token_leading
                    ]),
                    space_token(),
                ])
            },
            &r_curly_token?,
        )?))
    }
}
