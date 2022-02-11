use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::utils::format_type_member_separator;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::TsGetterSignatureTypeMember;
use rslint_parser::ast::TsGetterSignatureTypeMemberSlots;

impl ToFormatElement for TsGetterSignatureTypeMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsGetterSignatureTypeMemberSlots {
            get_token,
            name,
            l_paren_token,
            r_paren_token,
            type_annotation,
            separator_token,
        } = self.as_slots();

        let get = get_token.format(formatter)?;
        let name = name.format(formatter)?;
        let l_paren = l_paren_token.format(formatter)?;
        let r_paren = r_paren_token.format(formatter)?;
        let type_annotation = type_annotation.format_or_empty(formatter)?;
        let separator = format_type_member_separator(separator_token, formatter)?;

        Ok(format_elements![
            get,
            space_token(),
            name,
            l_paren,
            r_paren,
            type_annotation,
            separator
        ])
    }
}
