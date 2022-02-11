use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsInExpression;
use rslint_parser::ast::JsInExpressionSlots;

impl ToFormatElement for JsInExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsInExpressionSlots {
            property,
            in_token,
            object,
        } = self.as_slots();

        Ok(format_elements![
            property.format(formatter)?,
            space_token(),
            in_token.format(formatter)?,
            space_token(),
            object.format(formatter)?,
        ])
    }
}
