use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::TsTypeAnnotation;
use rslint_parser::ast::TsTypeAnnotationSlots;

impl ToFormatElement for TsTypeAnnotation {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsTypeAnnotationSlots { colon_token, ty } = self.as_slots();

        let colon = colon_token.format(formatter)?;
        let ty = ty.format(formatter)?;

        Ok(format_elements![colon, space_token(), ty])
    }
}
