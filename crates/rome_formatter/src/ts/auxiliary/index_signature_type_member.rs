use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::utils::format_type_member_separator;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::TsIndexSignatureTypeMember;
use rslint_parser::ast::TsIndexSignatureTypeMemberSlots;

impl ToFormatElement for TsIndexSignatureTypeMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsIndexSignatureTypeMemberSlots {
            readonly_token,
            l_brack_token,
            parameter,
            r_brack_token,
            type_annotation,
            separator_token,
        } = self.as_slots();

        let readonly = readonly_token.format_with_or_empty(formatter, |readonly_token| {
            format_elements![readonly_token, space_token()]
        })?;

        let l_bracket = l_brack_token.format(formatter)?;
        let parameter = parameter.format(formatter)?;
        let r_bracket = r_brack_token.format(formatter)?;

        let type_annotation = type_annotation.format(formatter)?;
        let separator = format_type_member_separator(separator_token, formatter)?;

        Ok(format_elements![
            readonly,
            l_bracket,
            parameter,
            r_bracket,
            type_annotation,
            separator,
        ])
    }
}
