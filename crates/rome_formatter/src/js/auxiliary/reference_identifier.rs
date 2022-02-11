use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsReferenceIdentifier;
use rslint_parser::ast::JsReferenceIdentifierSlots;

impl ToFormatElement for JsReferenceIdentifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsReferenceIdentifierSlots { value_token } = self.as_slots();

        value_token.format(formatter)
    }
}
