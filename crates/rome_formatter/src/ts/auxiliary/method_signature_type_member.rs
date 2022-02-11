use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::utils::format_type_member_separator;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsMethodSignatureTypeMember;
use rslint_parser::ast::TsMethodSignatureTypeMemberSlots;

impl ToFormatElement for TsMethodSignatureTypeMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsMethodSignatureTypeMemberSlots {
            name,
            optional_token,
            type_parameters,
            parameters,
            return_type_annotation,
            separator_token,
        } = self.as_slots();

        let name = name.format(formatter)?;
        let optional_token = optional_token.format_or_empty(formatter)?;
        let type_arguments = type_parameters.format_or_empty(formatter)?;
        let parameters = parameters.format(formatter)?;
        let return_type_annotation = return_type_annotation.format_or_empty(formatter)?;
        let separator = format_type_member_separator(separator_token, formatter)?;
        Ok(format_elements![
            name,
            optional_token,
            type_arguments,
            parameters,
            return_type_annotation,
            separator
        ])
    }
}
