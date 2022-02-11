use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsBinaryExpression;
use rslint_parser::ast::JsBinaryExpressionSlots;

impl ToFormatElement for JsBinaryExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsBinaryExpressionSlots {
            left,
            operator,
            right,
        } = self.as_slots();

        Ok(format_elements![
            left.format(formatter)?,
            space_token(),
            operator.format(formatter)?,
            space_token(),
            right.format(formatter)?,
        ])
    }
}
