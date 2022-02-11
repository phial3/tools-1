use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsContinueStatement;
use rslint_parser::ast::JsContinueStatementSlots;

impl ToFormatElement for JsContinueStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsContinueStatementSlots {
            continue_token,
            label_token,
            semicolon_token,
        } = self.as_slots();

        let label = label_token
            .format_with_or_empty(formatter, |token| format_elements![space_token(), token])?;

        let semicolon = semicolon_token.format_or(formatter, || token(";"))?;

        Ok(format_elements![
            continue_token.format(formatter)?,
            label,
            semicolon
        ])
    }
}
