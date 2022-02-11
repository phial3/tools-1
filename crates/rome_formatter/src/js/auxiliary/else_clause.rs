use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsElseClause;
use rslint_parser::ast::JsElseClauseSlots;

impl ToFormatElement for JsElseClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsElseClauseSlots {
            else_token,
            alternate,
        } = self.as_slots();

        Ok(format_elements![
            else_token.format(formatter)?,
            space_token(),
            alternate.format(formatter)?,
        ])
    }
}
