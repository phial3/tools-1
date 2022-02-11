use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsNullLiteralExpression;
use rslint_parser::ast::JsNullLiteralExpressionSlots;

impl ToFormatElement for JsNullLiteralExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsNullLiteralExpressionSlots { value_token } = self.as_slots();

        value_token.format(formatter)
    }
}
