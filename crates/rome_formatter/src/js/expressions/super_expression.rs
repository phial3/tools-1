use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsSuperExpression;
use rslint_parser::ast::JsSuperExpressionSlots;

impl ToFormatElement for JsSuperExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsSuperExpressionSlots { super_token } = self.as_slots();

        super_token.format(formatter)
    }
}
