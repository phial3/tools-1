use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsGetterClassMember;
use rslint_parser::ast::JsGetterClassMemberSlots;

impl ToFormatElement for JsGetterClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsGetterClassMemberSlots {
            access_modifier,
            static_token,
            abstract_token,
            get_token,
            name,
            l_paren_token,
            r_paren_token,
            return_type,
            body,
        } = self.as_slots();

        Ok(format_elements![
            access_modifier
                .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?,
            static_token
                .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?,
            abstract_token
                .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?,
            get_token.format(formatter)?,
            space_token(),
            name.format(formatter)?,
            l_paren_token.format(formatter)?,
            r_paren_token.format(formatter)?,
            return_type.format_or_empty(formatter)?,
            space_token(),
            body.format(formatter)?
        ])
    }
}
