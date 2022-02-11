use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsAwaitExpression;
use rslint_parser::ast::JsAwaitExpressionSlots;

impl ToFormatElement for JsAwaitExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsAwaitExpressionSlots {
            await_token,
            argument,
        } = self.as_slots();

        Ok(format_elements![
            await_token.format(formatter)?,
            space_token(),
            argument.format(formatter)?,
        ])
    }
}
