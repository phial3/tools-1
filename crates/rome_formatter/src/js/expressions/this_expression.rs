use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsThisExpression;
use rslint_parser::ast::JsThisExpressionSlots;

impl ToFormatElement for JsThisExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsThisExpressionSlots { this_token } = self.as_slots();

        this_token.format(formatter)
    }
}
