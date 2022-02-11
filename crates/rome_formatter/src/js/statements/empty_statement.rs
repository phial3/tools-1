use crate::{empty_element, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsEmptyStatement;
use rslint_parser::ast::JsEmptyStatementSlots;

impl ToFormatElement for JsEmptyStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsEmptyStatementSlots { semicolon_token } = self.as_slots();

        formatter.format_replaced(&semicolon_token?, empty_element())
    }
}
