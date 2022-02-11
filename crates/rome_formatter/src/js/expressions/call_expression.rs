use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsCallExpression;
use rslint_parser::ast::JsCallExpressionSlots;

impl ToFormatElement for JsCallExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsCallExpressionSlots {
            callee,
            optional_chain_token_token,
            type_arguments,
            arguments,
        } = self.as_slots();

        let name = callee.format(formatter)?;
        let option = optional_chain_token_token.format_or_empty(formatter)?;
        let type_arguments = type_arguments.format_or_empty(formatter)?;
        let arguments = arguments.format(formatter)?;

        Ok(format_elements![name, option, type_arguments, arguments])
    }
}
