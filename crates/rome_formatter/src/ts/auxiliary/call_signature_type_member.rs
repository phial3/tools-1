use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::utils::format_type_member_separator;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsCallSignatureTypeMember;
use rslint_parser::ast::TsCallSignatureTypeMemberSlots;

impl ToFormatElement for TsCallSignatureTypeMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsCallSignatureTypeMemberSlots {
            type_parameters,
            parameters,
            return_type_annotation,
            separator_token,
        } = self.as_slots();

        let type_parameters = type_parameters.format_or_empty(formatter)?;
        let parameters = parameters.format(formatter)?;
        let return_type_annotation = return_type_annotation.format_or_empty(formatter)?;
        let separator = format_type_member_separator(separator_token, formatter)?;

        Ok(format_elements![
            type_parameters,
            parameters,
            return_type_annotation,
            separator
        ])
    }
}
