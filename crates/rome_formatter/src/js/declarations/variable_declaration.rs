use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsVariableDeclaration;
use rslint_parser::ast::JsVariableDeclarationSlots;

impl ToFormatElement for JsVariableDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsVariableDeclarationSlots { kind, declarators } = self.as_slots();

        Ok(format_elements![
            kind.format(formatter)?,
            space_token(),
            declarators.format(formatter)?,
        ])
    }
}
