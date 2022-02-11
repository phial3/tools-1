use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsNumberLiteralExpression;
use rslint_parser::ast::JsNumberLiteralExpressionSlots;

impl ToFormatElement for JsNumberLiteralExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsNumberLiteralExpressionSlots { value_token } = self.as_slots();

        value_token.format(formatter)
    }
}
