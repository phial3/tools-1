use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsIdentifierExpression;
use rslint_parser::ast::JsIdentifierExpressionSlots;

impl ToFormatElement for JsIdentifierExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsIdentifierExpressionSlots { name } = self.as_slots();

        name.format(formatter)
    }
}
