use crate::formatter_traits::FormatTokenAndNode;
use crate::utils::format_initializer_clause;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsVariableDeclarator;
use rslint_parser::ast::JsVariableDeclaratorSlots;

impl ToFormatElement for JsVariableDeclarator {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsVariableDeclaratorSlots {
            id,
            variable_annotation,
            initializer,
        } = self.as_slots();

        let initializer = format_initializer_clause(formatter, initializer)?;

        Ok(format_elements![id.format(formatter)?, initializer])
    }
}
