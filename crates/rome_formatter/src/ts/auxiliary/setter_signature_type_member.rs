use crate::formatter_traits::FormatTokenAndNode;
use crate::utils::format_type_member_separator;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::TsSetterSignatureTypeMember;
use rslint_parser::ast::TsSetterSignatureTypeMemberSlots;

impl ToFormatElement for TsSetterSignatureTypeMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsSetterSignatureTypeMemberSlots {
            set_token,
            name,
            l_paren_token,
            parameter,
            r_paren_token,
            separator_token,
        } = self.as_slots();

        let set = set_token.format(formatter)?;
        let name = name.format(formatter)?;
        let l_paren = l_paren_token.format(formatter)?;
        let parameter = parameter.format(formatter)?;
        let r_paren = r_paren_token.format(formatter)?;
        let separator = format_type_member_separator(separator_token, formatter)?;

        Ok(format_elements![
            set,
            space_token(),
            name,
            l_paren,
            parameter,
            r_paren,
            separator
        ])
    }
}
