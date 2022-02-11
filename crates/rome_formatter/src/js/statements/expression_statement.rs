use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{format_elements, token, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsExpressionStatement;
use rslint_parser::ast::JsExpressionStatementSlots;

impl ToFormatElement for JsExpressionStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExpressionStatementSlots {
            expression,
            semicolon_token,
        } = self.as_slots();

        Ok(format_elements![
            expression.format(formatter)?,
            semicolon_token.format_or(formatter, || token(";"))?
        ])
    }
}
