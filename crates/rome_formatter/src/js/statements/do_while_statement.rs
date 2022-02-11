use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, group_elements, soft_block_indent, space_token, token, FormatElement,
    FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsDoWhileStatement;
use rslint_parser::ast::JsDoWhileStatementSlots;

impl ToFormatElement for JsDoWhileStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsDoWhileStatementSlots {
            do_token,
            body,
            while_token,
            l_paren_token,
            test,
            r_paren_token,
            semicolon_token,
        } = self.as_slots();

        Ok(format_elements![
            do_token.format(formatter)?,
            space_token(),
            body.format(formatter)?,
            space_token(),
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
            semicolon_token.format_or(formatter, || token(";"))?
        ])
    }
}
