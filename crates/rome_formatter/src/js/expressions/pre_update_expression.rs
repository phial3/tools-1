use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsPreUpdateExpression;
use rslint_parser::ast::JsPreUpdateExpressionSlots;

impl ToFormatElement for JsPreUpdateExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsPreUpdateExpressionSlots { operator, operand } = self.as_slots();

        Ok(format_elements![
            operator.format(formatter)?,
            operand.format(formatter)?,
        ])
    }
}
