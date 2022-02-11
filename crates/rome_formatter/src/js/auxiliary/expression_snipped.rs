use crate::{
    format_elements, formatter_traits::FormatTokenAndNode, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::ast::JsExpressionSnipped;
use rslint_parser::ast::JsExpressionSnippedSlots;

impl ToFormatElement for JsExpressionSnipped {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExpressionSnippedSlots {
            expression,
            eof_token,
        } = self.as_slots();

        Ok(format_elements![
            expression.format(formatter)?,
            eof_token.format(formatter)?,
        ])
    }
}
