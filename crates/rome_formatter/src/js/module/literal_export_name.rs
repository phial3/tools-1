use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsLiteralExportName;
use rslint_parser::ast::JsLiteralExportNameSlots;

impl ToFormatElement for JsLiteralExportName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsLiteralExportNameSlots { value } = self.as_slots();

        value.format(formatter)
    }
}
