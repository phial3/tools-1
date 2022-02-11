use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsIdentifierBinding;
use rslint_parser::ast::JsIdentifierBindingSlots;

impl ToFormatElement for JsIdentifierBinding {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsIdentifierBindingSlots { name_token } = self.as_slots();

        name_token.format(formatter)
    }
}
