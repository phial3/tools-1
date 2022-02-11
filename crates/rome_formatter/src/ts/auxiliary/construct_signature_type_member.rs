use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsConstructSignatureTypeMember;
use rslint_parser::ast::TsConstructSignatureTypeMemberSlots;

impl ToFormatElement for TsConstructSignatureTypeMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsConstructSignatureTypeMemberSlots {
            new_token,
            type_parameters,
            parameters,
            type_annotation,
            separator_token,
        } = self.as_slots();

        let new = new_token.format(formatter)?;
        let type_parameters = type_parameters.format_or_empty(formatter)?;
        let parameters = parameters.format(formatter)?;
        let type_annotation = type_annotation.format_or_empty(formatter)?;

        Ok(format_elements![
            new,
            type_parameters,
            parameters,
            type_annotation
        ])
    }
}
