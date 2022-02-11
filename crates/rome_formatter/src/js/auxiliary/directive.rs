use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{format_elements, token, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsDirective;
use rslint_parser::ast::JsDirectiveSlots;

impl ToFormatElement for JsDirective {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsDirectiveSlots {
            value_token,
            semicolon_token,
        } = self.as_slots();

        Ok(format_elements![
            value_token.format(formatter)?,
            semicolon_token.format_or(formatter, || token(";"))?,
        ])
    }
}
