use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsIdentifierAssignment;
use rslint_parser::ast::JsIdentifierAssignmentSlots;

impl ToFormatElement for JsIdentifierAssignment {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsIdentifierAssignmentSlots { name_token } = self.as_slots();

        name_token.format(formatter)
    }
}
