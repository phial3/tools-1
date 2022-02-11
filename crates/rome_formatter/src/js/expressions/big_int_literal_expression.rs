use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsBigIntLiteralExpression;
use rslint_parser::ast::JsBigIntLiteralExpressionSlots;

impl ToFormatElement for JsBigIntLiteralExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsBigIntLiteralExpressionSlots { value_token } = self.as_slots();

        value_token.format(formatter)
    }
}
