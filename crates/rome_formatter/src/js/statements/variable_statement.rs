use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsVariableStatement;
use rslint_parser::ast::JsVariableStatementSlots;

impl ToFormatElement for JsVariableStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsVariableStatementSlots {
            declare_token,
            declaration,
            semicolon_token,
        } = self.as_slots();

        Ok(format_elements![
            declare_token
                .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?,
            declaration.format(formatter)?,
            semicolon_token.format_or(formatter, || token(";"))?,
        ])
    }
}
