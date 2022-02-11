use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, group_elements, soft_block_indent, space_token, FormatElement, FormatResult,
    Formatter, ToFormatElement,
};

use rslint_parser::ast::JsWhileStatement;
use rslint_parser::ast::JsWhileStatementSlots;

impl ToFormatElement for JsWhileStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsWhileStatementSlots {
            while_token,
            l_paren_token,
            test,
            r_paren_token,
            body,
        } = self.as_slots();

        Ok(format_elements![
            while_token.format(formatter)?,
            space_token(),
            group_elements(formatter.format_delimited(
                &l_paren_token?,
                |open_token_trailing, close_token_leading| Ok(soft_block_indent(format_elements![
                    open_token_trailing,
                    test.format(formatter)?,
                    close_token_leading,
                ])),
                &r_paren_token?,
            )?),
            space_token(),
            body.format(formatter)?
        ])
    }
}
