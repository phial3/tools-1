use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsPrivateName;
use rslint_parser::ast::JsPrivateNameSlots;

impl ToFormatElement for JsPrivateName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsPrivateNameSlots {
            hash_token,
            value_token,
        } = self.as_slots();

        Ok(format_elements![
            hash_token.format(formatter)?,
            value_token.format(formatter)?
        ])
    }
}
