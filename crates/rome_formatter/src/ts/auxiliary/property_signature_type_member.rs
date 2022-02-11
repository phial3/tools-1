use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::utils::format_type_member_separator;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::TsPropertySignatureTypeMember;
use rslint_parser::ast::TsPropertySignatureTypeMemberSlots;

impl ToFormatElement for TsPropertySignatureTypeMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsPropertySignatureTypeMemberSlots {
            readonly_token,
            name,
            optional_token,
            type_annotation,
            separator_token,
        } = self.as_slots();

        let readonly = readonly_token.format_or_empty(formatter)?;
        let name = name.format(formatter)?;
        let optional = optional_token.format_or_empty(formatter)?;
        let type_annotation = type_annotation.format_or_empty(formatter)?;
        let separator = format_type_member_separator(separator_token, formatter)?;

        Ok(format_elements![
            readonly,
            space_token(),
            name,
            optional,
            type_annotation,
            separator
        ])
    }
}
