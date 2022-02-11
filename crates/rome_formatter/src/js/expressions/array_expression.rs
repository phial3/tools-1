use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, group_elements, soft_block_indent, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};

use rslint_parser::ast::JsArrayExpression;
use rslint_parser::ast::JsArrayExpressionSlots;

impl ToFormatElement for JsArrayExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsArrayExpressionSlots {
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
                    close_token_leading,
                ]))
            },
            &r_brack_token?,
        )?))
    }
}
