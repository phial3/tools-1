use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsPrivateClassMemberName;
use rslint_parser::ast::JsPrivateClassMemberNameSlots;

impl ToFormatElement for JsPrivateClassMemberName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsPrivateClassMemberNameSlots {
            hash_token,
            id_token,
        } = self.as_slots();

        Ok(format_elements![
            hash_token.format(formatter)?,
            id_token.format(formatter)?,
        ])
    }
}
