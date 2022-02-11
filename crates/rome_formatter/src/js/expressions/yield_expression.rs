use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsYieldExpression;
use rslint_parser::ast::JsYieldExpressionSlots;

impl ToFormatElement for JsYieldExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsYieldExpressionSlots {
            yield_token,
            argument,
        } = self.as_slots();

        let argument = argument.format_or_empty(formatter)?;

        Ok(format_elements![yield_token.format(formatter)?, argument])
    }
}
