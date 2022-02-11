use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsPropertyObjectMember;
use rslint_parser::ast::JsPropertyObjectMemberSlots;

impl ToFormatElement for JsPropertyObjectMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsPropertyObjectMemberSlots {
            name,
            colon_token,
            value,
        } = self.as_slots();

        let key = name.format(formatter)?;
        let colon = colon_token.format(formatter)?;
        let value = value.format(formatter)?;
        Ok(format_elements![key, colon, space_token(), value])
    }
}
