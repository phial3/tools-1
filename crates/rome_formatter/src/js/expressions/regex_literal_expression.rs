use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsRegexLiteralExpression;
use rslint_parser::ast::JsRegexLiteralExpressionSlots;

impl ToFormatElement for JsRegexLiteralExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsRegexLiteralExpressionSlots { value_token } = self.as_slots();

        value_token.format(formatter)
    }
}
