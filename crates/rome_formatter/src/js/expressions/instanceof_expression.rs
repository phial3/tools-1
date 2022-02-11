use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsInstanceofExpression;
use rslint_parser::ast::JsInstanceofExpressionSlots;

impl ToFormatElement for JsInstanceofExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsInstanceofExpressionSlots {
            left,
            instanceof_token,
            right,
        } = self.as_slots();

        Ok(format_elements![
            left.format(formatter)?,
            space_token(),
            instanceof_token.format(formatter)?,
            space_token(),
            right.format(formatter)?,
        ])
    }
}
