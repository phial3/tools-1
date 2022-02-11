use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsBooleanLiteralExpression;
use rslint_parser::ast::JsBooleanLiteralExpressionSlots;

impl ToFormatElement for JsBooleanLiteralExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsBooleanLiteralExpressionSlots { value_token } = self.as_slots();

        value_token.format(formatter)
    }
}
