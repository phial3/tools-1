use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsName;
use rslint_parser::ast::JsNameSlots;

impl ToFormatElement for JsName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsNameSlots { value_token } = self.as_slots();

        value_token.format(formatter)
    }
}
