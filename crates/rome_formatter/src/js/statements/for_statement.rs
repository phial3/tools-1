use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, group_elements, soft_block_indent, soft_line_break_or_space, space_token,
    FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsForStatement;
use rslint_parser::ast::JsForStatementSlots;

impl ToFormatElement for JsForStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsForStatementSlots {
            for_token,
            l_paren_token,
            initializer,
            first_semi_token,
            test,
            second_semi_token,
            update,
            r_paren_token,
            body,
        } = self.as_slots();

        let inner = if initializer.is_some() || test.is_some() || update.is_some() {
            format_elements![
                initializer.format_or_empty(formatter)?,
                first_semi_token.format(formatter)?,
                soft_line_break_or_space(),
                test.format_or_empty(formatter)?,
                second_semi_token.format(formatter)?,
                soft_line_break_or_space(),
                update.format_or_empty(formatter)?,
            ]
        } else {
            format_elements![
                first_semi_token.format(formatter)?,
                second_semi_token.format(formatter)?,
            ]
        };

        Ok(group_elements(format_elements![
            for_token.format(formatter)?,
            space_token(),
            formatter.format_delimited(
                &l_paren_token?,
                |open_token_trailing, close_token_leading| Ok(group_elements(soft_block_indent(
                    format_elements![open_token_trailing, inner, close_token_leading]
                ))),
                &r_paren_token?,
            )?,
            space_token(),
            body.format(formatter)?
        ]))
    }
}
