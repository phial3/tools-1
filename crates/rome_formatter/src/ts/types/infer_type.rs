use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::TsInferType;
use rslint_parser::ast::TsInferTypeSlots;

impl ToFormatElement for TsInferType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsInferTypeSlots {
            infer_token,
            type_parameter,
        } = self.as_slots();

        let infer = infer_token.format(formatter)?;
        let type_parameter = type_parameter.format(formatter)?;
        Ok(format_elements![infer, space_token(), type_parameter])
    }
}
