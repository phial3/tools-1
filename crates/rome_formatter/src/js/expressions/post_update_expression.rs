use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsPostUpdateExpression;
use rslint_parser::ast::JsPostUpdateExpressionSlots;

impl ToFormatElement for JsPostUpdateExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsPostUpdateExpressionSlots { operand, operator } = self.as_slots();

        Ok(format_elements![
            operand.format(formatter)?,
            operator.format(formatter)?,
        ])
    }
}
