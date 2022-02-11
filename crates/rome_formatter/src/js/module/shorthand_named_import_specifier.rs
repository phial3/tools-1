use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsShorthandNamedImportSpecifier;
use rslint_parser::ast::JsShorthandNamedImportSpecifierSlots;

impl ToFormatElement for JsShorthandNamedImportSpecifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsShorthandNamedImportSpecifierSlots { local_name } = self.as_slots();

        local_name.format(formatter)
    }
}
