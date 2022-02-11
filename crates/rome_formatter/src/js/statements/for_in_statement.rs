use rslint_parser::ast::JsForInStatement;

use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, group_elements, soft_block_indent, soft_line_break_or_space, space_token,
    FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsForInStatementSlots;

impl ToFormatElement for JsForInStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsForInStatementSlots {
            for_token,
            l_paren_token,
            initializer,
            in_token,
            expression,
            r_paren_token,
            body,
        } = self.as_slots();

        let for_token = for_token.format(formatter)?;
        let initializer = initializer.format(formatter)?;
        let in_token = in_token.format(formatter)?;
        let expression = expression.format(formatter)?;
        let body = body.format(formatter)?;

        Ok(format_elements![
            for_token,
            space_token(),
            formatter.format_delimited(
                &l_paren_token?,
                |open_token_trailing, close_token_leading| Ok(group_elements(soft_block_indent(
                    format_elements![
                        open_token_trailing,
                        initializer,
                        soft_line_break_or_space(),
                        in_token,
                        soft_line_break_or_space(),
                        expression,
                        close_token_leading,
                    ]
                ))),
                &r_paren_token?
            )?,
            space_token(),
            body
        ])
    }
}
