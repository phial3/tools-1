use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsShorthandPropertyObjectMember;
use rslint_parser::ast::JsShorthandPropertyObjectMemberSlots;

impl ToFormatElement for JsShorthandPropertyObjectMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsShorthandPropertyObjectMemberSlots { name } = self.as_slots();

        name.format(formatter)
    }
}
