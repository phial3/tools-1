use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsReferenceType;
use rslint_parser::ast::TsReferenceTypeSlots;

impl ToFormatElement for TsReferenceType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsReferenceTypeSlots {
            name,
            type_arguments,
        } = self.as_slots();

        let name = name.format(formatter)?;
        let type_arguments = type_arguments.format_or_empty(formatter)?;
        Ok(format_elements![name, type_arguments])
    }
}
