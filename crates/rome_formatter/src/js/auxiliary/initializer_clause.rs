use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsInitializerClause;
use rslint_parser::ast::JsInitializerClauseSlots;

impl ToFormatElement for JsInitializerClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsInitializerClauseSlots {
            eq_token,
            expression,
        } = self.as_slots();

        Ok(format_elements![
            eq_token.format(formatter)?,
            space_token(),
            expression.format(formatter)?
        ])
    }
}
