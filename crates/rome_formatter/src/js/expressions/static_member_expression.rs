use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, group_elements, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsStaticMemberExpression;
use rslint_parser::ast::JsStaticMemberExpressionSlots;

impl ToFormatElement for JsStaticMemberExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsStaticMemberExpressionSlots {
            object,
            operator,
            member,
        } = self.as_slots();

        Ok(group_elements(format_elements![
            object.format(formatter)?,
            operator.format(formatter)?,
            member.format(formatter)?,
        ]))
    }
}
