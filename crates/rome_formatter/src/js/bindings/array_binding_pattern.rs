use crate::{
    format_elements, formatter_traits::FormatTokenAndNode, group_elements, soft_block_indent,
    FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsArrayBindingPattern;
use rslint_parser::ast::JsArrayBindingPatternSlots;

impl ToFormatElement for JsArrayBindingPattern {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsArrayBindingPatternSlots {
            l_brack_token,
            elements,
            r_brack_token,
        } = self.as_slots();

        Ok(group_elements(formatter.format_delimited(
            &l_brack_token?,
            |open_token_trailing, close_token_leading| {
                Ok(soft_block_indent(format_elements![
                    open_token_trailing,
                    elements.format(formatter)?,
                    close_token_leading
                ]))
            },
            &r_brack_token?,
        )?))
    }
}
