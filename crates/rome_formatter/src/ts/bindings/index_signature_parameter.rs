use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsIndexSignatureParameter;
use rslint_parser::ast::TsIndexSignatureParameterSlots;

impl ToFormatElement for TsIndexSignatureParameter {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsIndexSignatureParameterSlots {
            binding,
            type_annotation,
        } = self.as_slots();

        let binding = binding.format(formatter)?;
        let type_annotation = type_annotation.format(formatter)?;

        Ok(format_elements![binding, type_annotation])
    }
}
