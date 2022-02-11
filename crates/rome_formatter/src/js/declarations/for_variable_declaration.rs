use rslint_parser::ast::JsForVariableDeclaration;

use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsForVariableDeclarationSlots;

impl ToFormatElement for JsForVariableDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsForVariableDeclarationSlots {
            kind_token,
            declarator,
        } = self.as_slots();

        Ok(format_elements![
            kind_token.format(formatter)?,
            space_token(),
            declarator.format(formatter)?,
        ])
    }
}
