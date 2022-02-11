use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsExportNamedShorthandSpecifier;
use rslint_parser::ast::JsExportNamedShorthandSpecifierSlots;

impl ToFormatElement for JsExportNamedShorthandSpecifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExportNamedShorthandSpecifierSlots { type_token, name } = self.as_slots();

        let type_token = type_token.format_with_or_empty(formatter, |type_token| {
            format_elements![type_token, space_token()]
        })?;
        let name = name.format(formatter)?;

        Ok(format_elements![type_token, name])
    }
}
