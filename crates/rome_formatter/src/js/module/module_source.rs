use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsModuleSource;
use rslint_parser::ast::JsModuleSourceSlots;

impl ToFormatElement for JsModuleSource {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsModuleSourceSlots { value_token } = self.as_slots();

        value_token.format(formatter)
    }
}
